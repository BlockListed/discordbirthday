use diesel::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod statements;

#[cfg(all(feature = "postgres", not(features = "sqlite")))]
pub static DB: Lazy<Mutex<PgConnection>> = Lazy::new(|| {
    Mutex::new(PgConnection::establish(&std::env::var("DATABASE_URL").unwrap()).unwrap())
});

#[cfg(feature = "sqlite")]
pub static DB: Lazy<Mutex<SqliteConnection>> = Lazy::new(|| {
    Mutex::new(SqliteConnection::establish(
        &std::env::var("DATABASE_URL").unwrap(),
    ))
    .unwrap()
});
