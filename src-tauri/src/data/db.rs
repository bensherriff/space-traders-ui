use std::fs;
use std::path::{Path, PathBuf};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::data_dir;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn init() {
  if !db_file_exists() {
    create_db_file();
  }

  // run_migrations();
}

// pub fn establish_db_connection() -> SqliteConnection {
//   let db_path = get_db_path().clone();

//   SqliteConnection::establish
// }

fn create_db_file() {
  fs::File::create(get_db_path()).unwrap();
}

fn db_file_exists() -> bool {
  get_db_path().exists()
}

fn get_db_path() -> PathBuf {
  Path::new(&data_dir()).join("database.sqlite")
}