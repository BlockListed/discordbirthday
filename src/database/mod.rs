use std::sync::Mutex;
use diesel::prelude::*;
use once_cell::sync::Lazy;

#[cfg(all(feature = "postgres", not(features = "sqlite")))]
pub static DB: Lazy<Mutex<PgConnection>> = Lazy::new(|| {
	Mutex::new(PgConnection::establish(&std::env::var("DATABASE_URL").unwrap()).unwrap())
});

#[cfg(feature = "sqlite")]
pub static DB: Lazy<Mutex<SqliteConnection>> = Lazy::new(|| {
	Mutex::new(SqliteConnection::establish(&std::env::var("DATABASE_URL").unwrap())).unwrap()
});