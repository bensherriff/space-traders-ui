use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use super::{trait_type::TraitType, SymbolResponse, chart::Chart, ship::{cooldown::Cooldown}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
  pub symbol: String,
  #[serde(rename = "type")]
  pub waypoint_type: WaypointType,
  #[serde(rename = "systemSymbol")]
  pub system_symbol: String,
  pub x: i64,
  pub y: i64,
  pub orbitals: Vec<SymbolResponse>,
  pub faction: Option<SymbolResponse>,
  pub traits: Vec<WaypointTrait>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub chart: Option<Chart>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaypointType {
  #[serde(rename = "PLANET")]
  Planet,
  #[serde(rename = "GAS_GIANT")]
  GasGiant,
  #[serde(rename = "MOON")]
  Moon,
  #[serde(rename = "ORBITAL_STATION")]
  OrbitalStation,
  #[serde(rename = "JUMP_GATE")]
  JumpGate,
  #[serde(rename = "ASTEROID_FIELD")]
  AsteroidField,
  #[serde(rename = "NEBULA")]
  Nebula,
  #[serde(rename = "DEBRIS_FIELD")]
  DebrisField,
  #[serde(rename = "GRAVITY_WELL")]
  GravityWell,
}

impl Display for WaypointType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      WaypointType::Planet => write!(f, "Planet"),
      WaypointType::GasGiant => write!(f, "Gas Giant"),
      WaypointType::Moon => write!(f, "Moon"),
      WaypointType::OrbitalStation => write!(f, "Orbital Station"),
      WaypointType::JumpGate => write!(f, "Jump Gate"),
      WaypointType::AsteroidField => write!(f, "Asteroid Field"),
      WaypointType::Nebula => write!(f, "Nebula"),
      WaypointType::DebrisField => write!(f, "Debris Field"),
      WaypointType::GravityWell => write!(f, "Gravity Well"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointTrait {
  pub symbol: TraitType,
  pub name: String,
  pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointScanResponse {
  pub cooldown: Cooldown,
  pub systems: Vec<ScannedWaypoint>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedWaypoint {
  pub cooldown: Cooldown,
  pub systems: Vec<ScannedWaypoint>
}
