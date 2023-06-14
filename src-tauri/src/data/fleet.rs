use std::str::FromStr;

use crate::models::ship::*;

use diesel::prelude::*;
use diesel::{RunQueryDsl, QueryDsl, insert_or_ignore_into, SqliteConnection, r2d2::{Pool, ConnectionManager}};

pub fn get_ship(pool: &Pool<ConnectionManager<SqliteConnection>>, ship_symbol: &str) -> Option<Ship> {
  None
}
