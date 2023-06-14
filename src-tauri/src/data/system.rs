use std::str::FromStr;

use crate::data::models::{WaypointDB, NewWaypointDB};
use crate::models::SymbolResponse;
use crate::models::system::{System, SystemWaypoint, SystemType};
use crate::data::{models::{SystemDB, NewSystemDB}, schema};
use crate::models::waypoint::WaypointType;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

pub fn get_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str) -> Option<System> {
  use schema::systems;

  let mut connection = pool.get().unwrap();
  let result: Result<SystemDB, diesel::result::Error> = systems::table
    .filter(systems::system_symbol.eq(system_symbol))
    .select(SystemDB::as_select())
    .first(&mut connection);
  match result {
      Ok(r) => {
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
          waypoints: get_system_waypoints(pool, system_symbol),
          factions: system_factions
        }) 
      }
      Err(_err) => None
    }
}

pub fn insert_system(pool: &Pool<ConnectionManager<SqliteConnection>>, system: &System) {
  use schema::systems;

  let mut connection: diesel::r2d2::PooledConnection<ConnectionManager<SqliteConnection>> = pool.get().unwrap();
  let mut _waypoints = "".to_string();
  for (_index, waypoint) in system.waypoints.iter().enumerate() {
    insert_system_waypoint(pool, &system.symbol, waypoint);
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
    factions: &_factions
  };
  insert_or_ignore_into(systems::table)
    .values(&_system)
    .execute(&mut connection)
    .expect("Error saving new system");
}

pub fn get_system_waypoints(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str) -> Vec<SystemWaypoint> {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<WaypointDB>, diesel::result::Error> = waypoints::table
    .filter(waypoints::system_symbol.eq(system_symbol))
    .select(WaypointDB::as_select())
    .load(&mut connection);
  match result {
    Ok(w) => {
      let mut waypoints: Vec<SystemWaypoint> = vec![];
      for (_index, item) in w.iter().enumerate() {
        waypoints.push(SystemWaypoint {
          symbol: item.system_symbol.to_string(),
          waypoint_type: WaypointType::from_str(&item.waypoint_type).unwrap(),
          x: item.x,
          y: item.y
        });
      }
      waypoints
    }
    Err(_err) => vec![]
  }
}

pub fn get_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint_symbol: &str) -> Option<SystemWaypoint> {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<WaypointDB, diesel::result::Error> = waypoints::table
    .filter(waypoints::system_symbol.eq(system_symbol).and(waypoints::waypoint_symbol.eq(waypoint_symbol)))
    .select(WaypointDB::as_select())
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

pub fn insert_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint: &SystemWaypoint) {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let _system_waypoint = NewWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y,
    orbitals: "",
    faction: None,
    traits: "",
    chart_waypoint: None,
    chart_submitted_by: None,
    chart_submitted_on: None
  };



  insert_or_ignore_into(waypoints::table)
    .values(&_system_waypoint)
    .execute(&mut connection)
    .expect("Error saving new system waypoint");
}
