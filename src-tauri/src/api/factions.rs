use reqwest::Client;
use tauri::State;

use crate::{models::faction::Faction};

use super::requests::{ResponseObject, handle_result, get_request};

#[tauri::command]
pub async fn get_faction(client: State<'_, Client>, token: String, faction_symbol: String) -> Result<ResponseObject<Faction>, ()> {
  let url = format!("/factions/{}", faction_symbol);
  let result = handle_result(get_request::<Faction>(&client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn get_factions(client: State<'_, Client>, token: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<Faction>>, ()> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page),
  ];
  let result = handle_result(get_request::<Vec<Faction>>(&client, token, "/factions".to_string(), Some(query)).await);
  Ok(result)
}