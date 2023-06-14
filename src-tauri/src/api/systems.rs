use diesel::{SqliteConnection, r2d2::{ConnectionManager, Pool}};
use reqwest::Client;
use tauri::State;

use crate::{models::{system::{System, JumpGate}, waypoint::Waypoint, market::Market, shipyard::Shipyard}, data::system::insert_system};

use super::requests::{ResponseObject, get_request, handle_result};

/// Return a list of all systems.
#[tauri::command]
pub async fn list_systems(client: State<'_, Client>, token: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<System>>, ()> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page)
  ];
  let result = handle_result(get_request::<Vec<System>>(&client, token, "/systems".to_string(), Some(query)).await);
  Ok(result)
}

/// Get the details of a system.
#[tauri::command]
pub async fn get_system(client: State<'_, Client>, pool: State<'_, Pool<ConnectionManager<SqliteConnection>>>, token: String, system: String) -> Result<ResponseObject<System>, ()> {
  match crate::data::system::get_system(&pool, &system) {
    Some(s) => {
      Ok(ResponseObject { data: Some(s), error: None, meta: None })
    },
    None => {
      let url = format!("/systems/{}", system);
      let result = handle_result(get_request::<System>(&client, token, url, None).await);
      match &result.data {
        Some(data) => {
          insert_system(&pool, data);
        }
        None => {}
      };
      Ok(result)
    }
  }
}

/// Return a list of all waypoints.
#[tauri::command]
pub async fn list_waypoints(client: State<'_, Client>, token: String, system: String) -> Result<ResponseObject<Vec<Waypoint>>, ()> {
  let url = format!("/systems/{}/waypoints", system);
  let result = handle_result(get_request::<Vec<Waypoint>>(&client, token, url, None).await);
  Ok(result)
}

/// Get the details of a waypoint.
#[tauri::command]
pub async fn get_waypoint(client: State<'_, Client>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Waypoint>, ()> {
  let url = format!("/systems/{}/waypoints/{}", system, waypoint);
  let result = handle_result(get_request::<Waypoint>(&client, token, url, None).await);
  Ok(result)
}

/// Retrieve imports, exports and exchange data from a marketplace.
/// Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold.
/// Send a ship to the waypoint to access trade good prices and recent transactions.
#[tauri::command]
pub async fn get_market(client: State<'_, Client>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Market>, ()> {
  let url = format!("/systems/{}/waypoints/{}/market", system, waypoint);
  let result = handle_result(get_request::<Market>(&client, token, url, None).await);
  Ok(result)
}

/// Get the shipyard for a waypoint Send a ship to the waypoint to access ships that are
/// currently available for purchase and recent transactions.
#[tauri::command]
pub async fn get_shipyard(client: State<'_, Client>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Shipyard>, ()> {
  let url = format!("/systems/{}/waypoints/{}/shipyard", system, waypoint);
  let result = handle_result(get_request::<Shipyard>(&client, token, url, None).await);
  Ok(result)
}

/// Get jump gate details for a waypoint.
#[tauri::command]
pub async fn get_jump_gate(client: State<'_, Client>, token: String, system: String, waypoint: String) -> Result<ResponseObject<JumpGate>, ()> {
  let url = format!("/systems/{}/waypoints/{}/jump-gate", system, waypoint);
  let result = handle_result(get_request::<JumpGate>(&client, token, url, None).await);
  Ok(result)
}