use std::{fs::{File, create_dir_all}, path::{Path, PathBuf}};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod db;
pub mod models;
pub mod schema;

pub fn dir() -> PathBuf {
  std::env::current_dir().unwrap()
}

pub fn data_dir() -> PathBuf {
  let path = Path::new(&dir()).join("data");
  if !path.exists() {
    create_dir_all(&path).unwrap();
  }
  path
}

pub fn init() {
  // create_file();
}

pub fn create_file() {
  let db_path = get_path_string();
  let path = Path::new(&db_path);
  File::create(path).unwrap();
}

pub fn establish_connection() -> SqliteConnection {
  let db_path = &get_path_string();
  SqliteConnection::establish(db_path)
  .unwrap_or_else(|_| panic!("Error connecting to {}", db_path))
}

pub fn get_path() -> PathBuf {
  Path::new(&data_dir()).join("sqlite.db")
}

pub fn get_path_string() -> String {
  match get_path().as_path().to_str() {
    Some(path) => path.to_string(),
    None => panic!("Error")
  }
}
