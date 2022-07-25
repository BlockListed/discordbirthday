use diesel::delete;
use diesel::prelude::*;
use diesel::update;

use chrono::prelude::*;

use crate::database::DB;

use crate::models::Birthday;
#[allow(clippy::wildcard_imports)]
use crate::schema::birthdays::dsl::*;

use crate::utils;

#[macro_export]
macro_rules! get_postgres_db_lock {
    () => {
        &*DB.lock().unwrap()
    };
}

pub fn update_bday_last_updated(update_userid: &String, updated_lastdate: NaiveDate) {
    update(birthdays.filter(userid.eq(update_userid)))
        .set(lastdate.eq(updated_lastdate))
        .execute(get_postgres_db_lock!())
        .unwrap();
}

pub fn get_bdays_today(today_naive: NaiveDate) -> Vec<Birthday> {
    birthdays
        .filter(date.eq(utils::date_as_year_zero(today_naive)))
        .filter(allexceptdate.eq(false))
        .load::<Birthday>(get_postgres_db_lock!())
        .unwrap()
}

pub fn get_bday_with_userid(uid: &str) -> Birthday {
    birthdays
        .filter(userid.eq(uid))
        .first(get_postgres_db_lock!())
        .unwrap()
}

pub fn get_bdays_with_guildid(gid: &String) -> Vec<Birthday> {
    birthdays
        .filter(guildid.eq(gid))
        .load::<Birthday>(get_postgres_db_lock!())
        .unwrap()
}

pub fn delete_bday_with_userid_and_guildid(d_userid: &String, d_guildid: &String) -> QueryResult<usize> {
    delete(birthdays)
        .filter(userid.eq(d_userid))
        .filter(guildid.eq(d_guildid))
        .execute(get_postgres_db_lock!())
}

pub fn get_allexceptdata_bdays() -> Vec<Birthday> {
    birthdays
        .filter(allexceptdate.eq(true))
        .load::<Birthday>(get_postgres_db_lock!())
        .unwrap()
}

pub fn check_if_id_exists(id_gen: &String) -> bool {
    !birthdays
        .filter(id.eq(id_gen))
        .limit(1)
        .load::<Birthday>(get_postgres_db_lock!())
        .unwrap()
        .is_empty()
}

pub fn insert_bday(bday: Birthday) -> QueryResult<usize> {
    diesel::insert_into(birthdays)
        .values(bday)
        .execute(get_postgres_db_lock!())
}

pub fn clear_all_lastdates() -> QueryResult<usize> {
    update(birthdays)
        .set(lastdate.eq(NaiveDate::from_ymd(0, 1, 1)))
        .execute(get_postgres_db_lock!())
}
