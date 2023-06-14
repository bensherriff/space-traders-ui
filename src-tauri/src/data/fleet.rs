use std::str::FromStr;

use crate::models::ship::*;
use crate::data::{models::{ShipDB, NewShipDB}, schema};
use crate::models::ship::cargo::Cargo;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

pub fn get_ship(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Option<Ship> {
  use schema::fleet;

  let mut connection = pool.get().unwrap();
  let result: Result<ShipDB, diesel::result::Error> = fleet::table
    .filter(fleet::ship_symbol.eq(ship_symbol))
    .select(ShipDB::as_select())
    .first(&mut connection);

  match result {
    Ok(r) => {

      None
    }
    Err(_err) => None
  }
}

pub fn get_cargo(pool: &Pool<ConnectionManager<SqliteConnection>>, cargo_id: &str) -> Option<Cargo> {
  use schema::fleet_cargo;
  
  None
}
