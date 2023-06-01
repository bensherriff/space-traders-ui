use crate::{models::faction::Faction};

use super::requests::{ResponseObject, handle_result, get_request};

#[tauri::command]
pub async fn get_faction(token: String, faction_symbol: String) -> ResponseObject<Faction> {
  let url = format!("/factions/{}", faction_symbol);
  handle_result(get_request::<Faction>(token, url, None).await)
}

#[tauri::command]
pub async fn get_factions(token: String, limit: u64, page: u64) -> ResponseObject<Vec<Faction>> {
  let _limit = limit.to_string();
  let _page = page.to_string();
  let query = vec![
    ("limit", _limit),
    ("page", _page),
  ];
  handle_result(get_request::<Vec<Faction>>(token, "/factions".to_string(), Some(query)).await)
}