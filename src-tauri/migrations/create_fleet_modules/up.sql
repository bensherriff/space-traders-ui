-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fleet_modules (
  id TEXT PRIMARY KEY NOT NULL,
  capacity INTEGER,
  range INTEGER,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  req_power INTEGER,
  req_crew INTEGER,
  req_slots INTEGER
)