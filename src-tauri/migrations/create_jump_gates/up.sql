-- Your SQL goes here
CREATE TABLE IF NOT EXISTS jump_gates (
  symbol TEXT NOT NULL,
  jump_range INTEGER NOT NULL,
  faction_symbol TEXT NOT NULL,
  connected_symbol TEXT NOT NULL,
  connected_sector_symbol TEXT NOT NULL,
  connected_system_type TEXT NOT NULL,
  connected_faction_symbol TEXT NOT NULL,
  connected_x INTEGER NOT NULL,
  connected_y INTEGER NOT NULL,
  connected_distance INTEGER NOT NULL,
  PRIMARY KEY (symbol, connected_symbol)
)