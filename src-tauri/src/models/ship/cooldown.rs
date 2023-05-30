use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooldown {
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  #[serde(rename = "totalSeconds")]
  pub total_seconds: u64,
  #[serde(rename = "remainingSeconds")]
  pub remaining_seconds: u64,
  pub expiration: Option<String>
}