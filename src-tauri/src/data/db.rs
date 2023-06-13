use crate::{models::system::System, data::{models::NewSystemDB, establish_connection}};

use diesel::RunQueryDsl;

pub fn insert_system(system: &System) {
  use crate::data::schema::systems;
  let connection = &mut establish_connection();
  let _system = NewSystemDB {
    symbol: &system.symbol,
    sector_symbol: &system.sector_symbol,
    system_type: &system.system_type.to_string(),
    x: system.x as i32,
    y: system.y as i32,
    waypoints: "",
    factions: ""
  };
  diesel::insert_or_ignore_into(systems::table)
    .values(&_system)
    .execute(connection)
    .expect("Error saving new system");
}