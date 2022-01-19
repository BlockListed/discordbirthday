-- Your SQL goes here
CREATE TABLE birthdays (
  id TEXT PRIMARY KEY NOT NULL,
  userid TEXT NOT NULL,
  channelid TEXT NOT NULL,
  guildid TEXT NOT NULL,
  date DATE NOT NULL,
  lastdate DATE NOT NULL,
  allexceptdate BOOLEAN NOT NULL,
  notifyrole TEXT
);
