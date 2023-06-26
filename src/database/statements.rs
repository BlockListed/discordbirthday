use sea_query::{Expr, Query};
use sea_query_binder::SqlxBinder;

use chrono::prelude::*;

use crate::models::Birthday;
use crate::schema::Birthdays;

use super::get_db_and_query_builder;

use crate::utils;

fn all_columns() -> impl Iterator<Item = Birthdays> {
    use Birthdays::*;

    static COLS: &[Birthdays] = &[
        Id,
        Userid,
        Channelid,
        Guildid,
        Date,
        Lastdate,
        Allexceptdate,
        NotifyRole,
    ];

    COLS.iter().copied()
}

pub async fn update_bday_last_updated(update_userid: &str, updated_lastdate: NaiveDate) {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::update()
        .table(Table)
        .and_where(Expr::col(Userid).eq(update_userid))
        .value(Lastdate, updated_lastdate)
        .build_any_sqlx(query_builder);

    sqlx::query_with(&sql, arguments)
        .execute(pool)
        .await
        .unwrap();
}

pub async fn get_bdays_today(today_naive: NaiveDate) -> Vec<Birthday> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::select()
        .from(Table)
        .columns(all_columns())
        .and_where(Expr::col(Date).eq(utils::date_as_year_zero(today_naive)))
        .and_where(Expr::col(Allexceptdate).eq(false))
        .build_any_sqlx(query_builder);

    sqlx::query_as_with(&sql, arguments)
        .fetch_all(pool)
        .await
        .unwrap()
}

// Todo:
// Implement that getting first as option, as a function.
pub async fn get_bday_with_userid(uid: u64) -> Option<Birthday> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::select()
        .from(Table)
        .columns(all_columns())
        .and_where(Expr::col(Userid).eq(uid))
        .build_any_sqlx(query_builder);

    sqlx::query_as_with(&sql, arguments)
        .fetch_one(pool)
        .await
        .ok()
}

pub async fn get_bdays_with_guildid(gid: u64) -> Vec<Birthday> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::select()
        .from(Table)
        .columns(all_columns())
        .and_where(Expr::col(Guildid).eq(gid))
        .build_any_sqlx(query_builder);

    sqlx::query_as_with(&sql, arguments)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn delete_bday_with_userid_and_guildid(
    d_userid: &str,
    d_guildid: &str,
) -> sqlx::Result<u64> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::delete()
        .from_table(Table)
        .and_where(Expr::col(Userid).eq(d_userid))
        .and_where(Expr::col(Guildid).eq(d_guildid))
        .build_any_sqlx(query_builder);

    sqlx::query_with(&sql, arguments)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
}

pub async fn get_allexceptdate_bdays() -> Vec<Birthday> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::select()
        .from(Table)
        .columns(all_columns())
        .and_where(Expr::col(Allexceptdate).eq(true))
        .build_any_sqlx(query_builder);

    sqlx::query_as_with(&sql, arguments)
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn insert_bday(bday: Birthday) -> sqlx::Result<u64> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::insert()
        .into_table(Table)
        .columns(all_columns())
        .values_panic([
            bday.id.into(),
            bday.userid.into(),
            bday.channelid.into(),
            bday.guildid.into(),
            bday.date.into(),
            bday.lastdate.into(),
            bday.allexceptdate.into(),
            bday.notifyrole.into(),
        ])
        .build_any_sqlx(query_builder);

    sqlx::query_with(&sql, arguments)
        .execute(pool)
        .await
        .map(|v| v.rows_affected())
}

pub async fn clear_all_lastdates() -> sqlx::Result<u64> {
    use Birthdays::*;

    let (pool, query_builder) = get_db_and_query_builder().await;

    let (sql, arguments) = Query::update()
        .table(Table)
        .value(Lastdate, NaiveDate::from_ymd_opt(0, 1, 1).unwrap())
        .build_any_sqlx(query_builder);

    sqlx::query_with(&sql, arguments)
        .execute(pool)
        .await
        .map(|v| v.rows_affected())
}
