use tauri::State;
use chrono::{DateTime, Local};

use crate::{models::{status::Status, agent::{NewAgent, NewAgentResponse}}, DataState};

use self::requests::{ResponseObject};

pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod requests;
pub mod systems;

#[tauri::command]
pub async fn get_status(state: State<'_, DataState>) -> Result<ResponseObject<Status>, ()> {
  let result = state.request.get_request::<Status>("".to_string(), "/".to_string(), None).await;
  Ok(result)
}

#[tauri::command]
pub async fn register(state: State<'_, DataState>, faction: String, symbol: String, email: String) -> Result<ResponseObject<NewAgentResponse>, ()> {
  let new_agent = NewAgent {
    faction: faction.to_string(),
    symbol: symbol.to_string(),
    email: email.to_string()
  };
  let result = state.request.post_request::<NewAgentResponse>("".to_string(), "/register".to_string(), Some(serde_json::to_string(&new_agent).unwrap())).await;
  Ok(result)
}

pub fn calculate_timeout(start_time: String, end_time: String) -> i64 {
  let start = DateTime::parse_from_rfc3339(&start_time).unwrap();
  let end = DateTime::parse_from_rfc3339(&end_time).unwrap();
  end.timestamp() - std::cmp::min(start.timestamp(), Local::now().timestamp())
}