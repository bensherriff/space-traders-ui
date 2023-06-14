use std::{fmt::{Formatter, Display}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::{SymbolResponse, waypoint::WaypointType, ship::cooldown::Cooldown};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
  pub symbol: String,
  #[serde(rename = "sectorSymbol")]
  pub sector_symbol: String,
  #[serde(rename = "type")]
  pub system_type: SystemType,
  pub x: i32,
  pub y: i32,
  pub waypoints: Vec<SystemWaypoint>,
  pub factions: Vec<SymbolResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemWaypoint {
  pub symbol: String,
  #[serde(rename = "type")]
  pub waypoint_type: WaypointType,
  pub x: i32,
  pub y: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
  #[serde(rename = "NEUTRON_STAR")]
  NeutronStar,
  #[serde(rename = "RED_STAR")]
  RedStar,
  #[serde(rename = "ORANGE_STAR")]
  OrangeStar,
  #[serde(rename = "BLUE_STAR")]
  BlueStar,
  #[serde(rename = "YOUNG_STAR")]
  YoungStar,
  #[serde(rename = "WHITE_DWARF")]
  WhiteDwarf,
  #[serde(rename = "BLACK_HOLE")]
  BlackHole,
  #[serde(rename = "HYPERGIANT")]
  HyperGiant,
  #[serde(rename = "NEBULA")]
  Nebula,
  #[serde(rename = "UNSTABLE")]
  Unstable,
}

impl Display for SystemType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        SystemType::NeutronStar => write!(f, "Neutron Star"),
        SystemType::RedStar => write!(f, "Red Star"),
        SystemType::OrangeStar => write!(f, "Orange Star"),
        SystemType::BlueStar => write!(f, "Blue Star"),
        SystemType::YoungStar => write!(f, "Young Star"),
        SystemType::WhiteDwarf => write!(f, "White Dwarf"),
        SystemType::BlackHole => write!(f, "Black Hole"),
        SystemType::HyperGiant => write!(f, "Hyper Giant"),
        SystemType::Nebula => write!(f, "Nebula"),
        SystemType::Unstable => write!(f, "Unstable"),
    }
  }
}

impl FromStr for SystemType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "Neutron Star" => Ok(SystemType::NeutronStar),
          "Red Star" => Ok(SystemType::NeutronStar),
          "Orange Star" => Ok(SystemType::NeutronStar),
          "Blue Star" => Ok(SystemType::NeutronStar),
          "Young Star" => Ok(SystemType::NeutronStar),
          "White Dwarf" => Ok(SystemType::NeutronStar),
          "Black Hole" => Ok(SystemType::NeutronStar),
          "Hyper Giant" => Ok(SystemType::NeutronStar),
          "Nebula" => Ok(SystemType::NeutronStar),
          "Unstable" => Ok(SystemType::NeutronStar),
          _ => Err(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemScanResponse {
  pub cooldown: Cooldown,
  pub systems: Vec<ScannedSystem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedSystem {
  pub symbol: String,
  #[serde(rename = "sectorSymbol")]
  pub sector_symbol: String,
  #[serde(rename = "type")]
  pub system_type: SystemType,
  pub x: i32,
  pub y: i32,
  distance: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpGate {
  #[serde(rename = "jumpRange")]
  pub jump_range: u64,
  #[serde(rename = "factionSymbol")]
  pub faction_symbol: String,
  #[serde(rename = "connectedSystems")]
  pub connected_systems: Vec<ConnectedSystem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedSystem {
  pub symbol: String,
  #[serde(rename = "sectorSymbol")]
  pub sector_symbol: String,
  #[serde(rename = "type")]
  pub system_type: SystemType,
  #[serde(rename = "factionSymbol")]
  pub faction_symbol: String,
  pub x: i32,
  pub y: i32,
  distance: u64
}