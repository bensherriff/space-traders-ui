// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::RwLock, collections::HashMap};

use dotenv::dotenv;
use state::InitCell;

use api::requests::Request;
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use log::{error, info, LevelFilter};

use data::connection_pool;
use models::system::System;
use reqwest::Client;
use tauri_plugin_log::LogTarget;

mod api;
mod app;
mod data;
mod models;

pub struct DataState {
  pool: Pool<ConnectionManager<SqliteConnection>>,
  request: Request
}

pub static CACHE: InitCell<RwLock<Cache>> = InitCell::new();

pub struct Cache {
  pub systems: HashMap<String, System>,
  pub system_graph: petgraph::Graph<String, i32>
}

fn main() {
  dotenv().ok();

  let debug = match std::env::var("DEBUG") {
    Ok(val) => val == "true",
    Err(_) => false
  };
  if debug {
    println!("Debug mode enabled");
  }

  let state = DataState {
    pool: connection_pool(),
    request: Request {
      client: Client::new(),
      base_url: "https://api.spacetraders.io/v2".to_string(),
      max_attemps: 5
    }
  };

  let cache = Cache {
    systems: HashMap::new(),
    system_graph: petgraph::Graph::<String, i32>::new()
  };
  CACHE.set(RwLock::new(cache));

  match tauri::Builder::default()
    .plugin(tauri_plugin_log::Builder::default()
      .targets([
        LogTarget::Stdout,
        LogTarget::Webview
      ])
      .level_for("space_traders_ui", if debug { LevelFilter::Debug } else { LevelFilter::Info })
      .level(LevelFilter::Info)
      .build())
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_sql::Builder::default().build())
    .manage(state)
    .setup(|app| {
      data::init(&connection_pool(), app);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      api::agents::get_my_agent,
      api::contracts::list_contracts,
      api::contracts::get_contract,
      api::contracts::accept_contract,
      api::contracts::deliver_contract,
      api::contracts::fulfill_contract,
      api::factions::get_faction,
      api::factions::get_factions,
      api::factions::list_faction_strings,
      api::fleet::list_ships,
      api::fleet::purchase_ship,
      api::fleet::get_ship,
      api::fleet::get_ships_at_waypoint,
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
      api::fleet::navigate_ship_to_system,
      api::fleet::navigate_ship_anywhere,
      api::get_status,
      api::register,
      api::systems::list_systems,
      api::systems::list_all_systems,
      api::systems::get_system,
      api::systems::list_waypoints,
      api::systems::get_waypoint,
      api::systems::get_waypoints,
      api::systems::get_market,
      api::systems::get_shipyard,
      api::systems::get_jump_gate,
      app::auto_extract_resources,
      ])
    .run(tauri::generate_context!()) {
      Ok(_) => info!("App started"),
      Err(err) => error!("{}; Failed to start app", err)
    }
}