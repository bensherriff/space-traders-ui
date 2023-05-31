use crate::{models::{system::{System, JumpGate}, waypoint::Waypoint, market::Market, shipyard::Shipyard}};

use super::requests::{ResponseObject, get_request, handle_result};

/// Return a list of all systems.
#[tauri::command]
pub async fn list_systems(token: String) -> ResponseObject<Vec<System>> {
  handle_result(get_request::<Vec<System>>(token, "/systems".to_string(), None).await)
}

/// Get the details of a system.
#[tauri::command]
pub async fn get_system(token: String, system: String) -> ResponseObject<System> {
  let url = format!("/systems/{}", system);
  handle_result(get_request::<System>(token, url, None).await)
}

/// Return a list of all waypoints.
#[tauri::command]
pub async fn list_waypoints(token: String, system: String) -> ResponseObject<Vec<Waypoint>> {
  let url = format!("/systems/{}/waypoints", system);
  handle_result(get_request::<Vec<Waypoint>>(token, url, None).await)
}

/// Get the details of a waypoint.
#[tauri::command]
pub async fn get_waypoint(token: String, system: String, waypoint: String) -> ResponseObject<Waypoint> {
  let url = format!("/systems/{}/waypoints/{}", system, waypoint);
  handle_result(get_request::<Waypoint>(token, url, None).await)
}

/// Retrieve imports, exports and exchange data from a marketplace.
/// Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold.
/// Send a ship to the waypoint to access trade good prices and recent transactions.
#[tauri::command]
pub async fn get_market(token: String, system: String, waypoint: String) -> ResponseObject<Market> {
  let url = format!("/systems/{}/waypoints/{}/market", system, waypoint);
  handle_result(get_request::<Market>(token, url, None).await)
}

/// Get the shipyard for a waypoint Send a ship to the waypoint to access ships that are
/// currently available for purchase and recent transactions.
#[tauri::command]
pub async fn get_shipyard(token: String, system: String, waypoint: String) -> ResponseObject<Shipyard> {
  let url = format!("/systems/{}/waypoints/{}/shipyard", system, waypoint);
  handle_result(get_request::<Shipyard>(token, url, None).await)
}

/// Get jump gate details for a waypoint.
#[tauri::command]
pub async fn get_jump_gate(token: String, system: String, waypoint: String) -> ResponseObject<JumpGate> {
  let url = format!("/systems/{}/waypoints/{}/jump-gate", system, waypoint);
  handle_result(get_request::<JumpGate>(token, url, None).await)
}