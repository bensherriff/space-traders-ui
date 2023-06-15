-- Your SQL goes here
CREATE TABLE IF NOT EXISTS market_trade_goods (
  waypoint_symbol TEXT NOT NULL,
  symbol TEXT NOT NULL,
  trade_volume INTEGER NOT NULL,
  supply_type TEXT NOT NULL,
  purchase_price INTEGER NOT NULL,
  sell_price INTEGER NOT NULL,
  PRIMARY KEY (waypoint_symbol, symbol)
)