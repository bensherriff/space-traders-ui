-- Your SQL goes here
CREATE TABLE IF NOT EXISTS market_transactions (
  waypoint_symbol TEXT NOT NULL,
  ship_symbol TEXT NOT NULL,
  trade_symbol TEXT NOT NULL,
  transaction_type TEXT NOT NULL,
  units INTEGER NOT NULL,
  price_per_unit INTEGER NOT NULL,
  total_price INTEGER NOT NULL,
  timestamp TEXT NOT NULL,
  PRIMARY KEY(waypoint_symbol, ship_symbol, trade_symbol, transaction_type, units, price_per_unit, total_price, timestamp)
)