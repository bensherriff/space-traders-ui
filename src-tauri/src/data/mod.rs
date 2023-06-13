use std::{fs::{create_dir_all}, path::{Path, PathBuf}, time::Duration};

use diesel::{r2d2::{Pool, ConnectionManager, CustomizeConnection}, connection::SimpleConnection};
use diesel::sqlite::SqliteConnection;
// use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub mod system;
pub mod models;
pub mod schema;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

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
  // TODO Use migrations to create database tables if they don't exist
  // let mut connection = pool.get().unwrap();
}

#[derive(Debug)]
pub struct ConnectionOptions {
  pub enable_wal: bool,
  pub enable_foreign_keys: bool,
  pub busy_timeout: Option<Duration>
}

impl CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
  fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
    (|| {
      if self.enable_wal {
        conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
      }
      if self.enable_foreign_keys {
        conn.batch_execute("PRAGMA foreign_keys = ON;")?;
      }
      if let Some(d) = self.busy_timeout {
        conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
      }
      Ok(())
    })()
    .map_err(diesel::r2d2::Error::QueryError)
  }
}

pub fn connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
  let db_path = &get_path_string();
  let manager = ConnectionManager::<SqliteConnection>::new(db_path);
  Pool::builder()
    .connection_customizer(Box::new(ConnectionOptions {
      enable_wal: true,
      enable_foreign_keys: true,
      busy_timeout: Some(Duration::from_secs(30))
    }))
    .build(manager)
    .unwrap()
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
