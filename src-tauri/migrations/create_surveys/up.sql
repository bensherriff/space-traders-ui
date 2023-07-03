-- Your SQL goes here
CREATE TABLE IF NOT EXISTS surveys (
  survey_signature TEXT PRIMARY KEY NOT NULL,
  waypoint_symbol TEXT NOT NULL,
  deposits TEXT NOT NULL,
  expiration TEXT NOT NULL,
  size TEXT NOT NULL,
  cooldown_ship_symbol TEXT NOT NULL,
  cooldown_total_seconds INTEGER NOT NULL,
  cooldown_remaining_seconds INTEGER NOT NULL,
  cooldown_expiration TEXT NOT NULL
)