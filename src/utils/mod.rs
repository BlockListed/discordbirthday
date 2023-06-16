use crate::database;
use chrono::{Utc, NaiveDate, Datelike};
use nanoid::nanoid;
pub mod discord;
pub mod macros;

pub fn date_as_year_zero(date: NaiveDate) -> NaiveDate {
    date.with_year(0).unwrap()
}

pub fn date_as_year_today(date: NaiveDate) -> NaiveDate {
    let year = Utc::now().year();
    date.with_year(year).unwrap()
}

pub fn gen_id() -> String {
    let mut id_gen = nanoid!();
    // If the database lookup returns anything, recurse. (Very unlikely tho, because id is 21 digits of BASE64)
    if database::statements::check_if_id_exists(&id_gen) {
        id_gen = gen_id();
    }

    id_gen
}
