use crate::{configuration, models::faction::Faction};

use super::requests::{ResponseObject, handle_result, get_request};

pub async fn get_faction(configuration: &configuration::Configuration, faction_symbol: String) -> ResponseObject<Faction> {
  let local_configuration = configuration;
  let url = format!("{}/factions/{}", local_configuration.base_url, faction_symbol);
  handle_result(get_request::<Faction>(configuration, &url, None).await)
}

pub async fn get_factions(configuration: &configuration::Configuration, limit: &u8, page: &u8) -> ResponseObject<Vec<Faction>> {
  let local_configuration = configuration;
  let url = format!("{}/factions", local_configuration.base_url);
  let local_limit = &limit.to_string();
  let local_page = &page.to_string();
  let query = vec![
    ("limit", local_limit),
    ("page", local_page),
  ];
  handle_result(get_request::<Vec<Faction>>(configuration, &url, Some(query)).await)
}