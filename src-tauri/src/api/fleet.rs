use reqwest::Client;
use tauri::State;

use crate::models::{ship::{Ship, ShipTransactionResponse, ShipType, navigation::{NavigationResponse, ShipJumpResponse, ShipNavigateResponse, FlightMode, Navigation}, cargo::{CargoRefinement, ExtractedCargo, CargoItem, Cargo, CargoResponse}, cooldown::Cooldown, ShipScanResponse, fuel::RefuelResponse}, survey::{SurveyResponse, Survey}, transaction::{TransactionResponse, Transaction}, system::SystemScanResponse, waypoint::WaypointScanResponse, contract::Contract};

use super::requests::{ResponseObject, handle_result, get_request, post_request, patch_request};

#[tauri::command]
pub async fn list_ships(client: State<'_, Client>, token: String) -> Result<ResponseObject<Vec<Ship>>, ()> {
  let result = handle_result(get_request::<Vec<Ship>>(&client, token, "/my/ships".to_string(), None).await);
  Ok(result)
}

#[tauri::command]
pub async fn purchase_ship(client: State<'_, Client>, token: String, ship_type: ShipType, waypoint: String) -> Result<ResponseObject<ShipTransactionResponse>, ()> {
  let body = serde_json::json!({
    "shipType": ship_type,
    "waypointSymbol": waypoint
  });
  let result = handle_result(post_request::<ShipTransactionResponse>(&client, token, "/my/ships".to_string(), Some(body.to_string())).await);
  Ok(result)
}

#[tauri::command]
pub async fn get_ship(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<Ship>, ()> {
  let url = format!("/my/ships/{}", symbol);
  let result = handle_result(get_request::<Ship>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn get_cargo(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<Ship>, ()> {
  let url = format!("/my/ships/{}/cargo", symbol);
  let result = handle_result(get_request::<Ship>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn orbit_ship(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<NavigationResponse>, ()> {
  let url = format!("/my/ships/{}/orbit", symbol);
  let result = handle_result(post_request::<NavigationResponse>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn refine(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<CargoRefinement>, ()> {
  let url = format!("/my/ships/{}/refine", symbol);
  let result = handle_result(post_request::<CargoRefinement>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn create_chart(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<CargoRefinement>, ()> {
  let url = format!("/my/ships/{}/chart", symbol);
  let result = handle_result(post_request::<CargoRefinement>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn get_cooldown(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<Cooldown>, ()> {
  let url = format!("/my/ships/{}/cooldown", symbol);
  let result = handle_result(get_request::<Cooldown>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn dock_ship(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<NavigationResponse>, ()> {
  let url = format!("/my/ships/{}/dock", symbol);
  let result = handle_result(post_request::<NavigationResponse>(&client, token, url, None).await);
  Ok(result)
}

/// If you want to target specific yields for an extraction, you can survey a waypoint,
/// such as an asteroid field, and send the survey in the body of the extract request.
/// Each survey may have multiple deposits, and if a symbol shows up more than once, that
/// indicates a higher chance of extracting that resource.
///
/// Your ship will enter a cooldown between consecutive survey requests.
/// Surveys will eventually expire after a period of time.
/// Multiple ships can use the same survey for extraction.
#[tauri::command]
pub async fn create_survey(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<SurveyResponse>, ()> {
  let url = format!("/my/ships/{}/survey", symbol);
  let result = handle_result(post_request::<SurveyResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.
#[tauri::command]
pub async fn extract_resources(client: State<'_, Client>, token: String, symbol: String, survey: Option<Survey>) -> Result<ResponseObject<ExtractedCargo>, ()> {
  let url = format!("/my/ships/{}/extract", symbol);
  let result; 
  match survey {
    None => result = handle_result(post_request::<ExtractedCargo>(&client, token, url, None).await),
    Some(s) => {
      let body = serde_json::json!({
        "survey": serde_json::json!(s)
      });
      result = handle_result(post_request::<ExtractedCargo>(&client, token, url, Some(body.to_string())).await);
    }
  }
  Ok(result)
}

/// Jettison cargo from your ship's cargo hold.
#[tauri::command]
pub async fn jettison_cargo(client: State<'_, Client>, token: String, symbol: String, item: CargoItem) -> Result<ResponseObject<Cargo>, ()> {
  let url = format!("/my/ships/{}/jettison", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  let result = handle_result(post_request::<Cargo>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Jump your ship instantly to a target system.
/// When used while in orbit or docked to a jump gate waypoint, any ship can use this command.
/// When used elsewhere, jumping requires a jump drive unit and consumes a unit of antimatter (which needs to be in your cargo).
#[tauri::command]
pub async fn jump_ship(client: State<'_, Client>, token: String, symbol: String, system: String) -> Result<ResponseObject<ShipJumpResponse>, ()> {
  let url = format!("/my/ships/{}/jump", symbol);
  let body = serde_json::json!({
    "systemSymbol": system
  });
  let result = handle_result(post_request::<ShipJumpResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Navigate to a target destination. The destination must be located within the same system as the ship.
/// Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
///
/// To travel between systems, see the ship's warp or jump actions.
#[tauri::command]
pub async fn navigate_ship(client: State<'_, Client>, token: String, symbol: String, waypoint: String) -> Result<ResponseObject<ShipNavigateResponse>, ()> {
  let url = format!("/my/ships/{}/navigate", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  let result = handle_result(post_request::<ShipNavigateResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Update the nav data of a ship, such as the flight mode.
#[tauri::command]
pub async fn patch_ship_navigation(client: State<'_, Client>, token: String, symbol: String, flight_mode: FlightMode) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/nav", symbol);
  let body = serde_json::json!({
    "flightMode": flight_mode
  });
  let result = handle_result(patch_request::<Navigation>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Get the current nav status of a ship
#[tauri::command]
pub async fn get_ship_nav(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/nav", symbol);
  let result = handle_result(get_request::<Navigation>(&client, token, url, None).await);
  Ok(result)
}

/// Warp your ship to a target destination in another system.
/// Warping will consume the necessary fuel and supplies from the ship's manifest,
/// and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
#[tauri::command]
pub async fn warp_ship(client: State<'_, Client>, token: String, symbol: String, waypoint: String) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/warp", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  let result = handle_result(post_request::<Navigation>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Sell cargo.
#[tauri::command]
pub async fn sell_cargo(client: State<'_, Client>, token: String, symbol: String, item: CargoItem) -> Result<ResponseObject<TransactionResponse>, ()> {
  let url = format!("/my/ships/{}/sell", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  let result = handle_result(post_request::<TransactionResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for system information.
#[tauri::command]
pub async fn scan_systems(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<SystemScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/systems", symbol);
  let result = handle_result(post_request::<SystemScanResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for waypoint information.
#[tauri::command]
pub async fn scan_waypoints(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<WaypointScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/waypoints", symbol);
  let result = handle_result(post_request::<WaypointScanResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for ship information.
#[tauri::command]
pub async fn scan_ships(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<ShipScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/ships", symbol);
  let result = handle_result(post_request::<ShipScanResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Refuel your ship from the local market.
#[tauri::command]
pub async fn refuel_ship(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<RefuelResponse>, ()> {
  let url = format!("/my/ships/{}/refuel", symbol);
  let result = handle_result(post_request::<RefuelResponse>(&client, token, url, None).await);
  Ok(result)
}

/// Purchase cargo.
#[tauri::command]
pub async fn purchase_cargo(client: State<'_, Client>, token: String, symbol: String, item: CargoItem) -> Result<ResponseObject<TransactionResponse>, ()> {
  let url = format!("/my/ships/{}/purchase", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  let result = handle_result(post_request::<TransactionResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

/// Purchase cargo.
#[tauri::command]
pub async fn transfer_cargo(client: State<'_, Client>, token: String, symbol: String, transaction: Transaction) -> Result<ResponseObject<CargoResponse>, ()> {
  let url = format!("/my/ships/{}/transfer", symbol);
  let body = serde_json::json!({
    "tradeSymbol": transaction.trade_symbol,
    "units": transaction.units,
    "shipSymbol": transaction.ship_symbol
  });
  let result = handle_result(post_request::<CargoResponse>(&client, token, url, Some(body.to_string())).await);
  Ok(result)
}

#[tauri::command]
pub async fn negotiate_contract(client: State<'_, Client>, token: String, symbol: String) -> Result<ResponseObject<Contract>, ()> {
  let url = format!("/my/ships/{}/negotiate/contract", symbol);
  let result = handle_result(post_request::<Contract>(&client, token, url, None).await);
  Ok(result)
}