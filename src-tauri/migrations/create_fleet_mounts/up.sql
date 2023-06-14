-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fleet_mounts (
  id TEXT PRIMARY KEY NOT NULL,
  deposits TEXT,
  strength INTEGER,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  req_power INTEGER,
  req_crew INTEGER,
  req_slots INTEGER
)