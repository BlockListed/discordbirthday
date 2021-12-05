use std::ops::DerefMut;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tokio::task;
use chrono::NaiveDate;
use chrono::Utc;
use serenity::framework::StandardFramework;
use serenity::client::Client;
use serenity::model::id::ChannelId;
use serenity::http::Http;

use std::sync::Mutex;
use std::env;

#[macro_use]
extern crate lazy_static;
extern crate dotenv;
#[macro_use]
extern crate diesel;
use diesel::{
    SqliteConnection,
    Connection,
    prelude::*
};
#[macro_use]
extern crate diesel_migrations;
use dotenv::dotenv;

mod bot;

use bot::commands::COMMANDS_GROUP;
use bot::BOT_HELP;
use bot::commands::Handler;

mod models;
mod schema;
mod utils;

use models::Birthday;

lazy_static! {
    static ref DB: Mutex<SqliteConnection> = Mutex::new(establish_connection());
}

embed_migrations!();

#[tokio::main]
async fn main() {
    dotenv().ok();
    embedded_migrations::run(DB.lock().unwrap().deref_mut()).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .help(&BOT_HELP)
        .group(&COMMANDS_GROUP);

    let client = serenity::client::ClientBuilder::new(env::var("DISCORD_TOKEN").unwrap())
        .event_handler(Handler)
        .framework(framework)
        .await.expect("Error creating client");
    
    let mut arc = Arc::new(client);
    poll_bdays(arc.clone()).await;
    if let Err(why) = Arc::get_mut(&mut arc).unwrap().start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn poll_bdays(client: Arc<Client>) {
    use schema::birthdays::dsl::*;
    let mut interval = interval(Duration::from_secs(30));
    let cache_and_http = Arc::clone(&(client).cache_and_http);

    task::spawn(async move {
        loop {
            let http = (cache_and_http).http.clone();
            let today_naive = Utc::today().naive_local();
            let mut bdays_today = birthdays.filter(date.eq(utils::date_as_year_zero(today_naive))).filter(allexceptdate.eq(false))
            .load::<models::Birthday>(DB.lock().unwrap().deref_mut()).unwrap();
            bday_process_vec_and_update(http.clone(), today_naive, &mut bdays_today).await;

            let mut bdays_all_except = birthdays.filter(allexceptdate.eq(true))
            .load::<models::Birthday>(DB.lock().unwrap().deref_mut()).unwrap();
            bday_process_vec_and_update(http.clone(), today_naive, &mut bdays_all_except).await;
            interval.tick().await;
        }
    });
}

async fn bday_process_vec_and_update(http: Arc<Http>, today_naive: NaiveDate, bdays: &mut Vec<Birthday>) {
    use schema::birthdays::dsl::*;

    for i in bdays.iter_mut() {
        let j = i.clone();
        *i = match send_bday(http.clone(), i, today_naive).await {
            Some(data) => data.clone(),
            None => j
        }
    }
    for i in bdays.iter() {
        let i_userid = i.userid.clone();
        diesel::update(birthdays.filter(userid.eq(i_userid)))
            .set(lastdate.eq(i.lastdate))
            .execute(DB.lock().unwrap().deref_mut()).unwrap();
    }
}

async fn send_bday(http: Arc<Http>, bday: &mut Birthday, today_naive: NaiveDate) -> Option<&mut Birthday>  {
    if bday.lastdate >= today_naive {
        return None;
    } else {
        bday.lastdate = today_naive;
        let channel_id = ChannelId::from(bday.channelid.parse::<u64>().unwrap());
        match &bday.notifyrole {
            None => channel_id.say(http, format!("Happy Birthday <@{}> @everyone!", bday.userid)).await.unwrap(),
            Some(x) => channel_id.say(http, format!("Happy Birthday <@{}> <@&{}>!", bday.userid, x)).await.unwrap()
        };
    } 
    Some(bday)
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let file_name = env::var("DATABASE_URL").unwrap();
    SqliteConnection::establish(file_name.as_str()).unwrap()
}