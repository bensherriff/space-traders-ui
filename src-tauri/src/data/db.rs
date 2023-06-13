use crate::{models::system::{System, SystemWaypoint}, data::{models::{NewSystemDB, NewSystemWaypointDB}}};

use diesel::{RunQueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

pub fn insert_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system: &System) {
  use crate::data::schema::systems;

  let mut connection = pool.get().unwrap();

  let mut _waypoints = "".to_string();
  for (index, waypoint) in system.waypoints.iter().enumerate() {
    insert_system_waypoint(pool, system.symbol.to_string(), waypoint);
    if index < system.waypoints.len() - 1 {
      _waypoints = format!("{}{},", _waypoints, waypoint.symbol);
    } else {
      _waypoints = format!("{}{}", _waypoints, waypoint.symbol);
    }
  }
  let mut _factions = "".to_string();
  for (index, faction) in system.factions.iter().enumerate() {
    if index < system.factions.len() - 1 {
      _factions = format!("{}{},", _factions, faction.symbol);
    } else {
      _factions = format!("{}{}", _factions, faction.symbol);
    }
  }

  let _system = NewSystemDB {
    system_symbol: &system.symbol,
    sector_symbol: &system.sector_symbol,
    system_type: &system.system_type.to_string(),
    x: system.x as i32,
    y: system.y as i32,
    waypoints: &_waypoints,
    factions: &_factions
  };
  insert_or_ignore_into(systems::table)
    .values(&_system)
    .execute(&mut connection)
    .expect("Error saving new system");
}

pub fn insert_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: String, waypoint: &SystemWaypoint) {
  use crate::data::schema::system_waypoints;

  let mut connection = pool.get().unwrap();

  let _system_waypoint = NewSystemWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x as i32,
    y: waypoint.y as i32
  };

  insert_or_ignore_into(system_waypoints::table)
    .values(&_system_waypoint)
    .execute(&mut connection)
    .expect("Error saving new system waypoint");
}
