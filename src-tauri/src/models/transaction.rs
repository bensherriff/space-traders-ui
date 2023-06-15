use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::{agent::Agent, ship::cargo::Cargo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
  #[serde(rename = "waypointSymbol")]
  pub waypoint_symbol: String,
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  #[serde(rename = "tradeSymbol")]
  pub trade_symbol: String,
  #[serde(rename = "type")]
  pub transaction_type: TransactionType,
  pub units: i32,
  #[serde(rename = "pricePerUnit")]
  pub price_per_unit: i32,
  #[serde(rename = "totalPrice")]
  pub total_price: i32,
  pub timestamp: String
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

impl FromStr for TransactionType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Purchase" => Ok(TransactionType::Purchase),
        "Sell" => Ok(TransactionType::Sell),
        _ => Err(())
      }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResponse {
  pub agent: Agent,
  pub cargo: Cargo,
  pub transaction: Transaction
}