use diesel::prelude::*;

use diesel_async::AsyncPgConnection;
use diesel_async::AsyncConnection;
use tokio::sync::OnceCell;
use tokio::sync::Mutex;

pub mod db_independant;
pub mod statements;

pub struct Postgres {
    pub asynchronous: AsyncPgConnection,
    pub synchronous: PgConnection,
}

pub enum DbType {
    Postgres(Postgres),
    Sqlite(SqliteConnection),
}

pub async fn get_db() -> &'static Mutex<DbType> {
    static DB: OnceCell<Mutex<DbType>> = OnceCell::const_new();

    DB.get_or_init(|| async {
        let db_type = std::env::var("DATABASE_TYPE").expect("Missing database type!");
        let db_url = std::env::var("DATABASE_URL").unwrap();

        let db_conn = match db_type.as_str() {
            "postgres" => {
                let asynchronous = AsyncPgConnection::establish(&db_url)
                    .await
                    .expect("Failed to initialise async postgres db");
                let synchronous = PgConnection::establish(&db_url)
                    .expect("Failed to initialise sync postgres db");
                
                DbType::Postgres(Postgres { asynchronous, synchronous })
            },
            "sqlite" => DbType::Sqlite(
                    SqliteConnection::establish(&db_url)
                        .expect("Failed to initialise sqlite db")
            ),
            _ => {
                panic!("invalid db type!");
            }
        };
        
        Mutex::new(db_conn)
    }).await
}