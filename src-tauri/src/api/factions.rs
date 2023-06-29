use tauri::State;
use strum::IntoEnumIterator;

use crate::{models::faction::{Faction, FactionSymbol}, DataState};

use super::requests::{ResponseObject};

#[tauri::command]
pub async fn get_faction(state: State<'_, DataState>, token: String, faction_symbol: String) -> Result<ResponseObject<Faction>, ()> {
  let url = format!("/factions/{}", faction_symbol);
  let result = state.request.get_request::<Faction>(token, url, None).await;
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
  let result = state.request.get_request::<Vec<Faction>>(token, "/factions".to_string(), Some(query)).await;
  Ok(result)
}

#[tauri::command]
pub async fn list_faction_strings() -> Result<ResponseObject<Vec<String>>, ()> {
  let values = FactionSymbol::iter().map(|x| x.to_string()).collect::<Vec<String>>();
  Ok(ResponseObject {
    data: Some(values),
    error: None,
    meta: None
  })
}