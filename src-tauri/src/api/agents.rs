use tauri::State;

use crate::{models::agent::Agent, DataState};

use super::requests::{ResponseObject, get_request, handle_result};

#[tauri::command]
pub async fn get_my_agent(state: State<'_, DataState>, token: String) -> Result<ResponseObject<Agent>, ()> {
  let result = handle_result(get_request::<Agent>(&state.client, token, "/my/agent".to_string(), None).await);
  Ok(result)
}