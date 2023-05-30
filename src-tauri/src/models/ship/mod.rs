use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

use self::{registration::Registration, navigation::Navigation, crew::Crew, frame::Frame, reactor::Reactor, engine::Engine, module::Module, mount::Mount, cargo::Cargo, fuel::Fuel, cooldown::Cooldown};

use super::SymbolResponse;

pub mod cargo;
pub mod cooldown;
pub mod crew;
pub mod engine;
pub mod frame;
pub mod fuel;
pub mod module;
pub mod mount;
pub mod navigation;
pub mod reactor;
pub mod registration;
pub mod requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
  pub symbol: String,
  pub registration: Registration,
  pub nav: Navigation,
  pub crew: Crew,
  pub frame: Frame,
  pub reactor: Reactor,
  pub engine: Engine,
  pub modules: Vec<Module>,
  pub mounts: Vec<Mount>,
  pub cargo: Cargo,
  pub fuel: Fuel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShipType {
  #[serde(rename = "SHIP_PROBE")]
  Probe,
  #[serde(rename = "SHIP_MINING_DRONE")]
  MiningDrone,
  #[serde(rename = "SHIP_INTERCEPTOR")]
  Interceptor,
  #[serde(rename = "SHIP_LIGHT_HAULER")]
  LightHauler,
  #[serde(rename = "SHIP_COMMAND_FRIGATE")]
  CommandFrigate,
  #[serde(rename = "SHIP_EXPLORER")]
  Explorer,
  #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
  HeavyFreighter,
  #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
  LightShuttle,
  #[serde(rename = "SHIP_ORE_HOUND")]
  OreHound,
  #[serde(rename = "SHIP_REFINING_FREIGHTER")]
  RefiningFreighter
}

impl Display for ShipType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ShipType::Probe => write!(f, "Probe"),
      ShipType::MiningDrone => write!(f, "Mining Drone"),
      ShipType::Interceptor => write!(f, "Interceptor"),
      ShipType::LightHauler => write!(f, "Light Hauler"),
      ShipType::CommandFrigate => write!(f, "Command Frigate"),
      ShipType::Explorer => write!(f, "Explorer"),
      ShipType::HeavyFreighter => write!(f, "Heavy Freighter"),
      ShipType::LightShuttle => write!(f, "Light Shuttle"),
      ShipType::OreHound => write!(f, "Ore Hound"),
      ShipType::RefiningFreighter => write!(f, "Refining Freighter"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipTypeResponse {
  #[serde(rename = "type")]
  pub ship_type: ShipType
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipTransactionResponse {
  #[serde(rename = "waypointSymbol")]
  pub waypoint_symbol: String,
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  pub price: u64,
  #[serde(rename = "agentSymbol")]
  pub agent_symbol: String,
  pub timestamp: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipScanResponse {
  pub cooldown: Cooldown,
  pub ships: Vec<Ship>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannedShip {
  pub symbol: String,
  pub registration: Registration,
  pub nav: Navigation,
  pub frame: SymbolResponse,
  pub reactor: SymbolResponse,
  pub engine: SymbolResponse,
  pub mounts: Vec<SymbolResponse>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardShip {
  #[serde(rename = "type")]
  pub ship_type: ShipType,
  pub name: String,
  pub description: String,
  pub frame: Frame,
  pub reactor: Reactor,
  pub engine: Engine,
  pub modules: Vec<Module>,
  pub mounts: Vec<Mount>
}