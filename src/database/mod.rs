use diesel::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod db_independant;
pub mod statements;
pub enum DbType {
    Postgres(PgConnection),
    Sqlite(SqliteConnection),
}

pub static DB: Lazy<Mutex<DbType>> = Lazy::new(|| {
    match std::env::var("DATABASE_TYPE")
        .expect("Database type not provided")
        .as_str()
    {
        "postgres" => Mutex::new(DbType::Postgres(
            PgConnection::establish(&std::env::var("DATABASE_URL").unwrap())
                .expect("Failed to initialise postgres DB."),
        )),
        "sqlite" => Mutex::new(DbType::Sqlite(
            SqliteConnection::establish(&std::env::var("DATABASE_URL").unwrap())
                .expect("Failed to initialise sqlite DB."),
        )),
    }
});
