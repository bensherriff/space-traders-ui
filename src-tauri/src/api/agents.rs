use crate::{configuration, models::agent::Agent};

use super::requests::{ResponseObject, get_request, handle_result};

pub async fn get_my_agent(configuration: &configuration::Configuration) -> ResponseObject<Agent> {
  let local_configuration = configuration;
  let url = format!("{}/my/agent", local_configuration.base_url);
  handle_result(get_request::<Agent>(configuration, &url, None).await)
}