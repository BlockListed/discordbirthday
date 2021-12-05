use chrono::NaiveDate;
use chrono::Utc;
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