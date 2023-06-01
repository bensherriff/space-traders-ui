// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::time;
use std::{thread, fs::{File}, io::{BufWriter, Write}, path::Path};

use async_recursion::async_recursion;

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
  const LIMIT: u64 = 20;

  let path  = match std::env::current_dir() {
    Ok(p) => p,
    Err(err) => {
      println!("{:#?}", err);
      return false;
    }
  };

  let file = match File::create(Path::new(&path).join("systems.json")) {
    Ok(file) => file,
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
      (meta.total as f64 / LIMIT as f64).ceil() as u64
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
  true
}

#[async_recursion]
async fn load_systems_db(file: &File, token: String, limit: u64, page: u64, attempts: u64) -> bool {
  let mut writer = BufWriter::new(file);
  let response: api::requests::ResponseObject<Vec<models::system::System>> = api::systems::list_systems(token.to_string(), limit, page).await;
  match response.data {
    Some(data) => {
      println!("Processing page {}", page);
      for (_index, system) in data.iter().enumerate() {
        let string = serde_json::json!(system).to_string();
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