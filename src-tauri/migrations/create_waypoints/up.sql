-- Your SQL goes here
CREATE TABLE IF NOT EXISTS waypoints (
    waypoint_symbol TEXT PRIMARY KEY NOT NULL,
    system_symbol TEXT NOT NULL,
    waypoint_type TEXT NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    orbitals TEXT NOT NULL,
    faction TEXT,
    traits TEXT NOT NULL,
    chart_waypoint TEXT,
    chart_submitted_by TEXT,
    chart_submitted_on TEXT
)