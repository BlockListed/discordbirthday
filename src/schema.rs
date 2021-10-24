table! {
    birthdays (id, userid) {
        id -> Integer,
        userid -> Text,
        channelid -> Text,
        guildid -> Text,
        date -> Date,
        allexceptdate -> Bool,
        notifyall -> Bool,
        notifyrole -> Text,
    }
}