use std::{fs::{create_dir_all}, path::{Path, PathBuf}, time::Duration};

use diesel::{r2d2::{Pool, ConnectionManager, CustomizeConnection}, connection::SimpleConnection};
use diesel::sqlite::SqliteConnection;
use log::{warn, error};
use tauri::{App, api::path::local_data_dir};
use tauri_plugin_store::StoreBuilder;

pub mod fleet;
pub mod models;
pub mod schema;
pub mod system;

const DATA_DIR: &str = "data";
const DB_FILE: &str = "stu.db";
const UP_SQL_FILE: &str = "up.sql";
const STORE_FILE: &str = "store.json";
const IDENTIFIER: &str = "com.bensherriff.stu";

pub fn env_dir() -> PathBuf {
  std::env::current_dir().unwrap()
}

pub fn data_dir() -> PathBuf {
  match local_data_dir() {
    Some(p) => Path::new(&p).join(IDENTIFIER),
    None => {
      let path = Path::new(&env_dir()).join(DATA_DIR);
      if !path.exists() {
        match create_dir_all(&path) {
          Ok(_) => {},
          Err(err) => warn!("{}", err)
        }
      }
      path
    }
  }
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

  if !get_store_path().exists() {
    match std::fs::File::create(get_store_path()) {
      Ok(_) => {
        let mut _store = StoreBuilder::new(app.handle(), get_store_path()).build();
      },
      Err(err) => warn!("{}", err)
    }
  }
}

#[derive(Debug)]
pub struct ConnectionOptions {
  pub enable_wal: bool,
  pub enable_foreign_keys: bool,
  pub busy_timeout: Option<Duration>,
  pub default_cache_size: i32
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
      conn.batch_execute(&format!("PRAGMA default_cache_size = {};", self.default_cache_size))?;
      Ok(())
    })()
    .map_err(diesel::r2d2::Error::QueryError)
  }
}

pub fn connection_pool() -> Pool<ConnectionManager<SqliteConnection>> {
  let db_path = &get_db_path_string();
  let manager = ConnectionManager::<SqliteConnection>::new(db_path);
  Pool::builder()
    .connection_customizer(Box::new(ConnectionOptions {
      enable_wal: true,
      enable_foreign_keys: true,
      busy_timeout: Some(Duration::from_secs(30)),
      default_cache_size: 8000
    }))
    .build(manager)
    .unwrap()
}

pub fn get_db_path() -> PathBuf {
  Path::new(&data_dir()).join(DB_FILE)
}

pub fn get_db_path_string() -> String {
  match get_db_path().as_path().to_str() {
    Some(path) => path.to_string(),
    None => panic!("Error")
  }
}

pub fn get_store_path() -> PathBuf {
  Path::new(&data_dir()).join(STORE_FILE)
}

pub fn get_store_path_string() -> String {
  match get_store_path().as_path().to_str() {
    Some(path) => path.to_string(),
    None => panic!("Error")
  }
}