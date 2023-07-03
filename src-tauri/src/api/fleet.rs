use std::{thread, time::Duration};

use log::{debug, warn};
use tauri::State;

use crate::{models::{ship::{Ship, ShipTransactionResponse, ShipType, navigation::{NavigationResponse, ShipJumpResponse, ShipNavigateResponse, FlightMode, Navigation, NavStatus}, cargo::{CargoRefinement, ExtractedCargo, CargoItem, Cargo, CargoResponse}, cooldown::Cooldown, ShipScanResponse, fuel::RefuelResponse}, survey::{SurveyResponse, Survey}, transaction::{TransactionResponse, Transaction}, system::SystemScanResponse, waypoint::{WaypointScanResponse, WaypointType}, contract::Contract, chart::ChartResponse}, data::{fleet::insert_ship}, DataState, api::calculate_timeout};

use super::requests::{ResponseObject, ErrorObject};

#[tauri::command]
pub async fn list_ships(state: State<'_, DataState>, token: String) -> Result<ResponseObject<Vec<Ship>>, ()> {
    let ships = crate::data::fleet::get_ships(&state.pool);
    if ships.len() > 0 {
      return Ok(ResponseObject { data: Some(ships), error: None, meta: None })
    } else {
      let result = state.request.get_request::<Vec<Ship>>(token, "/my/ships".to_string(), None).await;
      match &result.data {
        Some(data) => {
          for ship in data.iter() {
            insert_ship(&state.pool, ship)
          }
        },
        None => {}
      };
      Ok(result)
    }
}

#[tauri::command]
pub async fn purchase_ship(state: State<'_, DataState>, token: String, ship_type: ShipType, waypoint: String) -> Result<ResponseObject<ShipTransactionResponse>, ()> {
  let body = serde_json::json!({
    "shipType": ship_type,
    "waypointSymbol": waypoint
  });
  let result = state.request.post_request::<ShipTransactionResponse>(token, "/my/ships".to_string(), Some(body.to_string())).await;
  Ok(result)
}

#[tauri::command]
pub async fn get_ship(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<Ship>, ()> {
  match crate::data::fleet::get_ship(&state.pool, &symbol) {
    Some(s) => {
      Ok(ResponseObject { data: Some(s), error: None, meta: None })
    }
    None => {
      let url = format!("/my/ships/{}", symbol);
      let result = state.request.get_request::<Ship>(token, url, None).await;
      match &result.data {
        Some(data) => insert_ship(&state.pool, data),
        None => {}
      };
      Ok(result)
    }
  }
}

#[tauri::command]
pub fn get_ships_at_waypoint(state: State<'_, DataState>, waypoint: String) -> Result<ResponseObject<Vec<Ship>>, ()> {
  let ships = crate::data::fleet::get_ships_at_waypoint(&state.pool, &waypoint);
  Ok(ResponseObject { data: Some(ships), error: None, meta: None })
}

#[tauri::command]
pub async fn get_cargo(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<Cargo>, ()> {
  let url = format!("/my/ships/{}/cargo", symbol);
  let result = state.request.get_request::<Cargo>(token, url, None).await;
  match &result.data {
    Some(d) => crate::data::fleet::update_ship_cargo(&state.pool, &symbol, &d),
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn orbit_ship(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<NavigationResponse>, ()> {
  let url = format!("/my/ships/{}/orbit", symbol);
  let result = state.request.post_request::<NavigationResponse>(token, url, None).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data.nav),
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn refine(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<CargoRefinement>, ()> {
  let url = format!("/my/ships/{}/refine", symbol);
  let result = state.request.post_request::<CargoRefinement>(token, url, None).await;
  Ok(result)
}

#[tauri::command]
pub async fn create_chart(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<ChartResponse>, ()> {
  let url = format!("/my/ships/{}/chart", symbol);
  let result = state.request.post_request::<ChartResponse>(token, url, None).await;
  Ok(result)
}

#[tauri::command]
pub async fn get_cooldown(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<Cooldown>, ()> {
  let url = format!("/my/ships/{}/cooldown", symbol);
  let result = state.request.get_request::<Cooldown>(token, url, None).await;
  Ok(result)
}

#[tauri::command]
pub async fn dock_ship(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<NavigationResponse>, ()> {
  let url = format!("/my/ships/{}/dock", symbol);
  let result = state.request.post_request::<NavigationResponse>(token, url, None).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data.nav),
    None => {}
  };
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
pub async fn create_survey(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<SurveyResponse>, ()> {
  match crate::data::fleet::get_surveys(&state.pool, &symbol) {
    Some(s) => {
      Ok(ResponseObject { data: Some(s), error: None, meta: None })
    }
    None => {
      let url = format!("/my/ships/{}/survey", symbol);
      let result = state.request.post_request::<SurveyResponse>(token, url, None).await;
      match &result.data {
        Some(data) => crate::data::fleet::insert_surveys(&state.pool, &data),
        None => {}
      };
      Ok(result)
    }
  }
}

/// Extract resources from the waypoint into your ship. Send an optional survey as the payload to target specific yields.
#[tauri::command]
pub async fn extract_resources(state: State<'_, DataState>, token: String, symbol: String, survey: Option<Survey>) -> Result<ResponseObject<ExtractedCargo>, ()> {
  let url = format!("/my/ships/{}/extract", symbol);
  let result; 
  match survey {
    None => result = state.request.post_request::<ExtractedCargo>(token, url, None).await,
    Some(s) => {
      let body = serde_json::json!({
        "survey": serde_json::json!(s)
      });
      result = state.request.post_request::<ExtractedCargo>(token, url, Some(body.to_string())).await;
      match &result.data {
        Some(data) => crate::data::fleet::update_ship_cargo(&state.pool, &symbol, &data.cargo),
        None => {}
      };
    }
  }
  Ok(result)
}

/// Jettison cargo from your ship's cargo hold.
#[tauri::command]
pub async fn jettison_cargo(state: State<'_, DataState>, token: String, symbol: String, item: CargoItem) -> Result<ResponseObject<Cargo>, ()> {
  let url = format!("/my/ships/{}/jettison", symbol);
  let body = serde_json::json!({
    "symbol": item.symbol,
    "units": item.units
  });
  let result = state.request.post_request::<Cargo>(token, url, Some(body.to_string())).await;
  Ok(result)
}

/// Jump your ship instantly to a target system.
/// When used while in orbit or docked to a jump gate waypoint, any ship can use this command.
/// When used elsewhere, jumping requires a jump drive unit and consumes a unit of antimatter (which needs to be in your cargo).
#[tauri::command]
pub async fn jump_ship(state: State<'_, DataState>, token: String, symbol: String, system: String, use_jump_drive: bool) -> Result<ResponseObject<ShipJumpResponse>, ()> {
  async fn internal_jump(state: State<'_, DataState>, token: String, symbol: String, system: String) -> Result<ResponseObject<ShipJumpResponse>, ()> {
    let url = format!("/my/ships/{}/jump", symbol);
    let body = serde_json::json!({
      "systemSymbol": system
    });
    let result = state.request.post_request::<ShipJumpResponse>(token, url, Some(body.to_string())).await;
    match &result.data {
      Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data.nav),
      None => {}
    };
    return Ok(result);
  }

  let s = get_ship(state.to_owned(), token.to_owned(), symbol.to_owned()).await.unwrap();

  match &s.data {
    Some(ship) => {
      if ship.nav.system_symbol == system {
        return Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 0, message: "Ship already at location".to_string() }), meta: None });
      }
      if use_jump_drive {
        return internal_jump(state.to_owned(), token.to_owned(), symbol.to_owned(), system.to_owned()).await;
      } else {
        let s = crate::api::systems::get_system(state.to_owned(), token.to_owned(), ship.nav.system_symbol.to_string()).await.unwrap();
        match &s.data {
          Some(ship_system) => {
            for waypoint in ship_system.waypoints.iter() {
              if matches!(waypoint.waypoint_type, WaypointType::JumpGate) {
                // Navigate to jump gate waypoint if necessary and jump
                if ship.nav.waypoint_symbol.to_string() == waypoint.symbol.to_owned() {
                  if matches!(ship.nav.status, NavStatus::Docked) {
                    let r = orbit_ship(state.to_owned(), token.to_owned(), symbol.to_owned()).await.unwrap();
                    match &r.data {
                      Some(_) => {},
                      None => {}
                    }
                  }
                  return internal_jump(state.to_owned(), token.to_owned(), symbol.to_owned(), system.to_owned()).await;
                } else {
                  let n = navigate_ship(state.to_owned(), token.to_owned(), ship.symbol.to_owned(), waypoint.symbol.to_owned()).await.unwrap();
                  match &n.data {
                    Some(response) => {
                      crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &response.nav);
                      let timeout = calculate_timeout(response.nav.route.departure_time.to_string(), response.nav.route.arrival_time.to_string());
                      debug!("Waiting for navigate arrival: {}", timeout);
                      thread::sleep(Duration::from_secs(timeout as u64));
                      return internal_jump(state.to_owned(), token.to_owned(), symbol.to_owned(), system.to_owned()).await;
                    }
                    None => {}
                  };
                }
              }
            }
          }
          None => {}
        };
      }
    }
    None => {}
  }
  Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 0, message: "Unable to jump ship".to_string() }), meta: None })
}

/// Navigate to a target destination. The destination must be located within the same system as the ship.
/// Navigating will consume the necessary fuel and supplies from the ship's manifest, and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
///
/// To travel between systems, see the ship's warp or jump actions.
#[tauri::command]
pub async fn navigate_ship(state: State<'_, DataState>, token: String, symbol: String, waypoint: String) -> Result<ResponseObject<ShipNavigateResponse>, ()> {
  let s = get_ship(state.to_owned(), token.to_owned(), symbol.to_owned()).await.unwrap();
  match &s.data {
    Some(ship) => {
      if matches!(ship.nav.status, NavStatus::Docked) {
        orbit_ship(state.to_owned(), token.to_owned(), symbol.to_owned()).await.unwrap();
      }
    }
    None => {}
  }
  let url = format!("/my/ships/{}/navigate", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  let result = state.request.post_request::<ShipNavigateResponse>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data.nav),
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn navigate_ship_anywhere(state: State<'_, DataState>, token: String, symbol: String, waypoint: String, system: String) -> Result<ResponseObject<ShipNavigateResponse>, ()> {
  debug!("Navigating ship {} to {} in system {}", symbol, waypoint, system);
  let s = get_ship(state.to_owned(), token.to_owned(), symbol.to_owned()).await.unwrap();
  match &s.data {
    Some(ship) => {
      let current_system = ship.nav.system_symbol.to_string();
      if current_system == system {
        return navigate_ship(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned()).await;
      } else {
        let response = navigate_ship_to_system(state.to_owned(), token.to_owned(), symbol.to_owned(), current_system.to_owned(), system.to_owned()).await.unwrap();
        if ship.nav.waypoint_symbol.to_string() == waypoint {
          return Ok(response);
        } else {
          return navigate_ship(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned()).await;
        }
      }
    }
    None => Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 0, message: "Ship not found".to_string() }), meta: None })
  }
}

/// Update the nav data of a ship, such as the flight mode.
#[tauri::command]
pub async fn patch_ship_navigation(state: State<'_, DataState>, token: String, symbol: String, flight_mode: FlightMode) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/nav", symbol);
  let body = serde_json::json!({
    "flightMode": flight_mode
  });
  let result = state.request.patch_request::<Navigation>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data),
    None => {}
  };
  Ok(result)
}

/// Get the current nav status of a ship
#[tauri::command]
pub async fn get_ship_nav(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/nav", symbol);
  let result = state.request.get_request::<Navigation>(token, url, None).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data),
    None => {}
  };
  Ok(result)
}

/// Warp your ship to a target destination in another system.
/// Warping will consume the necessary fuel and supplies from the ship's manifest,
/// and will pay out crew wages from the agent's account.
///
/// The returned response will detail the route information including the expected time of arrival.
/// Most ship actions are unavailable until the ship has arrived at it's destination.
#[tauri::command]
pub async fn warp_ship(state: State<'_, DataState>, token: String, symbol: String, waypoint: String) -> Result<ResponseObject<Navigation>, ()> {
  let url = format!("/my/ships/{}/warp", symbol);
  let body = serde_json::json!({
    "waypointSymbol": waypoint
  });
  let result = state.request.post_request::<Navigation>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(data) => crate::data::fleet::update_ship_navigation(&state.pool, &symbol, &data),
    None => {}
  };
  Ok(result)
}

/// Sell cargo.
#[tauri::command]
pub async fn sell_cargo(state: State<'_, DataState>, token: String, symbol: String, item_symbol: String, units: i32) -> Result<ResponseObject<TransactionResponse>, ()> {
  let url = format!("/my/ships/{}/sell", symbol);
  let body = serde_json::json!({
    "symbol": item_symbol,
    "units": units
  });
  let result = state.request.post_request::<TransactionResponse>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(d) => crate::data::fleet::update_ship_cargo(&state.pool, &symbol, &d.cargo),
    None => {}
  };
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for system information.
#[tauri::command]
pub async fn scan_systems(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<SystemScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/systems", symbol);
  let result = state.request.post_request::<SystemScanResponse>(token, url, None).await;
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for waypoint information.
#[tauri::command]
pub async fn scan_waypoints(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<WaypointScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/waypoints", symbol);
  let result = state.request.post_request::<WaypointScanResponse>(token, url, None).await;
  Ok(result)
}

/// Activate your ship's sensor arrays to scan for ship information.
#[tauri::command]
pub async fn scan_ships(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<ShipScanResponse>, ()> {
  let url = format!("/my/ships/{}/scan/ships", symbol);
  let result = state.request.post_request::<ShipScanResponse>(token, url, None).await;
  Ok(result)
}

/// Refuel your ship from the local market.
#[tauri::command]
pub async fn refuel_ship(state: State<'_, DataState>, token: String, symbol: String, units: Option<i32>) -> Result<ResponseObject<RefuelResponse>, ()> {
  let url = format!("/my/ships/{}/refuel", symbol);
  let body = match units {
    Some(u) => Some(serde_json::json!({ "units": u }).to_string()),
    None => None
  };
  let result = state.request.post_request::<RefuelResponse>(token, url, body).await;
  Ok(result)
}

/// Purchase cargo.
#[tauri::command]
pub async fn purchase_cargo(state: State<'_, DataState>, token: String, symbol: String, item_symbol: String, units: i32) -> Result<ResponseObject<TransactionResponse>, ()> {
  let url = format!("/my/ships/{}/purchase", symbol);
  let body = serde_json::json!({
    "symbol": item_symbol,
    "units": units
  });
  let result = state.request.post_request::<TransactionResponse>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(d) => crate::data::fleet::update_ship_cargo(&state.pool, &symbol, &d.cargo),
    None => {}
  };
  Ok(result)
}

/// Purchase cargo.
#[tauri::command]
pub async fn transfer_cargo(state: State<'_, DataState>, token: String, symbol: String, transaction: Transaction) -> Result<ResponseObject<CargoResponse>, ()> {
  let url = format!("/my/ships/{}/transfer", symbol);
  let body = serde_json::json!({
    "tradeSymbol": transaction.trade_symbol,
    "units": transaction.units,
    "shipSymbol": transaction.ship_symbol
  });
  let result = state.request.post_request::<CargoResponse>(token, url, Some(body.to_string())).await;
  match &result.data {
    Some(d) => crate::data::fleet::update_ship_cargo(&state.pool, &symbol, &d.cargo),
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn negotiate_contract(state: State<'_, DataState>, token: String, symbol: String) -> Result<ResponseObject<Contract>, ()> {
  let url = format!("/my/ships/{}/negotiate/contract", symbol);
  let result = state.request.post_request::<Contract>(token, url, None).await;
  Ok(result)
}

#[tauri::command]
pub async fn navigate_ship_to_system(state: State<'_, DataState>, token: String, symbol: String, start_system: String, end_system: String) -> Result<ResponseObject<ShipNavigateResponse>, ()> {
  let p = crate::app::get_path_to_system(state.to_owned(), token.to_owned(), start_system.to_owned(), end_system.to_owned()).await.unwrap();
  match &p.data {
    Some(system_path) => {
      let mut path: Vec<String> = system_path.to_owned();
      debug!("Found path to {}: {:?}", symbol, path);
      path.reverse(); // Reverse the path with the next system at the end
      let _ = path.pop(); // Remove the current system from the path
      while !path.is_empty() {
        let r = jump_ship(state.to_owned(), token.to_owned(), symbol.to_owned(), path.pop().unwrap(), false).await.unwrap();
        match &r.data {
          Some(response) => {
            debug!("Jumped to system: {}; cooldown: {} seconds", response.nav.system_symbol, response.cooldown.total_seconds);
            thread::sleep(Duration::from_secs(response.cooldown.total_seconds as u64));
          }
          None => {
            match &r.error {
              Some(e) => {
                warn!("Error jumping to system; {}: {}", e.code, e.message);
                break;
              },
              None => {}
            };
          }
        };
      }
    }
    None => {
      match &p.error {
        Some(e) => return Ok(ResponseObject { data: None, error: Some(e.to_owned()), meta: None }),
        None => return Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 0, message: "Unable to find path to system".to_string() }), meta: None })
      };
    }
  };
  Ok(ResponseObject { data: None, error: None, meta: None })
}