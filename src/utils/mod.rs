use std::ops::DerefMut;

use chrono::NaiveDate;
use chrono::Utc;
use nanoid::nanoid;
use diesel::prelude::*;
use crate::DB;
use crate::schema;
use crate::models;
pub mod macros;
pub mod discord;

pub fn date_as_year_zero(date: NaiveDate) -> NaiveDate {
    date_as_year(date, 0)
}

pub fn date_as_year_today(date: NaiveDate) -> NaiveDate {
    let year = Utc::today().naive_local().format("%Y").to_string().parse::<i32>().unwrap();
    date_as_year(date, year)
}

pub fn date_as_year(date: NaiveDate, year: i32) -> NaiveDate {
    let date = date.format("%m-%d").to_string().split('-').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    NaiveDate::from_ymd(year, date[0], date[1])
}

pub fn gen_id() -> String {
    use schema::birthdays::dsl::*;
    let mut id_gen = nanoid!();
    // If the database lookup returns anything, recurse. (Very unlikely tho, because id is 21 digits of BASE64)
    if !birthdays.filter(id.eq(id_gen.clone())).limit(1).load::<models::Birthday>(DB.lock().unwrap().deref_mut()).unwrap().is_empty() {
        id_gen = gen_id();
    }

    id_gen
}