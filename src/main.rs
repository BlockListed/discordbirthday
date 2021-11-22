#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use tokio::time::{interval, Duration};
#[allow(unused_imports)]
use tokio::task;
#[allow(unused_imports)]
use chrono::NaiveDate;

use std::sync::Mutex;
use std::env;

#[macro_use]
extern crate lazy_static;
extern crate dotenv;
use diesel::SqliteConnection;
use diesel::Connection;
use dotenv::dotenv;
#[allow(unused_imports)]

#[allow(unused_imports)]

#[allow(unused_imports)]


mod bot;

pub mod models;
pub mod schema;
pub mod utils;

#[allow(unused_imports)]
use bot::commands;

lazy_static! {
    static ref DB: Mutex<SqliteConnection> = Mutex::new(establish_connection());
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let (bday_cache, nbday_cache) = db_util::load();
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let file_name = env::var("SQLITE_FILE").unwrap();
    SqliteConnection::establish(file_name.as_str()).unwrap()
}