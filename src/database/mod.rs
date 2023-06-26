#![allow(clippy::enum_glob_use)]
use sqlx::AnyPool;

use sea_query::SchemaBuilder;
use sea_query::{PostgresQueryBuilder, QueryBuilder, SqliteQueryBuilder};

use tokio::sync::OnceCell;

pub mod migration;
pub mod statements;

pub async fn get_db() -> &'static AnyPool {
    static DB: OnceCell<AnyPool> = OnceCell::const_new();

    DB.get_or_init(|| async {
        let db_url = std::env::var("DATABASE_URL").unwrap();

        AnyPool::connect(&db_url)
            .await
            .expect("Couldn't connect to database!")
    })
    .await
}

pub fn get_query_builder(pool: &AnyPool) -> &'static (dyn QueryBuilder + Sync) {
    use sqlx::any::AnyKind::*;
    match pool.any_kind() {
        Postgres => &PostgresQueryBuilder {},
        Sqlite => &SqliteQueryBuilder {},
    }
}

pub fn get_schema_builder(pool: &AnyPool) -> &'static (dyn SchemaBuilder + Sync) {
    use sqlx::any::AnyKind::*;
    match pool.any_kind() {
        Postgres => &PostgresQueryBuilder {},
        Sqlite => &SqliteQueryBuilder {},
    }
}

pub async fn get_db_and_query_builder() -> (&'static AnyPool, &'static (dyn QueryBuilder + Sync)) {
    let pool = get_db().await;

    let query_builder = get_query_builder(pool);

    (pool, query_builder)
}
