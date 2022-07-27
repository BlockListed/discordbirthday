// No, I don't know what all this trait black magic does.
// The compiler told me to add this and it's working so I
// do not care.

use diesel::prelude::*;

pub fn execute_dsl_postgres<T>(statement: T, database: &PgConnection) -> QueryResult<usize>
where
    T: RunQueryDsl<PgConnection>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::pg::Pg>,
{
    statement.execute(database)
}

pub fn load_dsl_postgres<T, D>(statement: T, database: &PgConnection) -> QueryResult<Vec<D>>
where
    T: RunQueryDsl<PgConnection>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::pg::Pg>
        + diesel::query_builder::Query,
    D: diesel::Queryable<<T as diesel::query_builder::Query>::SqlType, diesel::pg::Pg>,
    diesel::pg::Pg: diesel::sql_types::HasSqlType<<T as diesel::query_builder::Query>::SqlType>,
{
    statement.load(database)
}

pub fn execute_dsl_sqlite<T>(statement: T, database: &SqliteConnection) -> QueryResult<usize>
where
    T: RunQueryDsl<SqliteConnection>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
{
    statement.execute(database)
}

pub fn load_dsl_sqlite<T, D>(statement: T, database: &SqliteConnection) -> QueryResult<Vec<D>>
where
    T: RunQueryDsl<SqliteConnection>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>
        + diesel::query_builder::Query,
    D: diesel::Queryable<<T as diesel::query_builder::Query>::SqlType, diesel::sqlite::Sqlite>,
    diesel::sqlite::Sqlite:
        diesel::sql_types::HasSqlType<<T as diesel::query_builder::Query>::SqlType>,
{
    statement.load(database)
}

#[macro_export]
macro_rules! load_dsl {
    ($statement:expr) => {
        match &*$crate::database::DB.lock().unwrap() {
            $crate::database::DbType::Postgres(x) => {
                $crate::database::db_independant::load_dsl_postgres($statement, &x)
            },
            $crate::database::DbType::Sqlite(x) => {
                $crate::database::db_independant::load_dsl_sqlite($statement, &x)
            }
        }
    };
}

#[macro_export]
macro_rules! execute_dsl {
    ($statement:expr) => {
        match &*$crate::database::DB.lock().unwrap() {
            $crate::database::DbType::Postgres(x) => {
                $crate::database::db_independant::execute_dsl_postgres($statement, &x)
            },
            $crate::database::DbType::Sqlite(x) => {
                $crate::database::db_independant::execute_dsl_sqlite($statement, &x)
            }
        }
    };
}