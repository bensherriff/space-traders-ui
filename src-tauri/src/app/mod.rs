use std::collections::HashMap;

use log::warn;
use petgraph::graph::NodeIndex;
use tauri::State;

use crate::api::requests::{ResponseObject, ErrorObject};
use crate::{DataState, models::waypoint::WaypointType};
use crate::api::systems::{get_system, get_jump_gate};

//TOD: Introduce caching paths
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

#[tauri::command]
pub async fn auto_extract_resources(state: State<'_, DataState>, app_handle: tauri::AppHandle, token: String, system_symbol: String) -> Result<(), ()> {
  return Ok(())
}