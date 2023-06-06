use std::fs::File;
use std::path::PathBuf;
use std::path::Path;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::system::System;

use super::data_dir;

diesel::table! {
  systems {
    id -> Integer,
    symbol -> Text
  }
}

pub fn init() {
  create_file();
  get_systems_connection();
}

pub fn create_file() {
  let db_path = get_path_string();
  let path = Path::new(&db_path);
  File::create(path).unwrap();
}

pub fn get_systems_connection() -> SqliteConnection {
  let db_path = &get_path_string();
  SqliteConnection::establish(db_path)
  .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn insert_into_systems(system: &System) {

  // insert_into(systems).default_values().execute(get_systems_connection());
}

pub fn get_path() -> PathBuf {
  Path::new(&data_dir()).join("systems.db")
}

pub fn get_path_string() -> String {
  match get_path().as_path().to_str() {
    Some(path) => path.to_string(),
    None => panic!("Error")
  }
}