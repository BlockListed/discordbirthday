use std::future::Future;

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

pub fn gen_id() -> impl Future<Output = String> + std::marker::Send {
    async {
        let id_gen = nanoid!();
        let duplicate = database::statements::check_if_id_exists(id_gen.clone()).await;
        if duplicate {
            panic!("duplicated ID generated!");
        }

        id_gen
    }
}
