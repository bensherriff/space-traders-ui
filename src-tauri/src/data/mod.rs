use core::time;
use std::{thread, fs::{File, create_dir}, io::{BufWriter, Write}, path::Path, collections::HashMap, sync::Mutex};

use async_recursion::async_recursion;
use tauri::State;

use crate::{models::{system::System, waypoint::Waypoint, market::Market, shipyard::Shipyard}, api::{systems::list_systems, requests::{ResponseObject}}};

pub struct Database {
  systems: HashMap<String, System>,
  waypoints: HashMap<String, Waypoint>,
  markets: HashMap<String, Market>,
  shipyards: HashMap<String, Shipyard>
}

impl Database {
  pub fn new() -> Database {
    Database {
      systems: HashMap::new(),
      waypoints: HashMap::new(),
      markets: HashMap::new(),
      shipyards: HashMap::new()
    }
  }

  pub fn get_system(&self, key: String) -> ResponseObject<System>{
    if self.systems.contains_key(&key) {
      let system = self.systems.get(&key).unwrap();
      return ResponseObject { data: Some(system.to_owned()), error: None, meta: None }
    } else {
      return ResponseObject { data: None, error: None, meta: None }
    }
  }

  pub fn get_waypoint(&self, key: String) -> ResponseObject<Waypoint> {
    if self.waypoints.contains_key(&key) {
      let waypoint = self.waypoints.get(&key).unwrap();
      return ResponseObject { data: Some(waypoint.to_owned()), error: None, meta: None }
    } else {
      return ResponseObject { data: None, error: None, meta: None }
    }
  }
}

#[tauri::command]
pub async fn database_init(token: String) -> Result<bool, ()> {
  let app_path  = match std::env::current_dir() {
    Ok(p) => {
      match create_dir(Path::new(&p).join("data")) {
        Ok(_) => {},
        Err(_) => {}
      };
      p
    },
    Err(err) => {
      println!("{:#?}", err);
      return Ok(false);
    }
  };

  let systems_path = Path::new(&app_path).join("data").join("systems.json");
  if !systems_path.exists() {
    if write_systems_file(token, &systems_path).await {
      Ok(true)
    } else {
        Ok(false)
    }
  } else {
    Ok(true)
  }
}

async fn write_systems_file(token: String, systems_path: &Path) -> bool {
  const LIMIT: u64 = 20;
  let mut file = match File::create(systems_path) {
    Ok(mut file) => {
      match file.write_all(b"[") {
        Ok(_) => {},
        Err(err) => {
          println!("{:#?}", err);
          return false;
        }
      }
      file
    },
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  };

  // Determine the max pages
  let _response = list_systems(token.to_string(), LIMIT, 1).await;
  let max_pages: u64 = match _response.meta {
    Some(meta) => {
      println!("{:#?}", meta);
      (meta.total as f64 / LIMIT as f64).ceil() as u64 + 1
    },
    None => 1
  };

  // Store system results
  for page in 1..max_pages {
    let result = load_systems_db(&file, token.to_string(), LIMIT, page, 3).await;
    if !result {
      return false;
    }
  };

  match file.write_all(b"]") {
    Ok(_) => {},
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  }
  true
}

#[async_recursion]
async fn load_systems_db(file: &File, token: String, limit: u64, page: u64, attempts: u64) -> bool {
  let mut writer = BufWriter::new(file);
  let response: ResponseObject<Vec<System>> = list_systems(token.to_string(), limit, page).await;
  match response.data {
    Some(data) => {
      println!("Processing page {} ({} results)", page, data.len());
      for (index, system) in data.iter().enumerate() {
        let string = if index == 0 && page > 1 && index < data.len() - 1 {
          format!(",{},", serde_json::json!(system).to_string())
        } else if index == 0 && page > 1 {
          format!(",{}", serde_json::json!(system).to_string())
        } else if index < data.len() - 1 {
          format!("{},", serde_json::json!(system).to_string())
        } else {
          format!("{}", serde_json::json!(system).to_string())
        };
        match write!(writer, "{}", string) {
          Ok(_) => {},
          Err(err) => {
            println!("{:#?}", err);
            return false;
          }
        }
      }
      return true;
    }
    None => {
      match response.error {
        Some(error) => {
          if error.code == 429 && attempts > 0 {
            thread::sleep(time::Duration::from_secs(1));
            return load_systems_db(file, token, limit, page, attempts - 1).await
          } else {
            return false;
          }
        }
        None => {
          return false;
        }
      }
    }
  }
}