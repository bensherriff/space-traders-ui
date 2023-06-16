use tauri::State;

use crate::{models::agent::Agent, DataState};

use super::requests::{ResponseObject};

#[tauri::command]
pub async fn get_my_agent(state: State<'_, DataState>, token: String) -> Result<ResponseObject<Agent>, ()> {
  let result = state.request.get_request::<Agent>(token, "/my/agent".to_string(), None).await;
  Ok(result)
}