#![deny(clippy::pedantic)]

use chrono::NaiveDate;
use chrono::Utc;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use std::sync::Arc;
use tokio::task;
use tokio::time::{interval, Duration};

use std::env;

extern crate dotenv;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
use dotenv::dotenv;

mod bot;

use bot::commands::Handler;
use bot::commands::COMMANDS_GROUP;
use bot::BOT_HELP;

mod database;
mod models;
mod schema;
mod test;
mod utils;

use models::Birthday;

use crate::database::DB;

embed_migrations!();

#[tokio::main]
async fn main() {
    dotenv().ok();
    embedded_migrations::run(get_postgres_db_lock!()).unwrap();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .help(&BOT_HELP)
        .group(&COMMANDS_GROUP);

    let client = serenity::client::ClientBuilder::new(env::var("DISCORD_TOKEN").unwrap())
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let mut arc = Arc::new(client);
    poll_bdays(arc.clone()).await;
    if let Err(why) = Arc::get_mut(&mut arc).unwrap().start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn poll_bdays(client: Arc<Client>) {
    let mut interval = interval(Duration::from_secs(30));
    let cache_and_http = Arc::clone(&(client).cache_and_http);

    task::spawn(async move {
        loop {
            let http = (cache_and_http).http.clone();
            let today_naive = Utc::today().naive_local();
            let mut bdays_today = database::statements::get_bdays_today(today_naive);
            bday_process_vec_and_update(http.clone(), today_naive, &mut bdays_today).await;

            let mut bdays_all_except = database::statements::get_allexceptdata_bdays();
            bday_process_vec_and_update(http.clone(), today_naive, &mut bdays_all_except).await;
            interval.tick().await;
        }
    });
}

async fn bday_process_vec_and_update(
    http: Arc<Http>,
    today_naive: NaiveDate,
    bdays: &mut [Birthday],
) {
    for i in bdays.iter_mut() {
        if let Some(_) = send_bday(http.clone(), i, today_naive).await {}
    }
    for i in bdays.iter() {
        database::statements::update_bday_last_updated(&i.userid, i.lastdate);
    }
}

async fn send_bday(
    http: Arc<Http>,
    bday: &mut Birthday,
    today_naive: NaiveDate,
) -> Option<&mut Birthday> {
    let already_processed = bday.lastdate >= today_naive;
    let allexceptdate_not_satisfied =
        bday.allexceptdate && bday.date == utils::date_as_year_zero(Utc::today().naive_utc());
    if already_processed || allexceptdate_not_satisfied {
        return None;
    }
    bday.lastdate = today_naive;
    let channel_id = ChannelId::from(bday.channelid.parse::<u64>().unwrap());
    match &bday.notifyrole {
        None => channel_id
            .say(
                http,
                format!("Happy Birthday <@{}> @everyone!", bday.userid),
            )
            .await
            .unwrap(),
        Some(x) => channel_id
            .say(
                http,
                format!("Happy Birthday <@{}> <@&{}>!", bday.userid, x),
            )
            .await
            .unwrap(),
    };
    Some(bday)
}
