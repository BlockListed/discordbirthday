use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use fastrand::i32 as geni32;

use crate::birthdays::dsl as table;
use crate::birthdays::dsl::birthdays;

fn check_unique(conn: &mut SqliteConnection, id_check: i32) -> bool {
    matches!(birthdays.filter(table::id.eq(id_check))
        .select(table::id)
        .load::<i32>(conn).unwrap().len(), 0)
}

pub fn id_gen(conn: &mut SqliteConnection) -> i32 {
    let id = geni32(..);
    match check_unique(conn, id) {
        true => id,
        false => id_gen(conn),
    }
}