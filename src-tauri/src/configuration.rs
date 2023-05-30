use reqwest::Client;

#[derive(Debug, Clone)]
pub struct Configuration {
  pub base_url: String,
  pub token: String,
  pub client: Client
}

impl Configuration {
  pub fn new() -> Configuration {
    Configuration::default()
  }
}

impl Default for Configuration {
  fn default() -> Self {
    Configuration {
      base_url: "https://api.spacetraders.io/v2".to_string(),
      token: "".to_string(),
      client: Client::new()
    }
  }
}