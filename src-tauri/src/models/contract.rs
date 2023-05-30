use serde::{Deserialize, Serialize};

use super::{agent::Agent, ship::cargo::Cargo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
  pub id: String,
  #[serde(rename = "factionSymbol")]
  pub faction_symbol: String,
  #[serde(rename = "type")]
  pub contract_type: String,
  pub terms: Terms,
  pub accepted: bool,
  pub fulfilled: bool,
  pub expiration: String,
  #[serde(rename = "deadlineToAccept")]
  pub deadline_to_accept: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
  pub deadline: String,
  pub payment: Payment,
  pub deliver: Vec<DeliverGood>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
  #[serde(rename = "onAccepted")]
  pub on_accepted: u64,
  #[serde(rename = "onFulfilled")]
  pub on_fulfilled: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverGood {
  #[serde(rename = "tradeSymbol")]
  pub trade_symbol: String,
  #[serde(rename = "destinationSymbol")]
  pub destination_symbol: String,
  #[serde(rename = "unitsRequired")]
  pub units_required: u64,
  #[serde(rename = "unitsFulfilled")]
  pub units_fulfilled: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAgreementResponse {
  pub agent: Agent,
  pub contract: Contract
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDelivery {
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  #[serde(rename = "tradeSymbol")]
  pub trade_symbol: String,
  pub units: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeliveryResponse {
  pub contract: Contract,
  pub cargo: Cargo
}