use sea_query::{ColumnDef, Table};
use sqlx::query;

use crate::schema::Birthdays;

use super::{get_db, get_schema_builder};

pub async fn migrate() {
    let pool = get_db().await;

    let schema_builder = get_schema_builder(pool);

    let create_birthday_table = Table::create()
        .table(Birthdays::Table)
        .if_not_exists()
        .col(ColumnDef::new(Birthdays::Id).text().primary_key())
        .col(ColumnDef::new(Birthdays::Userid).text().not_null())
        .col(ColumnDef::new(Birthdays::Channelid).text().not_null())
        .col(ColumnDef::new(Birthdays::Guildid).text().not_null())
        .col(ColumnDef::new(Birthdays::Date).date().not_null())
        .col(ColumnDef::new(Birthdays::Lastdate).date().not_null())
        .col(
            ColumnDef::new(Birthdays::Allexceptdate)
                .boolean()
                .not_null(),
        )
        .col(ColumnDef::new(Birthdays::NotifyRole).text())
        .build_any(schema_builder);

    query(&create_birthday_table).execute(pool).await.unwrap();
}
