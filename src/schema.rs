use sea_query::Iden;

#[derive(Iden, Clone, Copy)]
pub enum Birthdays {
    Table,
    Id,
    Userid,
    Channelid,
    Guildid,
    Date,
    Lastdate,
    Allexceptdate,
    NotifyRole,
}
