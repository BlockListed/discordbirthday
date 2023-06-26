use chrono::NaiveDate;

use sqlx::FromRow;

#[derive(FromRow, PartialEq, Eq, Debug, Clone)]
pub struct Birthday {
    pub id: String,
    pub userid: String,
    pub channelid: String,
    pub guildid: String,
    pub date: NaiveDate,
    pub lastdate: NaiveDate,
    pub allexceptdate: bool,
    /*
    This is Optional, because `@everyone` is handled differently in the Discord API, than when using other roles.
    So `None` means @everyone and every other role is a `Some`.
    */
    pub notifyrole: Option<String>,
}
