use std::{fs::{create_dir_all}, path::{Path, PathBuf}, time::Duration};

use diesel::{r2d2::{Pool, ConnectionManager, CustomizeConnection}, connection::SimpleConnection};
use diesel::sqlite::SqliteConnection;
use log::warn;
use tauri::App;

pub mod fleet;
pub mod models;
pub mod schema;
pub mod system;

const DB_FILE: &str = "stu.db";
const UP_SQL_FILE: &str = "up.sql";
const DATA_DIR: &str = "data";

pub fn dir() -> PathBuf {
  std::env::current_dir().unwrap()
}

pub fn data_dir() -> PathBuf {
  let path = Path::new(&dir()).join(DATA_DIR);
  if !path.exists() {
    match create_dir_all(&path) {
      Ok(_) => {},
      Err(err) => warn!("{}", err)
    }
  }
  path
}

pub fn init(pool: &Pool<ConnectionManager<SqliteConnection>>, app: &mut App) {
  let mut connection = pool.get().unwrap();
  let migrations_dir = app.path_resolver().resolve_resource("migrations").expect("Unable to find migrations");
  let migrations = std::fs::read_dir(migrations_dir).unwrap();
  for migration in migrations {
    if migration.as_ref().unwrap().file_type().unwrap().is_dir() {
      let migration_paths = std::fs::read_dir(&migration.unwrap().path()).unwrap();
      for migration_path in migration_paths {
        if migration_path.as_ref().unwrap().file_name().eq_ignore_ascii_case(UP_SQL_FILE) {
          let path = &migration_path.unwrap().path();
          let contents = std::fs::read_to_string(path).expect("Unable to read from file");
          match connection.batch_execute(&contents) {
            Ok(_) => {},
            Err(err) => {
              warn!("{:#?}", err);
            }
          };
        }
      }
    }
  }
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
  Path::new(&data_dir()).join(DB_FILE)
}

pub fn get_path_string() -> String {
  match get_path().as_path().to_str() {
    Some(path) => path.to_string(),
    None => panic!("Error")
  }
}
