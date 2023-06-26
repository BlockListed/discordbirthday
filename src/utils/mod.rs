use chrono::{Datelike, NaiveDate, Utc};
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
    nanoid!()
}
