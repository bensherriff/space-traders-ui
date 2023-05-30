use serde::{Deserialize, Serialize};

use super::{contract::Contract, faction::Faction, ship::Ship};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
  #[serde(rename = "accountId")]
  pub account_id: String,
  pub credits: i64,
  pub headquarters: String,
  #[serde(rename = "startingFaction")]
  pub starting_faction: String,
  pub symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAgent {
  pub faction: String,
  pub symbol: String,
  pub email: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAgentResponse {
  pub agent: Agent,
  pub contract: Contract,
  pub faction: Faction,
  pub ship: Ship,
  pub token: String
}