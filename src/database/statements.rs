use diesel::delete;
use diesel::prelude::*;
use diesel::update;

use chrono::prelude::*;

use crate::load_dsl;
use crate::execute_dsl;
use crate::models::Birthday;
#[allow(clippy::wildcard_imports)]
use crate::schema::birthdays::dsl::*;

use crate::utils;

pub fn update_bday_last_updated(update_userid: &String, updated_lastdate: NaiveDate) {
    execute_dsl!(update(birthdays.filter(userid.eq(update_userid)))
    .set(lastdate.eq(updated_lastdate))).unwrap();

}

pub fn get_bdays_today(today_naive: NaiveDate) -> Vec<Birthday> {
    load_dsl!(
    birthdays
        .filter(date.eq(utils::date_as_year_zero(today_naive)))
        .filter(allexceptdate.eq(false))
    ).unwrap()
}

// Todo:
// Implement that getting first as option, as a function.
pub fn get_bday_with_userid(uid: &str) -> Option<Birthday> {
    let data: Vec<Birthday> = load_dsl!(
    birthdays
        .filter(userid.eq(uid))
        .limit(1)
    ).unwrap();
    if data.is_empty() {
        None
    } else {
        Some(data[0].clone())
    }
}

pub fn get_bdays_with_guildid(gid: &String) -> Vec<Birthday> {
    load_dsl!(
    birthdays
        .filter(guildid.eq(gid))
    ).unwrap()
}

pub fn delete_bday_with_userid_and_guildid(
    d_userid: &String,
    d_guildid: &String,
) -> QueryResult<usize> {
    execute_dsl!(
    delete(birthdays)
        .filter(userid.eq(d_userid))
        .filter(guildid.eq(d_guildid))
    )
}

pub fn get_allexceptdata_bdays() -> Vec<Birthday> {
    load_dsl!(
    birthdays
        .filter(allexceptdate.eq(true))
    ).unwrap()
}

pub fn check_if_id_exists(id_gen: &String) -> bool {
    let data: Vec<Birthday> = load_dsl!(birthdays
        .filter(id.eq(id_gen))
        .limit(1)).unwrap();

    !data.is_empty()
}

pub fn insert_bday(bday: Birthday) -> QueryResult<usize> {
    execute_dsl!(
    diesel::insert_into(birthdays)
        .values(bday)
    )
}

pub fn clear_all_lastdates() -> QueryResult<usize> {
    execute_dsl!(
    update(birthdays)
        .set(lastdate.eq(NaiveDate::from_ymd(0, 1, 1)))
    )
}
