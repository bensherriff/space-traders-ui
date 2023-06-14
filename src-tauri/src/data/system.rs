use std::str::FromStr;

use crate::data::models::SystemDB;
use crate::models::SymbolResponse;
use crate::models::system::{System, SystemWaypoint, SystemType};
use crate::data::{models::{NewSystemDB, NewSystemWaypointDB}, schema};
use crate::models::waypoint::WaypointType;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

use super::models::SystemWaypointDB;

pub fn get_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str) -> Option<System> {
  use schema::systems;

  let mut connection = pool.get().unwrap();
  let result: Result<SystemDB, diesel::result::Error> = systems::table
    .filter(systems::system_symbol.eq(system_symbol))
    .select(SystemDB::as_select())
    .first(&mut connection);
  match result {
      Ok(r) => {
        let mut system_waypoints: Vec<SystemWaypoint> = vec![];
        let waypoints = r.waypoints.split(",");
        for waypoint_symbol in waypoints {
          match get_system_waypoint(pool, system_symbol, waypoint_symbol) {
            Some(w) => {
              system_waypoints.push(w);
            },
            None => {}
          };
        }

        let mut system_factions: Vec<SymbolResponse> = vec![];
        let factions = r.factions.split(",");
        for faction_symbol in factions {
          system_factions.push(SymbolResponse { symbol: faction_symbol.to_string() })
        }

        Some(System {
          symbol: r.system_symbol,
          sector_symbol: r.sector_symbol,
          system_type: SystemType::from_str(&r.system_type).unwrap(),
          x: r.x,
          y: r.y,
          waypoints: system_waypoints,
          factions: system_factions
        }) 
      }
      Err(_err) => None
    }
}

pub fn insert_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system: &System) {
  use schema::systems;

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
    x: system.x,
    y: system.y,
    waypoints: &_waypoints,
    factions: &_factions
  };
  insert_or_ignore_into(systems::table)
    .values(&_system)
    .execute(&mut connection)
    .expect("Error saving new system");
}

pub fn get_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint_symbol: &str) -> Option<SystemWaypoint> {
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<SystemWaypointDB, diesel::result::Error> = system_waypoints::table
    .filter(system_waypoints::system_symbol.eq(system_symbol).and(system_waypoints::waypoint_symbol.eq(waypoint_symbol)))
    .select(SystemWaypointDB::as_select())
    .first(&mut connection);
  match result {
    Ok(w) => {
      Some(SystemWaypoint {
        symbol: w.waypoint_symbol,
        waypoint_type: WaypointType::from_str(&w.waypoint_type).unwrap(),
        x: w.x,
        y: w.y
      })
    }
    Err(_err) => None
  }
}

pub fn insert_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: String, waypoint: &SystemWaypoint) {
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let _system_waypoint = NewSystemWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y
  };

  insert_or_ignore_into(system_waypoints::table)
    .values(&_system_waypoint)
    .execute(&mut connection)
    .expect("Error saving new system waypoint");
}
