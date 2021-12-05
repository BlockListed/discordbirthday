-- Your SQL goes here
CREATE TABLE birthdays (
  userid TEXT PRIMARY KEY NOT NULL,
  channelid TEXT NOT NULL,
  guildid TEXT NOT NULL,
  date DATE NOT NULL,
  lastdate DATE NOT NULL,
  allexceptdate BOOLEAN NOT NULL,
  notifyrole TEXT
);
