use crate::{configuration, models::{system::{System, JumpGate}, waypoint::Waypoint, market::Market, shipyard::Shipyard}};

use super::requests::{ResponseObject, get_request, handle_result};

/// Return a list of all systems.
pub async fn list_systems(configuration: &configuration::Configuration) -> ResponseObject<Vec<System>> {
  let local_configuration = configuration;
  let url = format!("{}/systems", local_configuration.base_url);
  handle_result(get_request::<Vec<System>>(configuration, &url, None).await)
}

/// Get the details of a system.
pub async fn get_system(configuration: &configuration::Configuration, system: String) -> ResponseObject<System> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}", local_configuration.base_url, system);
  handle_result(get_request::<System>(configuration, &url, None).await)
}

/// Return a list of all waypoints.
pub async fn list_waypoints(configuration: &configuration::Configuration, system: String) -> ResponseObject<Vec<Waypoint>> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}/waypoints", local_configuration.base_url, system);
  handle_result(get_request::<Vec<Waypoint>>(configuration, &url, None).await)
}

/// Get the details of a waypoint.
pub async fn get_waypoint(configuration: &configuration::Configuration, system: String, waypoint: String) -> ResponseObject<Waypoint> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}/waypoints/{}", local_configuration.base_url, system, waypoint);
  handle_result(get_request::<Waypoint>(configuration, &url, None).await)
}

/// Retrieve imports, exports and exchange data from a marketplace.
/// Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold.
/// Send a ship to the waypoint to access trade good prices and recent transactions.
pub async fn get_market(configuration: &configuration::Configuration, system: String, waypoint: String) -> ResponseObject<Market> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}/waypoints/{}/market", local_configuration.base_url, system, waypoint);
  handle_result(get_request::<Market>(configuration, &url, None).await)
}

/// Get the shipyard for a waypoint Send a ship to the waypoint to access ships that are
/// currently available for purchase and recent transactions.
pub async fn get_shipyard(configuration: &configuration::Configuration, system: String, waypoint: String) -> ResponseObject<Shipyard> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}/waypoints/{}/shipyard", local_configuration.base_url, system, waypoint);
  handle_result(get_request::<Shipyard>(configuration, &url, None).await)
}

/// Get jump gate details for a waypoint.
pub async fn get_jump_gate(configuration: &configuration::Configuration, system: String, waypoint: String) -> ResponseObject<JumpGate> {
  let local_configuration = configuration;
  let url = format!("{}/systems/{}/waypoints/{}/jump-gate", local_configuration.base_url, system, waypoint);
  handle_result(get_request::<JumpGate>(configuration, &url, None).await)
}