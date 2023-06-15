use diesel::delete;
use diesel::prelude::*;
use diesel::update;

use super::db_independant::{execute_query, load_query};

use chrono::prelude::*;

use crate::models::Birthday;
#[allow(clippy::wildcard_imports)]
use crate::schema::birthdays::dsl::*;

use crate::utils;

pub fn update_bday_last_updated(update_userid: &String, updated_lastdate: NaiveDate) {
    execute_query(
        update(birthdays.filter(userid.eq(update_userid))).set(lastdate.eq(updated_lastdate)),
    )
    .unwrap();
}

pub fn get_bdays_today(today_naive: NaiveDate) -> Vec<Birthday> {
    load_query(
        birthdays
            .filter(date.eq(utils::date_as_year_zero(today_naive)))
            .filter(allexceptdate.eq(false)),
    )
}

// Todo:
// Implement that getting first as option, as a function.
pub fn get_bday_with_userid(uid: &str) -> Option<Birthday> {
    let data: Vec<Birthday> = load_query(birthdays.filter(userid.eq(uid)).limit(1));
    if data.is_empty() {
        None
    } else {
        Some(data[0].clone())
    }
}

pub fn get_bdays_with_guildid(gid: &String) -> Vec<Birthday> {
    load_query(birthdays.filter(guildid.eq(gid)))
}

pub fn delete_bday_with_userid_and_guildid(
    d_userid: &String,
    d_guildid: &String,
) -> QueryResult<usize> {
    execute_query(
        delete(birthdays)
            .filter(userid.eq(d_userid))
            .filter(guildid.eq(d_guildid)),
    )
}

pub fn get_allexceptdata_bdays() -> Vec<Birthday> {
    load_query(birthdays.filter(allexceptdate.eq(true)))
}

pub fn check_if_id_exists(id_gen: &String) -> bool {
    let data: Vec<Birthday> = load_query(birthdays.filter(id.eq(id_gen)).limit(1));

    !data.is_empty()
}

pub fn insert_bday(bday: Birthday) -> QueryResult<usize> {
    execute_query(diesel::insert_into(birthdays).values(bday))
}

pub fn clear_all_lastdates() -> QueryResult<usize> {
    execute_query(update(birthdays).set(lastdate.eq(NaiveDate::from_ymd_opt(0, 1, 1).unwrap())))
}
