use diesel::table;

table! {
    birthdays (userid) {
        userid -> Text,
        channelid -> Text,
        guildid -> Text,
        date -> Date,
        allexceptdate -> Bool,
        notifyrole -> Text,
    }
}