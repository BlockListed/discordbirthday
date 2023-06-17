use diesel::prelude::*;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::query_dsl::LoadQuery;

use diesel_async::methods::ExecuteDsl as AsyncExecuteDsl;
use diesel_async::methods::LoadQuery as AsyncLoadQuery;
use diesel_async::RunQueryDsl as AsyncRunQueryDsl;
use diesel_async::AsyncPgConnection;

use super::DbType;

pub async fn execute_query<Q>(query: Q) -> QueryResult<usize>
where
    Q: AsyncRunQueryDsl<AsyncPgConnection>
        + RunQueryDsl<SqliteConnection>
        + AsyncExecuteDsl<AsyncPgConnection>
        + ExecuteDsl<SqliteConnection>,
{
    match &mut *super::get_db().await.lock().await {
        DbType::Postgres(conn) => AsyncRunQueryDsl::execute(query, &mut conn.asynchronous).await,
        DbType::Sqlite(conn) => RunQueryDsl::execute(query, conn),
    }
}

pub async fn load_query<'a, R, Q>(query: Q) -> Vec<R>
where
    Q: AsyncRunQueryDsl<AsyncPgConnection>
        + RunQueryDsl<SqliteConnection>
        + AsyncLoadQuery<'a, AsyncPgConnection, R>
        + LoadQuery<'a, SqliteConnection, R>
        + 'a,
    R: Send,
{
    match &mut *super::get_db().await.lock().await {
        DbType::Postgres(conn) => AsyncRunQueryDsl::load(query, &mut conn.asynchronous).await.unwrap(),
        DbType::Sqlite(conn) => RunQueryDsl::load(query, conn).unwrap(),
    }
}
