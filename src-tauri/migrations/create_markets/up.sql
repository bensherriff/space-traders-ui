-- Your SQL goes here
CREATE TABLE IF NOT EXISTS markets (
  waypoint_symbol TEXT KEY NOT NULL,
  market_type TEXT NOT NULL,
  symbol TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  PRIMARY KEY(waypoint_symbol, market_type, symbol, name, description)
)