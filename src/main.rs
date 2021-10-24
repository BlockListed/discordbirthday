use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

extern crate dotenv;
use dotenv::dotenv;
#[allow(unused_imports)]
use tokio::time::{interval, Duration};
use chrono::NaiveDate;

#[allow(unused_imports)]
use discordbirthday::save_load;
use discordbirthday::models;

fn main() {
    dotenv().ok();

    let _bday_cache: Arc<Mutex<HashMap<NaiveDate, Vec<models::Birthday>>>> = Arc::new(Mutex::new(HashMap::new()));

}
