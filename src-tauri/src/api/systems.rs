use std::collections::HashMap;

use log::warn;
use petgraph::{graph::NodeIndex};
use tauri::State;
use tauri_plugin_store::StoreBuilder;

use crate::{models::{system::{System, JumpGate}, waypoint::{Waypoint, WaypointType}, market::Market, shipyard::Shipyard}, DataState, data::get_store_path, api::requests::ErrorObject};

use super::{requests::{ResponseObject}};

/// Return a list of all systems.
#[tauri::command]
pub async fn list_systems(state: State<'_, DataState>, token: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<System>>, ()> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page)
  ];
  let result = state.request.get_request::<Vec<System>>(token, "/systems".to_string(), Some(query)).await;
  match &result.data {
    Some(data) => {
      for system in data {
        crate::data::system::insert_system(&state.pool, system);
      }
    }
    None => {}
  };
  Ok(result)
}

#[tauri::command]
pub async fn list_all_systems(state: State<'_, DataState>, app_handle: tauri::AppHandle, token: String) -> Result<ResponseObject<Vec<System>>, ()> {
  let mut _state = state;
  let systems_count = crate::data::system::get_systems_count(&_state.pool);
  let mut _store = StoreBuilder::new(app_handle, get_store_path()).build();
  match _store.load() {
    Ok(_) => {}
    Err(err) => {
      warn!("Error loading store: {:?}", err);
    }
  };
  const SYSTEM_STRING: &str = "systems";

  let limit: u64 = 20;
  let _st = _state.to_owned();
  let _token = token.to_owned();
  let mut total_records = 0;
  let result = list_systems(_st, _token, limit, 1).await;
  let total_pages = match &result {
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

  if _store.has(SYSTEM_STRING) {
    let _value = _store.get(SYSTEM_STRING).unwrap().to_owned();
    let _systems: Vec<System> = match serde_json::from_value(_value) {
      Ok(s) => s,
      Err(_err) => vec![]
    };
    if _systems.len() as u64 == total_records {
      return Ok(ResponseObject {
        data: Some(_systems),
        error: None,
        meta: None
      });
    }
  }

  let mut systems: Vec<System> = vec![];
  if (systems_count as u64) < total_records {
    for page in 1..(total_pages + 1) {
      let _st = _state.to_owned();
      let _token = token.to_owned();
      let result = list_systems(_st, _token, limit, page).await;
      match result {
        Ok(r) => {
          match &r.data {
            Some(d) => {
              let mut _systems = d.to_owned();
              systems.append(&mut _systems);
          //     for system in d.iter() {
          //       let total_waypoints = system.waypoints.len() as u64;
          //       let waypoint_pages = (total_waypoints as f64 / limit as f64).ceil() as u64;
          //       for waypoint_page in 1..(waypoint_pages + 1) {
          //         let _st_waypoint = _state.to_owned();
          //         let _token_waypoint = token.to_owned();
          //         let waypoints_result = list_waypoints(_st_waypoint, _token_waypoint, system.symbol.to_string(), limit, waypoint_page).await;
          //         match waypoints_result {
          //           Ok(_) => {}
          //           Err(_err) => {}
          //         };
          //       }
          //     }
            }
            None => {}
          };
        }
        Err(_err) => {}
      };
    }
  } else {
    for i in (1..systems_count).step_by(1000) {
      systems.append(&mut crate::data::system::get_all_systems(&_state.pool, Some(i as i32), Some((i + 999) as i32)));
    }
  }
  let _systems = systems.to_owned();
  match _store.insert(SYSTEM_STRING.to_string(), serde_json::json!(_systems)) {
    Ok(_) => {
      match _store.save() {
        Ok(_) => {}
        Err(_err) => {
          warn!("Error saving store: {:?}", _err);
        }
      };
    },
    Err(err) => warn!("Error storing systems {:?}", err)
  }

  Ok(ResponseObject { data: Some(systems), error: None, meta: None })
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
      let result = state.request.get_request::<System>(token, url, None).await;
      match &result.data {
        Some(data) => crate::data::system::insert_system(&state.pool, data),
        None => {}
      };
      Ok(result)
    }
  }
}

#[tauri::command]
pub async fn get_path_to_system(state: State<'_, DataState>, app_handle: tauri::AppHandle, token: String, start_symbol: String, end_symbol: String) -> Result<ResponseObject<Vec<String>>, ()> {
  if start_symbol == end_symbol {
    return Ok(ResponseObject { data: Some(vec![]), error: None, meta: None });
  }
  let _state = state.to_owned();
  let _token = token.to_owned();
  let _app_handle = app_handle.to_owned();
  let _start_symbol = start_symbol.to_owned();

  let mut graph = petgraph::Graph::<String, i32>::new();
  let start_system = match get_system(_state, _token, start_symbol).await.unwrap().data {
    Some(s) => s,
    None => return Ok(ResponseObject { data: None, error: None, meta: None })
  };

  let mut checked_systems: HashMap<String, NodeIndex> = HashMap::new();
  let mut systems = vec![start_system];
  let mut searching_for_end_system = false;
  while !systems.is_empty() {
    let system = systems.pop().unwrap();
    let current_system_node: NodeIndex;
    if checked_systems.contains_key(system.symbol.as_str()) {
      current_system_node = checked_systems.get(system.symbol.as_str()).unwrap().to_owned();
    } else {
      current_system_node = graph.add_node(system.symbol.to_owned());
      checked_systems.insert(system.symbol.to_owned(), current_system_node);
    }
    for waypoint in system.waypoints.iter() {
      if matches!(waypoint.waypoint_type, WaypointType::JumpGate) {
        let _state = state.to_owned();
        let _token = token.to_owned();
        match get_jump_gate(_state, _token, system.symbol.to_owned(), waypoint.symbol.to_owned()).await {
          Ok(j) => {
            match &j.data {
              Some(jump_gate) => {
                for connected_system in jump_gate.connected_systems.iter() {
                  let next_system_node: NodeIndex;
                  if checked_systems.contains_key(connected_system.symbol.as_str()) {
                    next_system_node = checked_systems.get(connected_system.symbol.as_str()).unwrap().to_owned();
                  } else {
                    let _state = state.to_owned();
                    let _token = token.to_owned();
                    next_system_node = graph.add_node(connected_system.symbol.to_owned());
                    checked_systems.insert(connected_system.symbol.to_owned(), next_system_node);
                    if !searching_for_end_system {
                      let _system = get_system(_state, _token, connected_system.symbol.to_owned()).await.unwrap().data.unwrap();
                      systems.push(_system);
                    }
                  }
                  graph.update_edge(current_system_node.to_owned(), next_system_node.to_owned(), connected_system.distance);
                  if connected_system.symbol == end_symbol {
                    searching_for_end_system = true;
                  }
                }
              }
              None => {}
            }
          }
          Err(err) => warn!("Error getting jump gate: {:?}", err)
        }
      }
    }
  }

  let start_system_node = checked_systems.get(&_start_symbol).unwrap().to_owned();
  let end_system_node = checked_systems.get(&end_symbol).unwrap().to_owned();
  let path = petgraph::algo::astar(&graph, start_system_node.to_owned(), |finish| finish == end_system_node.to_owned(), |e| *e.weight(), |_| 0);
  match path {
    Some(p) => {
      let mut path_systems = vec![];
      for node in p.1.iter() {
        let system_symbol = graph.node_weight(*node).unwrap().to_owned();
        path_systems.push(system_symbol);
      }
      return Ok(ResponseObject { data: Some(path_systems), error: None, meta: None })
    }
    None => return Ok(ResponseObject { data: None, error: Some(ErrorObject { code: 9000, message: "No path found".to_string() }), meta: None })
  }
}

/// Return a list of all waypoints.
#[tauri::command]
pub async fn list_waypoints(state: State<'_, DataState>, token: String, system: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<Waypoint>>, ()> {
  let url = format!("/systems/{}/waypoints", system);
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page)
  ];
  let result = state.request.get_request::<Vec<Waypoint>>(token, url, Some(query)).await;
  match &result.data {
    Some(data) => {
      for waypoint in data.iter() {
        crate::data::system::insert_waypoint(&state.pool, waypoint);
      }
    }
    None => {}
  }
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
      let result = state.request.get_request::<Waypoint>(token, url, None).await;
      match &result.data {
        Some(data) => crate::data::system::insert_waypoint(&state.pool, data),
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
  let _state = state.to_owned();
  let _waypoint = waypoint.to_owned();

  async fn get_fresh_data(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Market>, ()> {
    let url = format!("/systems/{}/waypoints/{}/market", system, waypoint);
    let result = state.request.get_request::<Market>(token, url, None).await;
    match &result.data {
      Some(d) => {
        crate::data::system::insert_market(&state.pool, &waypoint, &d);
      }
      None => {}
    };
    return Ok(result)
  }

  match crate::data::system::get_market(&_state.pool, &_waypoint) {
    Some(m) => {
      match crate::api::fleet::get_ships_at_waypoint(_state, _waypoint) {
        Ok(r) => {
          match &r.data {
            Some(s) => {
              if s.len() > 0 {
                return get_fresh_data(state, token, system, waypoint).await
              }
            }
            None => {}
          }
        }
        Err(_err) => {}
      }
      return Ok(ResponseObject { data: Some(m), error: None, meta: None })
    }
    None => {
      return get_fresh_data(state, token, system, waypoint).await
    }
  }
}

/// Get the shipyard for a waypoint Send a ship to the waypoint to access ships that are
/// currently available for purchase and recent transactions.
#[tauri::command]
pub async fn get_shipyard(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<Shipyard>, ()> {
  let url = format!("/systems/{}/waypoints/{}/shipyard", system, waypoint);
  let result = state.request.get_request::<Shipyard>(token, url, None).await;
  Ok(result)
}

/// Get jump gate details for a waypoint.
#[tauri::command]
pub async fn get_jump_gate(state: State<'_, DataState>, token: String, system: String, waypoint: String) -> Result<ResponseObject<JumpGate>, ()> {
  match crate::data::system::get_jump_gate(&state.pool, &waypoint) {
    Some(j) => {
      Ok(ResponseObject { data: Some(j), error: None, meta: None })
    }
    None => {
      let url = format!("/systems/{}/waypoints/{}/jump-gate", system, waypoint);
      let result = state.request.get_request::<JumpGate>(token, url, None).await;
      match &result.data {
        Some(data) => crate::data::system::insert_jump_gate(&state.pool, &waypoint, data),
        None => {}
      };
      Ok(result)
    }
  }
}