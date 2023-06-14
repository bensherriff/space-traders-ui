use std::str::FromStr;

use crate::data::models::{WaypointDB, NewWaypointDB, WaypointTraitDB, NewWaypointTraitDB, SystemWaypointDB, NewSystemWaypointDB};
use crate::models::SymbolResponse;
use crate::models::chart::Chart;
use crate::models::system::{System, SystemWaypoint, SystemType};
use crate::data::{models::{SystemDB, NewSystemDB}, schema};
use crate::models::trait_type::TraitType;
use crate::models::waypoint::{WaypointType, Waypoint, WaypointTrait};

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
        for (_index, faction_symbol) in r.factions.split(",").filter(|&x| !x.is_empty()).enumerate() {
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
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<SystemWaypointDB>, diesel::result::Error> = system_waypoints::table
    .filter(system_waypoints::system_symbol.eq(system_symbol))
    .select(SystemWaypointDB::as_select())
    .load(&mut connection);
  match result {
    Ok(w) => {
      let mut waypoints: Vec<SystemWaypoint> = vec![];
      for (_index, item) in w.iter().enumerate() {
        waypoints.push(SystemWaypoint {
          symbol: item.waypoint_symbol.to_string(),
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

// pub fn get_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint_symbol: &str) -> Option<SystemWaypoint> {
//   use schema::waypoints;

//   let mut connection = pool.get().unwrap();
//   let result: Result<WaypointDB, diesel::result::Error> = waypoints::table
//     .filter(waypoints::system_symbol.eq(system_symbol).and(waypoints::waypoint_symbol.eq(waypoint_symbol)))
//     .select(WaypointDB::as_select())
//     .first(&mut connection);
//   match result {
//     Ok(w) => {
//       Some(SystemWaypoint {
//         symbol: w.waypoint_symbol,
//         waypoint_type: WaypointType::from_str(&w.waypoint_type).unwrap(),
//         x: w.x,
//         y: w.y
//       })
//     }
//     Err(_err) => None
//   }
// }

pub fn insert_system_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, system_symbol: &str, waypoint: &SystemWaypoint) {
  use schema::system_waypoints;

  let mut connection = pool.get().unwrap();
  let _system_waypoint = NewSystemWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y,
  };

  insert_or_ignore_into(system_waypoints::table)
    .values(&_system_waypoint)
    .execute(&mut connection)
    .expect("Error saving new system waypoint");
}

pub fn get_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_symbol: &str) -> Option<Waypoint> {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let result: Result<WaypointDB, diesel::result::Error> = waypoints::table
    .filter(waypoints::waypoint_symbol.eq(waypoint_symbol))
    .select(WaypointDB::as_select())
    .first(&mut connection);
  match result {
    Ok(w) => {
      let mut orbitals: Vec<SymbolResponse> = vec![];
      for (_index, item) in w.orbitals.split(",").filter(|&x| !x.is_empty()).enumerate() {
        orbitals.push(SymbolResponse { symbol: item.to_string() })
      }
      let mut traits: Vec<WaypointTrait> = vec![];
      for (_index, item) in w.traits.split(",").filter(|&x| !x.is_empty()).enumerate() {
        match get_waypoint_trait(pool, item) {
          Some(t) => traits.push(t),
          None => {}
        }
      }

      Some(Waypoint {
        symbol: w.waypoint_symbol,
        waypoint_type: WaypointType::from_str(&w.waypoint_type).unwrap(),
        system_symbol: w.system_symbol,
        x: w.x,
        y: w.y,
        orbitals,
        faction: match w.faction {
          Some(f) => Some(SymbolResponse { symbol: f }),
          None => None
        },
        traits,
        chart: Some(Chart {
          waypoint: w.chart_waypoint,
          submitted_by: w.chart_submitted_by,
          submitted_on: w.chart_submitted_on
        })
      })
    }
    Err(_err) => None
  }
}

pub fn insert_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint: &Waypoint) {
  use schema::waypoints;

  let mut connection = pool.get().unwrap();
  let mut orbitals = "".to_string();
  for (index, item) in waypoint.orbitals.iter().enumerate() {
    if index < waypoint.orbitals.len() - 1 {
      orbitals = format!("{}{},", orbitals, item.symbol);
    } else {
      orbitals = format!("{}{}", orbitals, item.symbol);
    }
  }
  let mut traits = "".to_string();
  for (index, item) in waypoint.traits.iter().enumerate() {
    insert_waypoint_trait(pool, item);
    if index < waypoint.traits.len() - 1 {
      traits = format!("{}{},", traits, item.symbol);
    } else {
      traits = format!("{}{}", traits, item.symbol);
    }
  }

  let _waypoint = NewWaypointDB {
    waypoint_symbol: &waypoint.symbol,
    system_symbol: &waypoint.system_symbol,
    waypoint_type: &waypoint.waypoint_type.to_string(),
    x: waypoint.x,
    y: waypoint.y,
    orbitals: &orbitals,
    faction: match &waypoint.faction {
      Some(f) => Some(&f.symbol),
      None => None
    },
    traits: &traits,
    chart_waypoint: match &waypoint.chart {
      Some(c) => match &c.waypoint {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
    chart_submitted_by: match &waypoint.chart {
      Some(c) => match &c.submitted_by {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
    chart_submitted_on: match &waypoint.chart {
      Some(c) => match &c.submitted_on {
        Some(w) => Some(&w),
        None => None
      },
      None => None
    },
  };

  insert_or_ignore_into(waypoints::table)
    .values(&_waypoint)
    .execute(&mut connection)
    .expect("Error saving new waypoint");
}

pub fn get_waypoint_trait(pool: &Pool<ConnectionManager<SqliteConnection>>, trait_symbol: &str) -> Option<WaypointTrait> {
  use schema::waypoint_traits;

  let mut connection = pool.get().unwrap();
  let result: Result<WaypointTraitDB, diesel::result::Error> = waypoint_traits::table
    .filter(waypoint_traits::trait_symbol.eq(trait_symbol))
    .select(WaypointTraitDB::as_select())
    .first(&mut connection);
  match result {
    Ok(t) => {
      Some(WaypointTrait { symbol: TraitType::from_str(&t.trait_symbol).unwrap(), name: t.name, description: t.description })
    }
    Err(_err) => None
  }
}

pub fn insert_waypoint_trait(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_trait: &WaypointTrait) {
  use schema::waypoint_traits;

  let mut connection = pool.get().unwrap();
  let _waypoint_trait = NewWaypointTraitDB {
    trait_symbol: &waypoint_trait.symbol.to_string(),
    name: &waypoint_trait.name,
    description: &waypoint_trait.description,
  };

  insert_or_ignore_into(waypoint_traits::table)
    .values(&_waypoint_trait)
    .execute(&mut connection)
    .expect("Error saving new waypoint trait");
}
