-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fleet_cargo (
  id INTEGER PRIMARY KEY NOT NULL,
  ship_symbol TEXT NOT NULL,
  symbol TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  units INTEGER NOT NULL
)