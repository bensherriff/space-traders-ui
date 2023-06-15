use serde::{Serialize, Deserialize};

use super::ship::{ShipTypeResponse, ShipyardShip};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipyard {
  pub symbol: String,
  #[serde(rename = "shipTypes")]
  pub ship_types: Vec<ShipTypeResponse>,
  pub transactions: Option<Vec<ShipyardTransaction>>,
  pub ships: Option<Vec<ShipyardShip>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardTransaction {
  #[serde(rename = "waypointSymbol")]
  pub waypoint_symbol: String,
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  pub price: i32,
  #[serde(rename = "agentSymbol")]
  pub agent_symbol: String,
  pub timestamp: String
}