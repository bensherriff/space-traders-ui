// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::time;
use std::{thread, fs::{File, create_dir}, io::{BufWriter, Write, BufReader, BufRead}, path::Path};

use async_recursion::async_recursion;
use models::system::System;

mod api;
mod models;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      database_init,
      api::agents::get_my_agent,
      api::contracts::list_contracts,
      api::contracts::get_contract,
      api::contracts::accept_contract,
      api::contracts::deliver_contract,
      api::contracts::fulfill_contract,
      api::factions::get_faction,
      api::factions::get_factions,
      api::fleet::list_ships,
      api::fleet::purchase_ship,
      api::fleet::get_ship,
      api::fleet::get_cargo,
      api::fleet::orbit_ship,
      api::fleet::refine,
      api::fleet::create_chart,
      api::fleet::get_cooldown,
      api::fleet::dock_ship,
      api::fleet::create_survey,
      api::fleet::extract_resources,
      api::fleet::jettison_cargo,
      api::fleet::jump_ship,
      api::fleet::navigate_ship,
      api::fleet::patch_ship_navigation,
      api::fleet::get_ship_nav,
      api::fleet::warp_ship,
      api::fleet::sell_cargo,
      api::fleet::scan_systems,
      api::fleet::scan_waypoints,
      api::fleet::scan_ships,
      api::fleet::refuel_ship,
      api::fleet::purchase_cargo,
      api::fleet::transfer_cargo,
      api::fleet::negotiate_contract,
      api::get_status,
      api::register,
      api::systems::list_systems,
      api::systems::get_system,
      api::systems::list_waypoints,
      api::systems::get_waypoint,
      api::systems::get_market,
      api::systems::get_shipyard,
      api::systems::get_jump_gate
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn database_init(token: String) -> bool {
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
      return false;
    }
  };

  let systems_path = Path::new(&app_path).join("data").join("systems.json");
  if !systems_path.exists() {
    if write_systems_file(token, &systems_path).await {
      load_database(&systems_path)
    } else {
        false
    }
  } else {
    load_database(&systems_path)
  }
}

fn load_database(systems_path: &Path) -> bool {
  let systems_db = match sled::open("/db/systems") {
    Ok(db) => db,
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  };

  let mut contents = "".to_string();
  let file = match File::open(systems_path) {
    Ok(f) => f,
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  };
  let reader = BufReader::new(file);

  for line in reader.lines() {
    let parsed_line = match line {
      Ok(_line) => _line,
      Err(err) => {
        println!("{:#?}", err);
        return false;
      }
    };
    contents = format!("{}{}", contents, parsed_line);
  }

  let systems = match serde_json::from_str::<Vec<System>>(contents.as_str()) {
    Ok(_systems) => _systems,
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  };

  for system in systems {
    match systems_db.insert(system.symbol, "") {
      Ok(_) => {},
      Err(err) => {
        println!("{:#?}", err);
        return false;
      }
    };
  }
  true
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
  let _response = api::systems::list_systems(token.to_string(), LIMIT, 1).await;
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
  let response: api::requests::ResponseObject<Vec<models::system::System>> = api::systems::list_systems(token.to_string(), limit, page).await;
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