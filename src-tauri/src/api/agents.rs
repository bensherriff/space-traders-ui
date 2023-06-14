use log::info;
use reqwest::Client;
use tauri::State;

use crate::{models::agent::Agent};

use super::requests::{ResponseObject, get_request, handle_result};

#[tauri::command]
pub async fn get_my_agent(client: State<'_, Client>, token: String) -> Result<ResponseObject<Agent>, ()> {
  let result = handle_result(get_request::<Agent>(&client, token, "/my/agent".to_string(), None).await);
  Ok(result)
}