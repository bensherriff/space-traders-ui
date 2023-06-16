use core::time;
use std::thread;

use reqwest::Client;
use tauri::State;

use crate::{models::{system::{System, JumpGate}, waypoint::Waypoint, market::Market, shipyard::Shipyard}, data::system::{insert_system, insert_waypoint}, DataState, SystemsState};

use super::{requests::{ResponseObject, get_request, handle_result, ErrorObject}};

/// Return a list of all systems.
#[tauri::command]
pub async fn list_systems(state: State<'_, DataState>, token: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<System>>, ()> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page)
  ];
  let result = handle_result(get_request::<Vec<System>>(&state.client, token, "/systems".to_string(), Some(query)).await);
  match &result.data {
    Some(data) => {
      for ship in data {
        insert_system(&state.pool, ship);
      }
    }
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn load_all_systems(state: State<'_, DataState>, sys: State<'_, SystemsState>, token: String) -> Result<ResponseObject<Vec<System>>, ()> {
  let mut _state = state;
  let systems_count = crate::data::system::get_systems_count(&_state.pool);

  let limit: u64 = 20;
  let _st = _state.to_owned();
  let _token = token.to_owned();
  let mut total_records = 0;
  let result = list_systems(_st, _token, limit, 1).await;
  let max_page = match &result {
    Ok(r) => {
      match &r.meta {
        Some(d) => {
          total_records = d.total;
          (d.total as f64 / limit as f64).ceil() as u64
        }
        None => 2
      }
    }
    Err(_err) => 2
  };

  let mut s = sys.0.lock().unwrap();
  if (systems_count as u64) < total_records {
    for page in 1..(max_page + 1) {
      let mut attempts = 3;
      while attempts > 0 {
        let _st = _state.to_owned();
        let _token = token.to_owned();
        let result = list_systems(_st, _token, limit, page).await;
        match result {
          Ok(r) => {
            match &r.error {
              Some(error) => {
                if error.code == 429 && attempts > 0 {
                  thread::sleep(time::Duration::from_secs(1));
                  attempts = attempts - 1;
                } else {
                  break;
                }
              }
              None => break
            }
          }
          Err(_err) => {}
        }; 
      }
    }
    let mut systems: Vec<System> = vec![];
    systems.append(&mut crate::data::system::get_all_systems(&_state.pool, None, None));
    *s = systems.to_owned();
    Ok(ResponseObject { data: Some(systems), error: None, meta: None })
  } else {
    let mut systems: Vec<System> = vec![];
    for i in (1..systems_count).step_by(1000) {
      systems.append(&mut crate::data::system::get_all_systems(&_state.pool, Some(i as i32), Some((i + 999) as i32)));
    }
    *s = systems.to_owned();
    Ok(ResponseObject { data: Some(systems), error: None, meta: None })
  }
}

/// Get the details of a system.
#[tauri::command]
pub async fn get_system(state: State<'_, DataState>, token: String, system: String) -> Result<ResponseObject<System>, ()> {
  match crate::data::system::get_system(&state.pool, &system) {
    Some(s) => {
      Ok(ResponseObject { data: Some(s), error: None, meta: None })
    },
    None => {
      let url = format!("/systems/{}", system);
      let result = handle_result(get_request::<System>(&state.client, token, url, None).await);
      match &result.data {
        Some(data) => insert_system(&state.pool, data),
        None => {}
      };
      Ok(result)
    }
  }
}

pub async fn get_path_between_systems(state: State<'_, DataState>, start_system: String, end_system: String) -> Result<ResponseObject<Vec<System>>, ()> {
  Ok(ResponseObject { data: None, error: None, meta: None })
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
pub async fn get_waypoint(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Waypoint>, ()> {
  match crate::data::system::get_waypoint(&state.pool, &waypoint) {
    Some(w) => {
      Ok(ResponseObject { data: Some(w), error: None, meta: None })
    }
    None => {
      let url = format!("/systems/{}/waypoints/{}", system, waypoint);
      let result = handle_result(get_request::<Waypoint>(&state.client, token, url, None).await);
      match &result.data {
        Some(data) => insert_waypoint(&state.pool, data),
        None => {}
      };
      Ok(result)
    }
  }
}

/// Retrieve imports, exports and exchange data from a marketplace.
/// Imports can be sold, exports can be purchased, and exchange goods can be purchased or sold.
/// Send a ship to the waypoint to access trade good prices and recent transactions.
#[tauri::command]
pub async fn get_market(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Market>, ()> {
  let url = format!("/systems/{}/waypoints/{}/market", system, waypoint);
  let result = handle_result(get_request::<Market>(&state.client, token, url, None).await);
  match &result.data {
    Some(data) => {
      crate::data::system::insert_market(&state.pool, &waypoint, &data);
      Ok(result)
    },
    None => {
      match crate::data::system::get_market(&state.pool, &waypoint) {
        Some(m) => {
          Ok(ResponseObject { data: Some(m), error: None, meta: None })
        }
        None => {
            Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 0, message: "Unable to update market data".to_string() }), meta: None })
        }
      }
    }
  }
}

/// Get the shipyard for a waypoint Send a ship to the waypoint to access ships that are
/// currently available for purchase and recent transactions.
#[tauri::command]
pub async fn get_shipyard(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Shipyard>, ()> {
  let url = format!("/systems/{}/waypoints/{}/shipyard", system, waypoint);
  let result = handle_result(get_request::<Shipyard>(&state.client, token, url, None).await);
  Ok(result)
}

/// Get jump gate details for a waypoint.
#[tauri::command]
pub async fn get_jump_gate(client: State<'_, Client>, token: String, system: String, waypoint: String) -> Result<ResponseObject<JumpGate>, ()> {
  let url = format!("/systems/{}/waypoints/{}/jump-gate", system, waypoint);
  let result = handle_result(get_request::<JumpGate>(&client, token, url, None).await);
  Ok(result)
}