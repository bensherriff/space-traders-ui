use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
  pub symbol: EngineType,
  pub name: String,
  pub description: String,
  pub condition: f32,
  pub speed: i32,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardEngine {
  pub symbol: EngineType,
  pub name: String,
  pub description: String,
  pub speed: i32,
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

impl FromStr for EngineType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Impulse Drive I" => Ok(EngineType::ImpulseDriveI),
        "Ion Drive I" => Ok(EngineType::IonDriveI),
        "Ion Drive II" => Ok(EngineType::IonDriveII),
        "Hyper Drive I" => Ok(EngineType::HyperDriveI),
        _ => Err(())
      }
  }
}
