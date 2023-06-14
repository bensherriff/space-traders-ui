use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
  pub symbol: FrameType,
  pub name: String,
  pub description: String,
  pub condition: f32,
  #[serde(rename = "moduleSlots")]
  pub module_slots: i32,
  #[serde(rename = "mountingPoints")]
  pub mounting_points: i32,
  #[serde(rename = "fuelCapacity")]
  pub fuel_capacity: i32,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardFrame {
  pub symbol: FrameType,
  pub name: String,
  pub description: String,
  #[serde(rename = "moduleSlots")]
  pub module_slots: i32,
  #[serde(rename = "mountingPoints")]
  pub mounting_points: i32,
  #[serde(rename = "fuelCapacity")]
  pub fuel_capacity: i32,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameType {
  #[serde(rename = "FRAME_PROBE")]
  Probe,
  #[serde(rename = "FRAME_DRONE")]
  Drone,
  #[serde(rename = "FRAME_INTERCEPTOR")]
  Interceptor,
  #[serde(rename = "FRAME_RACER")]
  Racer,
  #[serde(rename = "FRAME_FIGHTER")]
  Fighter,
  #[serde(rename = "FRAME_FRIGATE")]
  Frigate,
  #[serde(rename = "FRAME_SHUTTLE")]
  Shuttle,
  #[serde(rename = "FRAME_EXPLORER")]
  Explorer,
  #[serde(rename = "FRAME_MINER")]
  Miner,
  #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
  LightFreighter,
  #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
  HeavyFreighter,
  #[serde(rename = "FRAME_TRANSPORT")]
  Transport,
  #[serde(rename = "FRAME_DESTROYER")]
  Destroyer,
  #[serde(rename = "FRAME_CRUISER")]
  Cruiser,
  #[serde(rename = "FRAME_CARRIER")]
  Carrier,
}

impl Display for FrameType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      FrameType::Probe => write!(f, "Probe"),
      FrameType::Drone => write!(f, "Drone"),
      FrameType::Interceptor => write!(f, "Interceptor"),
      FrameType::Racer => write!(f, "Racer"),
      FrameType::Fighter => write!(f, "Fighter"),
      FrameType::Frigate => write!(f, "Frigate"),
      FrameType::Shuttle => write!(f, "Shuttle"),
      FrameType::Explorer => write!(f, "Explorer"),
      FrameType::Miner => write!(f, "Miner"),
      FrameType::LightFreighter => write!(f, "Light Freighter"),
      FrameType::HeavyFreighter => write!(f, "Heavy Freighter"),
      FrameType::Transport => write!(f, "Transport"),
      FrameType::Destroyer => write!(f, "Destroyer"),
      FrameType::Cruiser => write!(f, "Cruiser"),
      FrameType::Carrier => write!(f, "Carrier"),
    }
  }
}

impl FromStr for FrameType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Probe" => Ok(FrameType::Probe),
        "Drone" => Ok(FrameType::Drone),
        "Interceptor" => Ok(FrameType::Interceptor),
        "Racer" => Ok(FrameType::Racer),
        "Fighter" => Ok(FrameType::Fighter),
        "Frigate" => Ok(FrameType::Frigate),
        "Shuttle" => Ok(FrameType::Shuttle),
        "Explorer" => Ok(FrameType::Explorer),
        "Miner" => Ok(FrameType::Miner),
        "Light Freighter" => Ok(FrameType::LightFreighter),
        "Heavy Freighter" => Ok(FrameType::HeavyFreighter),
        "Transport" => Ok(FrameType::Transport),
        "Destroyer" => Ok(FrameType::Destroyer),
        "Cruiser" => Ok(FrameType::Cruiser),
        "Carrier" => Ok(FrameType::Carrier),
        _ => Err(())
      }
  }
}