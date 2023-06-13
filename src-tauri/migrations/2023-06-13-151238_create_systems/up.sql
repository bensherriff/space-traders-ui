-- Your SQL goes here
CREATE TABLE systems (
    symbol TEXT PRIMARY KEY NOT NULL,
    sector_symbol TEXT NOT NULL,
    system_type TEXT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL,
    waypoints TEXT NOT NULL,
    factions TEXT NOT NULL
)