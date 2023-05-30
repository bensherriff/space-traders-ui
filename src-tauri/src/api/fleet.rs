use crate::{configuration, models::{ship::{Ship, ShipTransactionResponse, ShipType, navigation::{NavigationResponse, ShipJumpResponse, ShipNavigateResponse, FlightMode, Navigation}, cargo::{CargoRefinement, ExtractedCargo, CargoItem, Cargo, CargoResponse}, cooldown::Cooldown, ShipScanResponse, fuel::RefuelResponse}, survey::{SurveyResponse, Survey}, transaction::{TransactionResponse, Transaction}, system::SystemScanResponse, waypoint::WaypointScanResponse, contract::Contract}};

use super::requests::{ResponseObject, handle_result, get_request, post_request, patch_request};


pub async fn list_ships(configuration: &configuration::Configuration) -> ResponseObject<Vec<Ship>> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships", local_configuration.base_url);
  handle_result(get_request::<Vec<Ship>>(configuration, &url, None).await)
}

pub async fn purchase_ship(configuration: &configuration::Configuration, ship_type: ShipType, waypoint: String) -> ResponseObject<ShipTransactionResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships", local_configuration.base_url);
  let body = serde_json::json!({
    "shipType": ship_type,
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<ShipTransactionResponse>(configuration, &url, Some(body.to_string())).await)
}

pub async fn get_ship(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<Ship> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}", local_configuration.base_url, symbol);
  handle_result(get_request::<Ship>(configuration, &url, None).await)
}

pub async fn get_cargo(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<Ship> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/cargo", local_configuration.base_url, symbol);
  handle_result(get_request::<Ship>(configuration, &url, None).await)
}

pub async fn orbit_ship(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<NavigationResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/orbit", local_configuration.base_url, symbol);
  handle_result(post_request::<NavigationResponse>(configuration, &url, None).await)
}

pub async fn refine(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<CargoRefinement> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/refine", local_configuration.base_url, symbol);
  handle_result(post_request::<CargoRefinement>(configuration, &url, None).await)
}

pub async fn create_chart(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<CargoRefinement> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/chart", local_configuration.base_url, symbol);
  handle_result(post_request::<CargoRefinement>(configuration, &url, None).await)
}

pub async fn get_cooldown(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<Cooldown> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/cooldown", local_configuration.base_url, symbol);
  handle_result(get_request::<Cooldown>(configuration, &url, None).await)
}

pub async fn dock_ship(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<NavigationResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/dock", local_configuration.base_url, symbol);
  handle_result(post_request::<NavigationResponse>(configuration, &url, None).await)
}

/// If you want to target specific yields for an extraction, you can survey a waypoint,
/// such as an asteroid field, and send the survey in the body of the extract request.
/// Each survey may have multiple deposits, and if a symbol shows up more than once, that
/// indicates a higher chance of extracting that resource.
///
/// Your ship will enter a cooldown between consecutive survey requests.
/// Surveys will eventually expire after a period of time.
/// Multiple ships can use the same survey for extraction.
pub async fn create_survey(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<SurveyResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/survey", local_configuration.base_url, symbol);
  handle_result(post_request::<SurveyResponse>(configuration, &url, None).await)
}

/// Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.
pub async fn extract_resources(configuration: &configuration::Configuration, symbol: String, survey: Option<Survey>) -> ResponseObject<ExtractedCargo> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/extract", local_configuration.base_url, symbol);
  match survey {
    None => handle_result(post_request::<ExtractedCargo>(configuration, &url, None).await),
    Some(s) => {
      let body = serde_json::json!({
        "survey": serde_json::json!(s)
      });
      handle_result(post_request::<ExtractedCargo>(configuration, &url, Some(body.to_string())).await)
    }
  }
}

/// Jettison cargo from your ship's cargo hold.
pub async fn jettison_cargo(configuration: &configuration::Configuration, symbol: String, item: CargoItem) -> ResponseObject<Cargo> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/jettison", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<Cargo>(configuration, &url, Some(body.to_string())).await)
}

/// Jump your ship instantly to a target system.
/// When used while in orbit or docked to a jump gate waypoint, any ship can use this command.
/// When used elsewhere, jumping requires a jump drive unit and consumes a unit of antimatter (which needs to be in your cargo).
pub async fn jump_ship(configuration: &configuration::Configuration, symbol: String, system: String) -> ResponseObject<ShipJumpResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/jump", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "systemSymbol": system
  });
  handle_result(post_request::<ShipJumpResponse>(configuration, &url, Some(body.to_string())).await)
}

/// Navigate to a target destination. The destination must be located within the same system as the ship.
/// Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
///
/// To travel between systems, see the ship's warp or jump actions.
pub async fn navigate_ship(configuration: &configuration::Configuration, symbol: String, waypoint: String) -> ResponseObject<ShipNavigateResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/navigate", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<ShipNavigateResponse>(configuration, &url, Some(body.to_string())).await)
}

/// Update the nav data of a ship, such as the flight mode.
pub async fn patch_ship_navigation(configuration: &configuration::Configuration, symbol: String, flight_mode: FlightMode) -> ResponseObject<Navigation> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/nav", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "flightMode": flight_mode
  });
  handle_result(patch_request::<Navigation>(configuration, &url, Some(body.to_string())).await)
}

/// Get the current nav status of a ship
pub async fn get_ship_nav(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<Navigation> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/nav", local_configuration.base_url, symbol);
  handle_result(get_request::<Navigation>(configuration, &url, None).await)
}

/// Warp your ship to a target destination in another system.
/// Warping will consume the necessary fuel and supplies from the ship's manifest,
/// and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
pub async fn warp_ship(configuration: &configuration::Configuration, symbol: String, waypoint: String) -> ResponseObject<Navigation> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/warp", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  handle_result(post_request::<Navigation>(configuration, &url, Some(body.to_string())).await)
}

/// Sell cargo.
pub async fn sell_cargo(configuration: &configuration::Configuration, symbol: String, item: CargoItem) -> ResponseObject<TransactionResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/sell", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<TransactionResponse>(configuration, &url, Some(body.to_string())).await)
}

/// Activate your ship's sensor arrays to scan for system information.
pub async fn scan_systems(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<SystemScanResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/scan/systems", local_configuration.base_url, symbol);
  handle_result(post_request::<SystemScanResponse>(configuration, &url, None).await)
}

/// Activate your ship's sensor arrays to scan for waypoint information.
pub async fn scan_waypoints(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<WaypointScanResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/scan/waypoints", local_configuration.base_url, symbol);
  handle_result(post_request::<WaypointScanResponse>(configuration, &url, None).await)
}

/// Activate your ship's sensor arrays to scan for ship information.
pub async fn scan_ships(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<ShipScanResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/scan/ships", local_configuration.base_url, symbol);
  handle_result(post_request::<ShipScanResponse>(configuration, &url, None).await)
}

/// Refuel your ship from the local market.
pub async fn refuel_ship(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<RefuelResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/refuel", local_configuration.base_url, symbol);
  handle_result(post_request::<RefuelResponse>(configuration, &url, None).await)
}

/// Purchase cargo.
pub async fn purchase_cargo(configuration: &configuration::Configuration, symbol: String, item: CargoItem) -> ResponseObject<TransactionResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/purchase", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  handle_result(post_request::<TransactionResponse>(configuration, &url, Some(body.to_string())).await)
}

/// Purchase cargo.
pub async fn transfer_cargo(configuration: &configuration::Configuration, symbol: String, transaction: Transaction) -> ResponseObject<CargoResponse> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/transfer", local_configuration.base_url, symbol);
  let body = serde_json::json!({
    "tradeSymbol": transaction.trade_symbol,
    "units": transaction.units,
    "shipSymbol": transaction.ship_symbol
  });
  handle_result(post_request::<CargoResponse>(configuration, &url, Some(body.to_string())).await)
}

pub async fn negotiate_contract(configuration: &configuration::Configuration, symbol: String) -> ResponseObject<Contract> {
  let local_configuration = configuration;
  let url = format!("{}/my/ships/{}/negotiate/contract", local_configuration.base_url, symbol);
  handle_result(post_request::<Contract>(configuration, &url, None).await)
}