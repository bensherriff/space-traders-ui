// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use data::{Database};

mod api;
mod data;
mod models;

fn main() {
  tauri::Builder::default()
    .manage(Database::new())
    .invoke_handler(tauri::generate_handler![
      data::database_init,
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