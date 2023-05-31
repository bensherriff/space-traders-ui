use crate::models::{ship::{Ship, ShipTransactionResponse, ShipType, navigation::{NavigationResponse, ShipJumpResponse, ShipNavigateResponse, FlightMode, Navigation}, cargo::{CargoRefinement, ExtractedCargo, CargoItem, Cargo, CargoResponse}, cooldown::Cooldown, ShipScanResponse, fuel::RefuelResponse}, survey::{SurveyResponse, Survey}, transaction::{TransactionResponse, Transaction}, system::SystemScanResponse, waypoint::WaypointScanResponse, contract::Contract};

use super::requests::{ResponseObject, handle_result, get_request, post_request, patch_request};

#[tauri::command]
pub async fn list_ships(token: String) -> ResponseObject<Vec<Ship>> {
  handle_result(get_request::<Vec<Ship>>(token, "/my/ships".to_string(), None).await)
}

#[tauri::command]
pub async fn purchase_ship(token: String, ship_type: ShipType, waypoint: String) -> ResponseObject<ShipTransactionResponse> {
  let body = serde_json::json!({
    "shipType": ship_type,
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<ShipTransactionResponse>(token, "/my/ships".to_string(), Some(body.to_string())).await)
}

#[tauri::command]
pub async fn get_ship(token: String, symbol: String) -> ResponseObject<Ship> {
  let url = format!("/my/ships/{}", symbol);
  handle_result(get_request::<Ship>(token, url, None).await)
}

#[tauri::command]
pub async fn get_cargo(token: String, symbol: String) -> ResponseObject<Ship> {
  let url = format!("/my/ships/{}/cargo", symbol);
  handle_result(get_request::<Ship>(token, url, None).await)
}

#[tauri::command]
pub async fn orbit_ship(token: String, symbol: String) -> ResponseObject<NavigationResponse> {
  let url = format!("/my/ships/{}/orbit", symbol);
  handle_result(post_request::<NavigationResponse>(token, url, None).await)
}

#[tauri::command]
pub async fn refine(token: String, symbol: String) -> ResponseObject<CargoRefinement> {
  let url = format!("/my/ships/{}/refine", symbol);
  handle_result(post_request::<CargoRefinement>(token, url, None).await)
}

#[tauri::command]
pub async fn create_chart(token: String, symbol: String) -> ResponseObject<CargoRefinement> {
  let url = format!("/my/ships/{}/chart", symbol);
  handle_result(post_request::<CargoRefinement>(token, url, None).await)
}

#[tauri::command]
pub async fn get_cooldown(token: String, symbol: String) -> ResponseObject<Cooldown> {
  let url = format!("/my/ships/{}/cooldown", symbol);
  handle_result(get_request::<Cooldown>(token, url, None).await)
}

#[tauri::command]
pub async fn dock_ship(token: String, symbol: String) -> ResponseObject<NavigationResponse> {
  let url = format!("/my/ships/{}/dock", symbol);
  handle_result(post_request::<NavigationResponse>(token, url, None).await)
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
pub async fn create_survey(token: String, symbol: String) -> ResponseObject<SurveyResponse> {
  let url = format!("/my/ships/{}/survey", symbol);
  handle_result(post_request::<SurveyResponse>(token, url, None).await)
}

/// Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.
#[tauri::command]
pub async fn extract_resources(token: String, symbol: String, survey: Option<Survey>) -> ResponseObject<ExtractedCargo> {
  let url = format!("/my/ships/{}/extract", symbol);
  match survey {
    None => handle_result(post_request::<ExtractedCargo>(token, url, None).await),
    Some(s) => {
      let body = serde_json::json!({
        "survey": serde_json::json!(s)
      });
      handle_result(post_request::<ExtractedCargo>(token, url, Some(body.to_string())).await)
    }
  }
}

/// Jettison cargo from your ship's cargo hold.
#[tauri::command]
pub async fn jettison_cargo(token: String, symbol: String, item: CargoItem) -> ResponseObject<Cargo> {
  let url = format!("/my/ships/{}/jettison", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<Cargo>(token, url, Some(body.to_string())).await)
}

/// Jump your ship instantly to a target system.
/// When used while in orbit or docked to a jump gate waypoint, any ship can use this command.
/// When used elsewhere, jumping requires a jump drive unit and consumes a unit of antimatter (which needs to be in your cargo).
#[tauri::command]
pub async fn jump_ship(token: String, symbol: String, system: String) -> ResponseObject<ShipJumpResponse> {
  let url = format!("/my/ships/{}/jump", symbol);
  let body = serde_json::json!({
    "systemSymbol": system
  });
  handle_result(post_request::<ShipJumpResponse>(token, url, Some(body.to_string())).await)
}

/// Navigate to a target destination. The destination must be located within the same system as the ship.
/// Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
///
/// To travel between systems, see the ship's warp or jump actions.
#[tauri::command]
pub async fn navigate_ship(token: String, symbol: String, waypoint: String) -> ResponseObject<ShipNavigateResponse> {
  let url = format!("/my/ships/{}/navigate", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<ShipNavigateResponse>(token, url, Some(body.to_string())).await)
}

/// Update the nav data of a ship, such as the flight mode.
#[tauri::command]
pub async fn patch_ship_navigation(token: String, symbol: String, flight_mode: FlightMode) -> ResponseObject<Navigation> {
  let url = format!("/my/ships/{}/nav", symbol);
  let body = serde_json::json!({
    "flightMode": flight_mode
  });
  handle_result(patch_request::<Navigation>(token, url, Some(body.to_string())).await)
}

/// Get the current nav status of a ship
#[tauri::command]
pub async fn get_ship_nav(token: String, symbol: String) -> ResponseObject<Navigation> {
  let url = format!("/my/ships/{}/nav", symbol);
  handle_result(get_request::<Navigation>(token, url, None).await)
}

/// Warp your ship to a target destination in another system.
/// Warping will consume the necessary fuel and supplies from the ship's manifest,
/// and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
#[tauri::command]
pub async fn warp_ship(token: String, symbol: String, waypoint: String) -> ResponseObject<Navigation> {
  let url = format!("/my/ships/{}/warp", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<Navigation>(token, url, Some(body.to_string())).await)
}

/// Sell cargo.
#[tauri::command]
pub async fn sell_cargo(token: String, symbol: String, item: CargoItem) -> ResponseObject<TransactionResponse> {
  let url = format!("/my/ships/{}/sell", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<TransactionResponse>(token, url, Some(body.to_string())).await)
}

/// Activate your ship's sensor arrays to scan for system information.
#[tauri::command]
pub async fn scan_systems(token: String, symbol: String) -> ResponseObject<SystemScanResponse> {
  let url = format!("/my/ships/{}/scan/systems", symbol);
  handle_result(post_request::<SystemScanResponse>(token, url, None).await)
}

/// Activate your ship's sensor arrays to scan for waypoint information.
#[tauri::command]
pub async fn scan_waypoints(token: String, symbol: String) -> ResponseObject<WaypointScanResponse> {
  let url = format!("/my/ships/{}/scan/waypoints", symbol);
  handle_result(post_request::<WaypointScanResponse>(token, url, None).await)
}

/// Activate your ship's sensor arrays to scan for ship information.
#[tauri::command]
pub async fn scan_ships(token: String, symbol: String) -> ResponseObject<ShipScanResponse> {
  let url = format!("/my/ships/{}/scan/ships", symbol);
  handle_result(post_request::<ShipScanResponse>(token, url, None).await)
}

/// Refuel your ship from the local market.
#[tauri::command]
pub async fn refuel_ship(token: String, symbol: String) -> ResponseObject<RefuelResponse> {
  let url = format!("/my/ships/{}/refuel", symbol);
  handle_result(post_request::<RefuelResponse>(token, url, None).await)
}

/// Purchase cargo.
#[tauri::command]
pub async fn purchase_cargo(token: String, symbol: String, item: CargoItem) -> ResponseObject<TransactionResponse> {
  let url = format!("/my/ships/{}/purchase", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<TransactionResponse>(token, url, Some(body.to_string())).await)
}

/// Purchase cargo.
#[tauri::command]
pub async fn transfer_cargo(token: String, symbol: String, transaction: Transaction) -> ResponseObject<CargoResponse> {
  let url = format!("/my/ships/{}/transfer", symbol);
  let body = serde_json::json!({
    "tradeSymbol": transaction.trade_symbol,
    "units": transaction.units,
    "shipSymbol": transaction.ship_symbol
  });
  handle_result(post_request::<CargoResponse>(token, url, Some(body.to_string())).await)
}

#[tauri::command]
pub async fn negotiate_contract(token: String, symbol: String) -> ResponseObject<Contract> {
  let url = format!("/my/ships/{}/negotiate/contract", symbol);
  handle_result(post_request::<Contract>(token, url, None).await)
}