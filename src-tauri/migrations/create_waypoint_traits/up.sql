-- Your SQL goes here
CREATE TABLE IF NOT EXISTS waypoint_traits (
    trait_symbol TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL
)