use crate::database;
use chrono::NaiveDate;
use chrono::Utc;
use nanoid::nanoid;
pub mod discord;
pub mod macros;

pub fn date_as_year_zero(date: NaiveDate) -> NaiveDate {
    date_as_year(date, 0)
}

pub fn date_as_year_today(date: NaiveDate) -> NaiveDate {
    let year = Utc::today()
        .naive_local()
        .format("%Y")
        .to_string()
        .parse::<i32>()
        .unwrap();
    date_as_year(date, year)
}

pub fn date_as_year(date: NaiveDate, year: i32) -> NaiveDate {
    let date = date
        .format("%m-%d")
        .to_string()
        .split('-')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    NaiveDate::from_ymd(year, date[0], date[1])
}

pub fn gen_id() -> String {
    let mut id_gen = nanoid!();
    // If the database lookup returns anything, recurse. (Very unlikely tho, because id is 21 digits of BASE64)
    if database::statements::check_if_id_exists(&id_gen) {
        id_gen = gen_id();
    }

    id_gen
}
