use diesel::{Queryable, Insertable};
use chrono::NaiveDate;
use crate::schema::birthdays;

#[derive(Queryable, Insertable, PartialEq, Debug, Clone)]
pub struct Birthday {
    pub userid: String,
    pub channelid: String,
    pub guildid: String,
    pub date: NaiveDate,
    pub allexceptdate: bool,
    pub notifyrole: String,
}