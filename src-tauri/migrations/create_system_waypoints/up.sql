-- Your SQL goes here
CREATE TABLE IF NOT EXISTS system_waypoints (
    waypoint_symbol TEXT PRIMARY KEY NOT NULL,
    system_symbol TEXT NOT NULL,
    waypoint_type TEXT NOT NULL,
    x INT NOT NULL,
    y INT NOT NULL
)