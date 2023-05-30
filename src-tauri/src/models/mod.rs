use serde::{Serialize, Deserialize};

pub mod agent;
pub mod chart;
pub mod contract;
pub mod faction;
pub mod market;
pub mod ship;
pub mod shipyard;
pub mod size;
pub mod status;
pub mod survey;
pub mod system;
pub mod trait_type;
pub mod transaction;
pub mod waypoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolResponse {
  pub symbol: String
}