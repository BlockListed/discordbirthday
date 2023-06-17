#![deny(clippy::pedantic)]
#![allow(clippy::similar_names)]

use chrono::NaiveDate;
use chrono::Utc;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::id::ChannelId;
use serenity::prelude::GatewayIntents;
use std::sync::Arc;
use tokio::task;
use tokio::time::{interval, Duration};

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use std::env;

extern crate dotenv;
#[macro_use]
extern crate diesel;
use dotenv::dotenv;

mod bot;

use bot::commands::Handler;
use bot::commands::COMMANDS_GROUP;
use bot::BOT_HELP;

mod database;
mod models;
mod schema;
mod utils;

use models::Birthday;

use crate::database::get_db;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() {
    dotenv().ok();
    match &mut *get_db().await.lock().await {
        database::DbType::Postgres(conn) => {
            conn.synchronous.run_pending_migrations(MIGRATIONS).unwrap();
        }
        database::DbType::Sqlite(conn) => {
            conn.run_pending_migrations(MIGRATIONS).unwrap();
        }
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(";"))
        .help(&BOT_HELP)
        .group(&COMMANDS_GROUP);

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let client = serenity::client::ClientBuilder::new(env::var("DISCORD_TOKEN").unwrap(), intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let mut arc = Arc::new(client);
    poll_bdays(arc.clone()).await;
    if let Err(why) = Arc::get_mut(&mut arc).unwrap().start().await {
        println!("An error occurred while running the client: {why:?}");
    }
}

async fn poll_bdays(client: Arc<Client>) {
    let mut interval = interval(Duration::from_secs(30));
    let cache_and_http = Arc::clone(&(client).cache_and_http);

    task::spawn(async move {
        loop {
            let http = (cache_and_http).http.clone();
            let today_naive = Utc::now().date_naive();
            let mut bdays_today = database::statements::get_bdays_today(today_naive).await;
            bday_process_vec_and_update(http.clone(), today_naive, &mut bdays_today).await;

            let mut bdays_all_except = database::statements::get_allexceptdate_bdays().await;
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
        if send_bday(http.clone(), i, today_naive).await.is_some() {}
    }
    for i in bdays.iter() {
        database::statements::update_bday_last_updated(&i.userid, i.lastdate).await;
    }
}

async fn send_bday(
    http: Arc<Http>,
    bday: &mut Birthday,
    today_naive: NaiveDate,
) -> Option<&mut Birthday> {
    let already_processed = bday.lastdate >= today_naive;
    let allexceptdate_not_satisfied =
        bday.allexceptdate && bday.date == utils::date_as_year_zero(Utc::now().date_naive());
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
