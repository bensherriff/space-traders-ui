use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactor {
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub condition: u64,
  #[serde(rename = "powerOutput")]
  pub power_output: u64,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardReactor {
  pub symbol: String,
  pub name: String,
  pub description: String,
  #[serde(rename = "powerOutput")]
  pub power_output: u64,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReactorType {
  #[serde(rename = "REACTOR_SOLAR_I")]
  SolarI,
  #[serde(rename = "REACTOR_FUSION_I")]
  FusionI,
  #[serde(rename = "REACTOR_FISSION_I")]
  FissionI,
  #[serde(rename = "REACTOR_CHEMICAL_I")]
  ChemicalI,
  #[serde(rename = "REACTOR_ANTIMATTER_I")]
  AntimatterI,
}

impl Display for ReactorType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ReactorType::SolarI => write!(f, "Solar Reactor I"),
      ReactorType::FusionI => write!(f, "Fusion Reactor I"),
      ReactorType::FissionI => write!(f, "Fission Reactor I"),
      ReactorType::ChemicalI => write!(f, "Chemical Reactor I"),
      ReactorType::AntimatterI => write!(f, "Antimatter Reactor I"),
    }
  }
}