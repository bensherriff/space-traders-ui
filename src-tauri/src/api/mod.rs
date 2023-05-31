use crate::models::{status::Status, agent::{NewAgent, NewAgentResponse}};

use self::requests::{ResponseObject, get_request, post_request, handle_result};

pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod requests;
pub mod systems;

#[tauri::command]
pub async fn get_status(token: String) -> ResponseObject<Status> {
  handle_result(get_request::<Status>(token, "/".to_string(), None).await)
}

#[tauri::command]
pub async fn register(token: String, faction: String, symbol: String, email: String) -> ResponseObject<NewAgentResponse> {
  let new_agent = NewAgent {
    faction: faction.to_string(),
    symbol: symbol.to_string(),
    email: email.to_string()
  };
  handle_result(post_request::<NewAgentResponse>(token, "/register".to_string(), Some(serde_json::to_string(&new_agent).unwrap())).await)
}