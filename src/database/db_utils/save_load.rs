use diesel::sqlite::SqliteConnection;
use diesel::upsert::excluded;
use diesel::prelude::*;
use crate::Birthday;
use crate::birthdays::dsl as table;
use crate::birthdays::dsl::birthdays;

pub fn save(conn: &mut SqliteConnection, data: Vec<Birthday>) -> Result<(), diesel::result::Error> {
    for i in data.iter() {
        match diesel::insert_into(birthdays)
            .values(i)
            .on_conflict(table::id)
            .do_update()
            .set(table::id.eq(excluded(table::id)))
            .execute(conn) {
                Ok(_) => (),
                Err(errdata) => return Err(errdata)
            }
    }
    Ok(())
}

pub fn load(conn: &mut SqliteConnection) -> Result<Vec<Birthday>, diesel::result::Error> {
    match birthdays.load::<Birthday>(conn) {
        Ok(out) => Ok(out),
        Err(error) => Err(error)
    }
}