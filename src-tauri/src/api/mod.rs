use crate::{models::{status::Status, agent::{NewAgent, NewAgentResponse}}, configuration};

use self::requests::{ResponseObject, get_request, post_request, handle_result};

pub mod agents;
pub mod contracts;
pub mod factions;
pub mod fleet;
pub mod requests;
pub mod systems;

pub async fn get_status(configuration: &configuration::Configuration) -> ResponseObject<Status> {
  let local_configuration = configuration;
  let url = format!("{}/", local_configuration.base_url);
  handle_result(get_request::<Status>(configuration, &url, None).await)
}

pub async fn register(configuration: &configuration::Configuration, faction: &String, symbol: &String, email: &String) -> ResponseObject<NewAgentResponse> {
  let local_configuration = configuration;
  let url = format!("{}/register", local_configuration.base_url);
  let new_agent = NewAgent {
    faction: faction.to_string(),
    symbol: symbol.to_string(),
    email: email.to_string()
  };
  handle_result(post_request::<NewAgentResponse>(configuration, &url, Some(serde_json::to_string(&new_agent).unwrap())).await)
}