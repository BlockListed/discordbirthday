#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub use models::Birthday;
pub mod schema;
pub use schema::birthdays;
pub mod database;
pub use database::save_load;
pub use database::id_gen;
pub mod utils;
pub use utils::macros;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let file_name = env::var("SQLITE_FILE").unwrap();
    SqliteConnection::establish(file_name.as_str()).unwrap()
}