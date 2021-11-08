table! {
    birthdays (userid, guildid, allexceptdate) {
        userid -> Text,
        channelid -> Text,
        guildid -> Text,
        date -> Date,
        allexceptdate -> Bool,
        notifyall -> Bool,
        notifyrole -> Text,
    }
}