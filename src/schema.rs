table! {
    birthdays (id) {
        id -> Text,
        userid -> Text,
        channelid -> Text,
        guildid -> Text,
        date -> Date,
        lastdate -> Date,
        allexceptdate -> Bool,
        notifyrole -> Nullable<Text>,
    }
}
