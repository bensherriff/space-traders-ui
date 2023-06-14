-- Your SQL goes here
CREATE TABLE IF NOT EXISTS systems (
    system_symbol TEXT PRIMARY KEY NOT NULL,
    sector_symbol TEXT NOT NULL,
    system_type TEXT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    factions TEXT NOT NULL
)