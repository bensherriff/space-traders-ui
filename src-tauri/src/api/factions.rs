use tauri::State;

use crate::{models::faction::Faction, DataState};

use super::requests::{ResponseObject, handle_result, get_request};

#[tauri::command]
pub async fn get_faction(state: State<'_, DataState>, token: String, faction_symbol: String) -> Result<ResponseObject<Faction>, ()> {
  let url = format!("/factions/{}", faction_symbol);
  let result = handle_result(get_request::<Faction>(&state.client, token, url, None).await);
  Ok(result)
}

#[tauri::command]
pub async fn get_factions(state: State<'_, DataState>, token: String, limit: u64, page: u64) -> Result<ResponseObject<Vec<Faction>>, ()> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page),
  ];
  let result = handle_result(get_request::<Vec<Faction>>(&state.client, token, "/factions".to_string(), Some(query)).await);
  Ok(result)
}