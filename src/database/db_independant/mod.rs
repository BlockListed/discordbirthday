use diesel::prelude::*;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::query_dsl::LoadQuery;

use super::DbType;

pub fn execute_query<Q>(query: Q) -> QueryResult<usize>
where
    Q: RunQueryDsl<PgConnection>
        + RunQueryDsl<SqliteConnection>
        + ExecuteDsl<PgConnection>
        + ExecuteDsl<SqliteConnection>,
{
    match &mut *super::DB.lock().unwrap() {
        DbType::Postgres(conn) => query.execute(conn),
        DbType::Sqlite(conn) => query.execute(conn),
    }
}

pub fn load_query<'a, R, Q>(query: Q) -> Vec<R>
where
    Q: RunQueryDsl<PgConnection>
        + RunQueryDsl<SqliteConnection>
        + LoadQuery<'a, PgConnection, R>
        + LoadQuery<'a, SqliteConnection, R>,
{
    match &mut *super::DB.lock().unwrap() {
        DbType::Postgres(conn) => query.load(conn).unwrap(),
        DbType::Sqlite(conn) => query.load(conn).unwrap(),
    }
}
