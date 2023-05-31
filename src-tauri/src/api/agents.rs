use crate::{models::agent::Agent};

use super::requests::{ResponseObject, get_request, handle_result};

#[tauri::command]
pub async fn get_my_agent(token: String) -> ResponseObject<Agent> {
  handle_result(get_request::<Agent>(token, "/my/agent".to_string(), None).await)
}