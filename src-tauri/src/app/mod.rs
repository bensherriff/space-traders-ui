use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use log::{warn, debug};
use petgraph::graph::NodeIndex;
use tauri::State;

use crate::api::requests::{ResponseObject, ErrorObject};
use crate::models::ship::cooldown::Cooldown;
use crate::models::survey::Survey;
use crate::{DataState, models::waypoint::WaypointType};
use crate::api::systems::{get_system, get_jump_gate};

//TOD: Introduce caching paths
pub async fn get_path_to_system(state: State<'_, DataState>, token: String, start_symbol: String, end_symbol: String) -> Result<ResponseObject<Vec<String>>, ()> {
  if start_symbol == end_symbol {
    return Ok(ResponseObject { data: Some(vec![]), error: None, meta: None });
  }

  let mut graph = petgraph::Graph::<String, i32>::new();
  let start_system = match get_system(state.to_owned(), token.to_owned(), start_symbol.to_owned()).await.unwrap().data {
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
        match get_jump_gate(state.to_owned(), token.to_owned(), system.symbol.to_owned(), waypoint.symbol.to_owned()).await {
          Ok(j) => {
            match &j.data {
              Some(jump_gate) => {
                for connected_system in jump_gate.connected_systems.iter() {
                  let next_system_node: NodeIndex;
                  if checked_systems.contains_key(connected_system.symbol.as_str()) {
                    next_system_node = checked_systems.get(connected_system.symbol.as_str()).unwrap().to_owned();
                  } else {
                    next_system_node = graph.add_node(connected_system.symbol.to_owned());
                    checked_systems.insert(connected_system.symbol.to_owned(), next_system_node);
                    if !searching_for_end_system {
                      let _system = get_system(state.to_owned(), token.to_owned(), connected_system.symbol.to_owned()).await.unwrap().data.unwrap();
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

  let start_system_node = checked_systems.get(&start_symbol.to_owned()).unwrap().to_owned();
  let end_system_node = checked_systems.get(&end_symbol.to_owned()).unwrap().to_owned();
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
pub async fn auto_extract_resources(state: State<'_, DataState>, token: String, symbol: String, waypoint: String, create_survey: bool) -> Result<bool, ()> {
  debug!("Auto extracting resources for ship: {}", symbol);

  async fn internal_extract(state: State<'_, DataState>, token: String, symbol: String, waypoint: String, survey: Option<Survey>) -> bool {
    let mut internal_survey = survey;
    debug!("Extracting resources for ship: {}", symbol);
    let mut cargo_full = false;
    while !cargo_full {
      match crate::api::fleet::extract_resources(state.to_owned(), token.to_owned(), symbol.to_owned(), internal_survey.to_owned()).await {
        Ok(er) => {
          match &er.data {
            Some(extracted_cargo) => {
              debug!("{} extracted {} units of {}", symbol, extracted_cargo.extraction.extraction_yield.units, extracted_cargo.extraction.extraction_yield.symbol);
              cargo_full = extracted_cargo.cargo.units >= extracted_cargo.cargo.capacity;
              if cargo_full {
                debug!("{}'s cargo hull is full", symbol);
                return true;
              } else {
                debug!("Extraction on cooldown for {} seconds", extracted_cargo.cooldown.remaining_seconds);
                thread::sleep(Duration::from_secs(extracted_cargo.cooldown.remaining_seconds.to_owned() as u64));
              }
            }
            None => {
              match &er.error {
                Some(error) => {
                  if error.code == 4221 {
                    internal_survey = generate_survey(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned()).await;
                  } else {
                    break;
                  }
                }
                None => {}
              }
            }
          }
        }
        Err(err) => warn!("Error extracting resources: {:?}", err)
      };
    }
    return false;
  }

  fn choose_best_survey(surveys: Vec<Survey>) -> Option<Survey> {
    debug!("Found {} {}", surveys.len(), if surveys.len() == 1 { "survey" } else { "surveys" });
    if surveys.len() > 0 {
      let mut best_survey = surveys[0].to_owned();
      for survey in surveys.iter() {
        if survey.size.to_owned() as u8 > best_survey.size.to_owned() as u8 {
          best_survey = survey.to_owned();
        }
      }
      Some(best_survey)
    } else {
      None
    }
  }

  async fn generate_survey(state: State<'_, DataState>, token: String, symbol: String, waypoint: String) -> Option<Survey> {
    let s = crate::api::fleet::create_survey(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned()).await;
    match s {
      Ok(sr) => {
        match &sr.data {
          Some(survey_response) => {
            let survey = choose_best_survey(survey_response.surveys.to_owned()).unwrap();
            let cooldown_remainder = &survey_response.cooldown.remaining_seconds;
            debug!("Using survey: {} (cooldown remainder: {})", survey.signature, cooldown_remainder);
            if cooldown_remainder > &0 {
              thread::sleep(Duration::from_secs(cooldown_remainder.to_owned() as u64));
              crate::data::fleet::update_survey(&state.pool, &survey.signature, &Cooldown { remaining_seconds: 0, ..survey_response.cooldown.to_owned() });
            }
            Some(survey)
          }
          None => None
        }
      }
      Err(err) => {
        warn!("Error creating survey: {:?}", err);
        None
      }
    }
  }

  if create_survey {
    let survey = generate_survey(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned()).await;
    Ok(internal_extract(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned(), survey).await)
  } else {
    Ok(internal_extract(state.to_owned(), token.to_owned(), symbol.to_owned(), waypoint.to_owned(), None).await)
  }
}