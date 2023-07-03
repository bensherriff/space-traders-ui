use std::str::FromStr;

use crate::data::models::{CargoDB, ModuleDB, MountDB, NewCargoDB, NewModuleDB, NewMountDB, SurveyDB, NewSurveyDB};
use crate::models::ship::cooldown::Cooldown;
use crate::models::{ship::*, SymbolResponse};
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
use crate::models::size::Size;
use crate::models::survey::{Survey, SurveyResponse};
use crate::models::waypoint::WaypointType;

use chrono::{DateTime, Local};
use diesel::{prelude::*, insert_into, replace_into, delete, update};
use diesel::{RunQueryDsl, QueryDsl, SqliteConnection, r2d2::{Pool, ConnectionManager}};
use log::{error, debug};

pub fn get_ship(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Option<Ship> {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result: Result<ShipDB, diesel::result::Error> = fleet::table
    .filter(fleet::ship_symbol.eq(ship_symbol))
    .select(ShipDB::as_select())
    .first(&mut connection);

  match result {
    Ok(r) =>Some(build_ship_from_db(pool, &r)),
    Err(_err) => None
  }
}

pub fn get_ships(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Vec<Ship> {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<ShipDB>, diesel::result::Error> = fleet::table
    .select(ShipDB::as_select())
    .load(&mut connection);

  match result {
    Ok(r) => {
      let mut ships: Vec<Ship> = vec![];
      for (_index, ship_db) in r.iter().enumerate() {
        ships.push(build_ship_from_db(pool, ship_db));
      }
      ships
    },
    Err(_err) => vec![]
  }
}

pub fn get_ships_at_waypoint(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint: &str) -> Vec<Ship> {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<ShipDB>, diesel::result::Error> = fleet::table
    .filter(fleet::nav_waypoint_symbol.eq(waypoint))
    .select(ShipDB::as_select())
    .load(&mut connection);
  match result {
    Ok(r) => {
      let mut ships: Vec<Ship> = vec![];
      for (_index, ship_db) in r.iter().enumerate() {
        let ship = build_ship_from_db(pool, ship_db);
        if matches!(ship.nav.status, NavStatus::Docked | NavStatus::InOrbit) {
        // if matches!(ship.nav.status, NavStatus::Docked) {
          ships.push(ship);
        }
      }
      ships
    }
    Err(_err) => vec![]
  }
}

fn build_ship_from_db(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_db: &ShipDB) -> Ship {
  let ship_symbol = &ship_db.ship_symbol.to_string();
  let cargo_items = get_cargo_items(pool, &ship_symbol);
  let cargo = Cargo {
    capacity: ship_db.cargo_capacity,
    units: ship_db.cargo_units,
    inventory: cargo_items
  };
  Ship {
    symbol: (&ship_symbol).to_string(),
    registration: Registration {
      name: ship_db.reg_name.to_owned(),
      faction_symbol: ship_db.reg_faction_symbol.to_owned(),
      role: Role::from_str(&ship_db.reg_role).unwrap()
    },
    nav: Navigation {
      system_symbol: ship_db.nav_system_symbol.to_owned(),
      waypoint_symbol: ship_db.nav_waypoint_symbol.to_owned(),
      route: Route {
        destination: RouteWaypoint {
          symbol: ship_db.nav_dest_symbol.to_owned(),
          waypoint_type: WaypointType::from_str(&ship_db.nav_dest_waypoint_type).unwrap(),
          system_symbol: ship_db.nav_dest_system_symbol.to_owned(),
          x: ship_db.nav_dest_x,
          y: ship_db.nav_dest_y
        },
        departure: RouteWaypoint {
          symbol: ship_db.nav_dep_symbol.to_owned(),
          waypoint_type: WaypointType::from_str(&ship_db.nav_dep_waypoint_type).unwrap(),
          system_symbol: ship_db.nav_dep_system_symbol.to_owned(),
          x: ship_db.nav_dep_x,
          y: ship_db.nav_dep_y
        },
        departure_time: ship_db.nav_departure_time.to_owned(),
        arrival_time: ship_db.nav_arrival_time.to_owned()
      },
      status: NavStatus::from_str(&ship_db.nav_status).unwrap(),
      flight_mode: FlightMode::from_str(&ship_db.nav_flight_mode).unwrap()
    },
    crew: crew::Crew {
      current: ship_db.crew_current,
      required: ship_db.crew_required,
      capacity: ship_db.crew_capacity,
      rotation: Rotation::from_str(&ship_db.crew_rotation).unwrap(),
      morale: ship_db.crew_moral,
      wages: ship_db.crew_wages
    },
    frame: Frame {
    symbol: FrameType::from_str(&ship_db.frame_symbol).unwrap(),
    name: ship_db.frame_name.to_owned(),
    description: ship_db.frame_desc.to_owned(),
    condition: ship_db.frame_condition,
    module_slots: ship_db.frame_modules,
    mounting_points: ship_db.frame_mounts,
    fuel_capacity: ship_db.frame_fuel_capacity,
    requirements: Requirements { power: ship_db.frame_req_power, crew: ship_db.frame_req_crew, slots: ship_db.frame_req_slots }
  },
    reactor: Reactor {
      symbol: ship_db.reactor_symbol.to_owned(),
      name: ship_db.reactor_name.to_owned(),
      description: ship_db.reactor_desc.to_owned(),
      condition: ship_db.reactor_condition,
      power_output: ship_db.reactor_power_output,
      requirements: Requirements { power: ship_db.reactor_req_power, crew: ship_db.reactor_req_crew, slots: ship_db.reactor_req_slots }
    },
    engine: Engine {
      symbol: EngineType::from_str(&ship_db.engine_symbol).unwrap(),
      name: ship_db.engine_name.to_owned(),
      description: ship_db.engine_desc.to_owned(),
      condition: ship_db.engine_condition,
      speed: ship_db.engine_speed,
      requirements: Requirements { power: ship_db.engine_req_power, crew: ship_db.engine_req_power, slots: ship_db.engine_req_slots }
    },
    modules: get_modules(pool, &ship_symbol),
    mounts: get_mounts(pool, &ship_symbol),
    cargo,
    fuel: Fuel {
      current: ship_db.fuel_current,
      capacity: ship_db.fuel_capacity,
      consumed: Consumed { amount: ship_db.fuel_consumed_amount, timestamp: ship_db.fuel_consumed_timestamp.to_owned() }
    }
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
      Err(err) => error!("Error inserting ship: {}", err)
    };
}

pub fn update_ship_cargo(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str, cargo: &Cargo) {
  use schema::fleet_cargo;
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  match delete(fleet_cargo::table)
    .filter(fleet_cargo::ship_symbol.eq(ship_symbol))
    .execute(&mut connection) {
      Ok(_) => {},
      Err(err) => error!("Error deleting cargo: {}", err)
    };
  for item in cargo.inventory.iter() {
    let result = insert_into(fleet_cargo::table)
    .values(NewCargoDB {
        ship_symbol,
        symbol: &item.symbol,
        name: &item.name,
        description: &item.description,
        units: item.units,
    })
    .execute(&mut connection);
    match result {
      Ok(_) => {},
      Err(err) => error!("Error inserting cargo: {}", err)
    };
  }
  let result = update(fleet::table)
    .filter(fleet::ship_symbol.eq(ship_symbol))
    .set(fleet::cargo_units.eq(cargo.units))
    .execute(&mut connection);
  match result {
    Ok(_) => {},
    Err(err) => error!("Error updating cargo units: {}", err)
  };
}

pub fn update_ship_navigation(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str, navigation: &Navigation) {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result = update(fleet::table)
    .filter(fleet::ship_symbol.eq(ship_symbol))
    .set((
      fleet::nav_system_symbol.eq(&navigation.system_symbol),
      fleet::nav_status.eq(&navigation.status.to_string()),
      fleet::nav_flight_mode.eq(&navigation.flight_mode.to_string()),
      fleet::nav_waypoint_symbol.eq(&navigation.waypoint_symbol),
      fleet::nav_departure_time.eq(&navigation.route.departure_time),
      fleet::nav_arrival_time.eq(&navigation.route.arrival_time),
      fleet::nav_dest_symbol.eq(&navigation.route.destination.symbol),
      fleet::nav_dest_waypoint_type.eq(&navigation.route.destination.waypoint_type.to_string()),
      fleet::nav_dest_system_symbol.eq(&navigation.route.destination.system_symbol),
      fleet::nav_dest_x.eq(&navigation.route.destination.x),
      fleet::nav_dest_y.eq(&navigation.route.destination.y),
      fleet::nav_dep_symbol.eq(&navigation.route.departure.symbol),
      fleet::nav_dep_waypoint_type.eq(&navigation.route.departure.waypoint_type.to_string()),
      fleet::nav_dep_system_symbol.eq(&navigation.route.departure.system_symbol),
      fleet::nav_dep_x.eq(&navigation.route.departure.x),
      fleet::nav_dep_y.eq(&navigation.route.departure.y),
    ))
    .execute(&mut connection);
  match result {
    Ok(_) => {},
    Err(_err) => {}
  }
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

pub fn get_surveys(pool: &Pool<ConnectionManager<SqliteConnection>>, waypoint_symbol: &str) -> Option<SurveyResponse> {
  use schema::surveys;

  let mut connection = pool.get().unwrap();
  let result: Result<Vec<SurveyDB>, diesel::result::Error> = surveys::table
    .filter(surveys::waypoint_symbol.eq(waypoint_symbol))
    .select(SurveyDB::as_select())
    .load(&mut connection);

  match result {
    Ok(r) => {
      if r.is_empty() {
        return None;
      }
      let mut surveys: Vec<Survey> = vec![];
      let mut cooldown: Cooldown = Cooldown {
        ship_symbol: "".to_string(),
        total_seconds: 0,
        remaining_seconds: 0,
        expiration: Some("".to_string()),
      };
      for (_index, survey) in r.iter().enumerate() {
        let survey_expiration = DateTime::parse_from_rfc3339(&survey.expiration).unwrap().timestamp();
        // If the survey is expired, delete it
        if Local::now().timestamp() >= survey_expiration {
          debug!("Deleting expired survey: {}", survey.survey_signature);
          match delete(surveys::table)
            .filter(surveys::survey_signature.eq(survey.survey_signature.to_string()))
            .execute(&mut connection) {
              Ok(_) => {},
              Err(err) => error!("Error deleting expired survey: {}", err)
            };
          continue;
        }
        let mut deposits: Vec<SymbolResponse> = vec![];
        for (_index, deposit) in survey.deposits.split(",").filter(|&x| !x.is_empty()).enumerate() {
          deposits.push(SymbolResponse { symbol: deposit.to_string() });
        }
        surveys.push(Survey {
          signature: survey.survey_signature.to_string(),
          symbol: survey.waypoint_symbol.to_string(),
          deposits,
          expiration: survey.expiration.to_string(),
          size: Size::from_str(&survey.size).unwrap(),
        });
        cooldown.ship_symbol = survey.cooldown_ship_symbol.to_string();
        cooldown.total_seconds = survey.cooldown_total_seconds;
        cooldown.remaining_seconds = survey.cooldown_remaining_seconds;
        cooldown.expiration = survey.cooldown_expiration.to_owned();
      }
      if surveys.is_empty() {
        None
      } else {
        Some(SurveyResponse {
          surveys,
          cooldown,
        })
      }
    },
    Err(_err) => None
  }
}

pub fn insert_surveys(pool: &Pool<ConnectionManager<SqliteConnection>>, survey_response: &SurveyResponse) {
  use schema::surveys;
  for (_index, survey) in survey_response.surveys.iter().enumerate() {
    let mut _deposits = "".to_string();
    for (i, deposit) in survey.deposits.iter().enumerate() {
      if i < survey.deposits.len() - 1 {
        _deposits = format!("{}{},", _deposits, deposit.symbol.to_string());
      } else {
        _deposits = format!("{}{}", _deposits, deposit.symbol.to_string());
      }
    }
    let mut connection = pool.get().unwrap();
    let survey = NewSurveyDB {
      survey_signature: &survey.signature,
      waypoint_symbol: &survey.symbol,
      expiration: &survey.expiration,
      size: &survey.size.to_string(),
      deposits: &_deposits,
      cooldown_ship_symbol: &survey_response.cooldown.ship_symbol.to_string(),
      cooldown_total_seconds: survey_response.cooldown.total_seconds,
      cooldown_remaining_seconds: survey_response.cooldown.remaining_seconds,
      cooldown_expiration: match &survey_response.cooldown.expiration {
        Some(e) => Some(e),
        None => None
      },
    };
    insert_into(surveys::table)
      .values(survey)
      .execute(&mut connection)
      .expect("Error saving ship survey");  
  }
}

pub fn update_survey(pool: &Pool<ConnectionManager<SqliteConnection>>, survey_signature: &str, cooldown: &Cooldown) {
  use schema::surveys;
  let mut connection = pool.get().unwrap();
  update(surveys::table)
    .filter(surveys::survey_signature.eq(survey_signature))
    .set((
      surveys::cooldown_ship_symbol.eq(cooldown.ship_symbol.to_string()),
      surveys::cooldown_total_seconds.eq(cooldown.total_seconds),
      surveys::cooldown_remaining_seconds.eq(cooldown.remaining_seconds),
      surveys::cooldown_expiration.eq(cooldown.expiration.to_owned()),
    ))
    .execute(&mut connection)
    .expect("Error updating survey cooldown");
}
