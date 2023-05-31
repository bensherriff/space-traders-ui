use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
  pub symbol: EngineType,
  pub name: String,
  pub description: String,
  pub condition: u64,
  pub speed: u64,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardEngine {
  pub symbol: EngineType,
  pub name: String,
  pub description: String,
  pub speed: u64,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EngineType {
  #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
  ImpulseDriveI,
  #[serde(rename = "ENGINE_ION_DRIVE_I")]
  IonDriveI,
  #[serde(rename = "ENGINE_ION_DRIVE_II")]
  IonDriveII,
  #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
  HyperDriveI,
}

impl Display for EngineType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      EngineType::ImpulseDriveI => write!(f, "Impulse Drive I"),
      EngineType::IonDriveI => write!(f, "Ion Drive I"),
      EngineType::IonDriveII => write!(f, "Ion Drive II"),
      EngineType::HyperDriveI => write!(f, "Hyper Drive I"),
    }
  }
}