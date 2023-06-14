use std::str::FromStr;

use crate::data::models::{CargoDB, ModuleDB, MountDB, NewCargoDB, NewModuleDB, NewMountDB};
use crate::models::ship::*;
use crate::data::{models::{ShipDB, NewShipDB}, schema};
use crate::models::ship::cargo::{Cargo, CargoItem};
use crate::models::ship::crew::Rotation;
use crate::models::ship::engine::{Engine, EngineType};
use crate::models::ship::frame::{Frame, FrameType};
use crate::models::ship::fuel::{Fuel, Consumed};
use crate::models::ship::module::{Module, ModuleType};
use crate::models::ship::mount::{Mount, MountType, DepositType};
use crate::models::ship::navigation::{Navigation, Route, RouteWaypoint, NavStatus, FlightMode};
use crate::models::ship::reactor::Reactor;
use crate::models::ship::registration::{Registration, Role};
use crate::models::ship::requirements::Requirements;
use crate::models::waypoint::WaypointType;

use diesel::{prelude::*, insert_into, replace_into, delete};
use diesel::{RunQueryDsl, QueryDsl, SqliteConnection, r2d2::{Pool, ConnectionManager}};
use log::warn;

pub fn get_ship(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Option<Ship> {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result: Result<ShipDB, diesel::result::Error> = fleet::table
    .filter(fleet::ship_symbol.eq(ship_symbol))
    .select(ShipDB::as_select())
    .first(&mut connection);

  match result {
    Ok(r) => {
      // Ship Cargo
      let ship_symbol = &r.ship_symbol.to_string();
      let cargo_items = get_cargo_items(pool, &ship_symbol);
      let cargo = Cargo {
        capacity: r.cargo_capacity,
        units: r.cargo_units,
        inventory: cargo_items
      };
      Some(Ship {
        symbol: (&ship_symbol).to_string(),
        registration: Registration {
          name: r.reg_name,
          faction_symbol: r.reg_faction_symbol,
          role: Role::from_str(&r.reg_role).unwrap()
        },
        nav: Navigation {
          system_symbol: r.nav_system_symbol,
          waypoint_symbol: r.nav_waypoint_symbol,
          route: Route {
            destination: RouteWaypoint {
              symbol: r.nav_dest_symbol,
              waypoint_type: WaypointType::from_str(&r.nav_dest_waypoint_type).unwrap(),
              system_symbol: r.nav_dest_system_symbol,
              x: r.nav_dest_x,
              y: r.nav_dest_y
            },
            departure: RouteWaypoint {
              symbol: r.nav_dep_symbol,
              waypoint_type: WaypointType::from_str(&r.nav_dep_waypoint_type).unwrap(),
              system_symbol: r.nav_dep_system_symbol,
              x: r.nav_dep_x,
              y: r.nav_dep_y
            },
            departure_time: r.nav_departure_time,
            arrival_time: r.nav_arrival_time
          },
          status: NavStatus::from_str(&r.nav_status).unwrap(),
          flight_mode: FlightMode::from_str(&r.nav_flight_mode).unwrap()
        },
        crew: crew::Crew {
          current: r.crew_current,
          required: r.crew_required,
          capacity: r.crew_capacity,
          rotation: Rotation::from_str(&r.crew_rotation).unwrap(),
          morale: r.crew_moral,
          wages: r.crew_wages
        },
        frame: Frame {
        symbol: FrameType::from_str(&r.frame_symbol).unwrap(),
        name: r.frame_name,
        description: r.frame_desc,
        condition: r.frame_condition,
        module_slots: r.frame_modules,
        mounting_points: r.frame_mounts,
        fuel_capacity: r.frame_fuel_capacity,
        requirements: Requirements { power: r.frame_req_power, crew: r.frame_req_crew, slots: r.frame_req_slots }
      },
        reactor: Reactor {
          symbol: r.reactor_symbol,
          name: r.reactor_name,
          description: r.reactor_desc,
          condition: r.reactor_condition,
          power_output: r.reactor_power_output,
          requirements: Requirements { power: r.reactor_req_power, crew: r.reactor_req_crew, slots: r.reactor_req_slots }
        },
        engine: Engine {
          symbol: EngineType::from_str(&r.engine_symbol).unwrap(),
          name: r.engine_name,
          description: r.engine_desc,
          condition: r.engine_condition,
          speed: r.engine_speed,
          requirements: Requirements { power: r.engine_req_power, crew: r.engine_req_power, slots: r.engine_req_slots }
        },
        modules: get_modules(pool, &ship_symbol),
        mounts: get_mounts(pool, &ship_symbol),
        cargo,
        fuel: Fuel {
          current: r.fuel_current,
          capacity: r.fuel_capacity,
          consumed: Consumed { amount: r.fuel_consumed_amount, timestamp: r.fuel_consumed_timestamp }
        }
      })
    }
    Err(_err) => None
  }
}

pub fn insert_ship(pool: &Pool<ConnectionManager<SqliteConnection>>, ship: &Ship) {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let _ship = NewShipDB {
    ship_symbol: &ship.symbol,
    reg_name: &ship.registration.name,
    reg_faction_symbol: &ship.registration.faction_symbol,
    reg_role: &ship.registration.role.to_string(),
    nav_system_symbol: &ship.nav.system_symbol,
    nav_status: &ship.nav.status.to_string(),
    nav_flight_mode: &ship.nav.flight_mode.to_string(),
    nav_waypoint_symbol: &ship.nav.waypoint_symbol,
    nav_departure_time: &ship.nav.route.departure_time,
    nav_arrival_time: &ship.nav.route.arrival_time,
    nav_dest_symbol: &ship.nav.route.destination.symbol,
    nav_dest_waypoint_type: &ship.nav.route.destination.waypoint_type.to_string(),
    nav_dest_system_symbol: &ship.nav.route.destination.system_symbol,
    nav_dest_x: ship.nav.route.destination.x,
    nav_dest_y: ship.nav.route.destination.y,
    nav_dep_symbol: &ship.nav.route.departure.symbol,
    nav_dep_waypoint_type: &ship.nav.route.departure.waypoint_type.to_string(),
    nav_dep_system_symbol: &ship.nav.route.departure.system_symbol,
    nav_dep_x: ship.nav.route.departure.x,
    nav_dep_y: ship.nav.route.departure.y,
    crew_current: ship.crew.current,
    crew_required: ship.crew.required,
    crew_capacity: ship.crew.capacity,
    crew_rotation: &ship.crew.rotation.to_string(),
    crew_moral: ship.crew.morale,
    crew_wages: ship.crew.wages,
    frame_symbol: &ship.frame.symbol.to_string(),
    frame_name: &ship.frame.name,
    frame_desc: &ship.frame.description,
    frame_condition: ship.frame.condition,
    frame_modules: ship.frame.module_slots,
    frame_mounts: ship.frame.mounting_points,
    frame_fuel_capacity: ship.frame.fuel_capacity,
    frame_req_power: ship.frame.requirements.power,
    frame_req_crew: ship.frame.requirements.crew,
    frame_req_slots: ship.frame.requirements.slots,
    reactor_symbol: &ship.reactor.symbol,
    reactor_name: &ship.reactor.name,
    reactor_desc: &ship.reactor.description,
    reactor_condition: ship.reactor.condition,
    reactor_power_output: ship.reactor.power_output,
    reactor_req_power: ship.reactor.requirements.power,
    reactor_req_crew: ship.reactor.requirements.crew,
    reactor_req_slots: ship.reactor.requirements.slots,
    engine_symbol: &ship.engine.symbol.to_string(),
    engine_name: &ship.engine.name,
    engine_desc: &ship.engine.description,
    engine_condition: ship.engine.condition,
    engine_speed: ship.engine.speed,
    engine_req_power: ship.engine.requirements.power,
    engine_req_crew: ship.engine.requirements.crew,
    engine_req_slots: ship.engine.requirements.slots,
    cargo_capacity: ship.cargo.capacity,
    cargo_units: ship.cargo.units,
    fuel_current: ship.fuel.current,
    fuel_capacity: ship.fuel.capacity,
    fuel_consumed_amount: ship.fuel.consumed.amount,
    fuel_consumed_timestamp: &ship.fuel.consumed.timestamp,
  };
  match replace_into(fleet::table)
    .values(&_ship)
    .execute(&mut connection) {
      Ok(_) => {
        insert_cargo_items(pool, &ship.symbol, &ship.cargo.inventory);
        insert_modules(pool, &ship.symbol, &ship.modules);
        insert_mounts(pool, &ship.symbol, &ship.mounts);
      }
      Err(err) => {
        warn!("{}", err);
      }
    };
}

pub fn get_cargo_items(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Vec<CargoItem> {
  use schema::fleet_cargo;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<CargoDB>, diesel::result::Error> = fleet_cargo::table
    .filter(fleet_cargo::ship_symbol.eq(ship_symbol))
    .select(CargoDB::as_select())
    .load(&mut connection);

  match result {
    Ok(r) => {
      let mut items: Vec<CargoItem> = vec![];
      for cargo in r {
        items.push(CargoItem {
          symbol: cargo.symbol,
          name: cargo.name,
          description: cargo.description,
          units: cargo.units
        });
      }
      items
    }
    Err(_err) => vec![]
  }
}

pub fn insert_cargo_items(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str, cargo_items: &Vec<CargoItem>) {
  use schema::fleet_cargo;
  let mut connection = pool.get().unwrap();
  delete(fleet_cargo::table)
    .filter(fleet_cargo::ship_symbol.eq(ship_symbol))
    .execute(&mut connection)
    .expect("Error deleting old ship cargo");
  for (_index, item) in cargo_items.iter().enumerate() {
    let cargo = NewCargoDB {
      // id: 1,
      ship_symbol: ship_symbol,
      symbol: &item.symbol,
      name: &item.name,
      description: &item.description,
      units: item.units,
    };
    insert_into(fleet_cargo::table)
      .values(cargo)
      .execute(&mut connection)
      .expect("Error saving ship cargo");
  }
}

pub fn get_modules(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Vec<Module> {
  use schema::fleet_modules;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<ModuleDB>, diesel::result::Error> = fleet_modules::table
    .filter(fleet_modules::ship_symbol.eq(ship_symbol))
    .select(ModuleDB::as_select())
    .load(&mut connection);

  match result {
    Ok(r) => {
      let mut modules: Vec<Module> = vec![];
      for module in r {
        modules.push(Module {
          symbol: ModuleType::from_str(&module.symbol).unwrap(),
          capacity: module.capacity,
          range: module.range,
          name: module.name,
          description: module.description,
          requirements: Requirements {
            crew: module.req_crew,
            power: module.req_power,
            slots: module.req_slots
          }
        });
      }
      modules
    }
    Err(_err) => vec![]
  }
}

pub fn insert_modules(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str, modules: &Vec<Module>) {
  use schema::fleet_modules;

  let mut connection = pool.get().unwrap();
  delete(fleet_modules::table)
    .filter(fleet_modules::ship_symbol.eq(ship_symbol))
    .execute(&mut connection)
    .expect("Error deleting old ship cargo");
  for (_index, item) in modules.iter().enumerate() {
    let module = NewModuleDB {
      ship_symbol,
      symbol: &item.symbol.to_string(),
      capacity: item.capacity,
      range: item.range,
      name: &item.name,
      description: &item.description,
      req_power: item.requirements.power,
      req_crew: item.requirements.crew,
      req_slots: item.requirements.slots,
    };
    insert_into(fleet_modules::table)
      .values(module)
      .execute(&mut connection)
      .expect("Error saving ship modules");
  }
}

pub fn get_mounts(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Vec<Mount> {
  use schema::fleet_mounts;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<MountDB>, diesel::result::Error> = fleet_mounts::table
    .filter(fleet_mounts::ship_symbol.eq(ship_symbol))
    .select(MountDB::as_select())
    .load(&mut connection);

  match result {
    Ok(r) => {
      let mut mounts: Vec<Mount> = vec![];
      for mount in r {
        mounts.push(Mount {
          symbol: MountType::from_str(&mount.symbol).unwrap(),
          name: mount.name,
          description: mount.description,
          strength: mount.strength,
          deposits: match mount.deposits {
            Some(d) => {
              let mut mount_deposits: Vec<DepositType> = vec![];
              for (_index, deposit) in d.split(",").filter(|&x| !x.is_empty()).enumerate() {
                mount_deposits.push(DepositType::from_str(&deposit).unwrap());
              }
              Some(mount_deposits)
            },
            None => None
          },
          requirements: Requirements {
            power: mount.req_power,
            crew: mount.req_crew,
            slots: mount.req_slots
          },
        });
      }
      mounts
    }
    Err(_err) => vec![]
  }
}

pub fn insert_mounts(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str, mounts: &Vec<Mount>) {
  use schema::fleet_mounts;
  
  let mut connection = pool.get().unwrap();
  delete(fleet_mounts::table)
    .filter(fleet_mounts::ship_symbol.eq(ship_symbol))
    .execute(&mut connection)
    .expect("Error deleting old ship cargo");
  for (_index, item) in mounts.iter().enumerate() {
    let mut _deposits = "".to_string();
    let mount = NewMountDB {
      ship_symbol,
      symbol: &item.symbol.to_string(),
      deposits: match &item.deposits {
        Some(d) => {
          for (i, deposit) in d.iter().enumerate() {
            if i < d.len() - 1 {
              _deposits = format!("{}{},", _deposits, deposit.to_string());
            } else {
              _deposits = format!("{}{}", _deposits, deposit.to_string());
            }
          }
          Some(&_deposits)
        }
        None => None
      },
      strength: item.strength,
      name: &item.name,
      description: &item.description,
      req_power: item.requirements.power,
      req_crew: item.requirements.crew,
      req_slots: item.requirements.slots,
    };
    insert_into(fleet_mounts::table)
      .values(mount)
      .execute(&mut connection)
      .expect("Error saving ship mounts");
  }
}
