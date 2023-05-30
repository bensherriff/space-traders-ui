use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use super::{agent::Agent, ship::cargo::Cargo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  #[serde(rename = "waypointSymbol")]
  pub waypoint_symbol: Option<String>,
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  #[serde(rename = "tradeSymbol")]
  pub trade_symbol: String,
  #[serde(rename = "type")]
  pub transaction_type: Option<TransactionType>,
  pub units: u64,
  #[serde(rename = "pricePerUnit")]
  pub price_per_unit: Option<u64>,
  #[serde(rename = "totalPrice")]
  pub total_price: Option<u64>,
  pub timestamp: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
  #[serde(rename = "PURCHASE")]
  Purchase,
  #[serde(rename = "SELL")]
  Sell
}

impl Display for TransactionType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TransactionType::Purchase => write!(f, "Purchase"),
      TransactionType::Sell => write!(f, "Sell"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
  pub agent: Agent,
  pub cargo: Cargo,
  pub transaction: Transaction
}