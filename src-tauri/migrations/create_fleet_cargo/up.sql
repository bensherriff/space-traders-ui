-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fleet (
  id TEXT PRIMARY KEY NOT NULL,
  symbol TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  units INTEGER NOT NULL
)