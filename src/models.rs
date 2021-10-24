use diesel::sqlite::SqliteConnection;
use chrono::NaiveDate;
use crate::birthdays;
use crate::id_gen::id_gen;

#[derive(Queryable, Insertable, PartialEq, Debug)]
pub struct Birthday {
    pub id: i32,
    pub userid: String,
    pub channelid: String,
    pub guildid: String,
    pub date: NaiveDate,
    pub allexceptdate: bool,
    pub notifyall: bool,
    pub notifyrole: String,
}

impl Birthday {
    #[allow(clippy::too_many_arguments)]
    fn new(conn: &mut SqliteConnection,
        userid: u64,
        channelid: u64,
        guildid: u64,
        date: NaiveDate,
        allexceptdate: bool,
        notifyall: bool,
        notifyrole: u64) -> Birthday {
        Birthday {
            id: id_gen(conn),
            userid: userid.to_string(),
            channelid: channelid.to_string(),
            guildid: guildid.to_string(),
            date,
            allexceptdate,
            notifyall,
            notifyrole: notifyrole.to_string(),
        }
    }
}