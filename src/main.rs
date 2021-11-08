use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

extern crate dotenv;
use dotenv::dotenv;
#[allow(unused_imports)]
use tokio::time::{interval, Duration};
use chrono::NaiveDate;

use dbday::models;

#[allow(unused_imports)]
use dbday::macros;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _bday_cache: Arc<Mutex<HashMap<NaiveDate, Vec<models::Birthday>>>> = Arc::new(Mutex::new(HashMap::new()));
}
