use serde::{Deserialize, Serialize};

use crate::models::{agent::Agent, transaction::Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fuel {
  pub current: u64,
  pub capacity: u64,
  pub consumed: Consumed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consumed {
  pub amount: u64,
  pub timestamp: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefuelResponse {
  pub agent: Agent,
  pub fuel: Fuel,
  pub transaction: Transaction
}