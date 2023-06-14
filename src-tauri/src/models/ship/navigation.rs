use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use crate::models::waypoint::{WaypointType};

use super::{cooldown::Cooldown, fuel::Fuel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navigation {
  #[serde(rename = "systemSymbol")]
  pub system_symbol: String,
  #[serde(rename = "waypointSymbol")]
  pub waypoint_symbol: String,
  pub route: Route,
  pub status: NavStatus,
  #[serde(rename = "flightMode")]
  pub flight_mode: FlightMode
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
  pub destination: RouteWaypoint,
  pub departure: RouteWaypoint,
  #[serde(rename = "departureTime")]
  pub departure_time: String,
  #[serde(rename = "arrival")]
  pub arrival_time: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteWaypoint {
  pub symbol: String,
  #[serde(rename = "type")]
  pub waypoint_type: WaypointType,
  #[serde(rename = "systemSymbol")]
  pub system_symbol: String,
  pub x: i32,
  pub y: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NavStatus {
  #[serde(rename = "IN_TRANSIT")]
  InTransit,
  #[serde(rename = "IN_ORBIT")]
  InOrbit,
  #[serde(rename = "DOCKED")]
  Docked
}

impl Display for NavStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      NavStatus::InTransit => write!(f, "In Transit"),
      NavStatus::InOrbit => write!(f, "In Orbit"),
      NavStatus::Docked => write!(f, "Docked"),
    }
  }
}

impl FromStr for NavStatus {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "In Transit" => Ok(NavStatus::InTransit),
        "In Orbit" => Ok(NavStatus::InOrbit),
        "Docked" => Ok(NavStatus::Docked),
        _ => Err(())
      }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlightMode {
  #[serde(rename = "DRIFT")]
  Drift,
  #[serde(rename = "STEALTH")]
  Stealth,
  #[serde(rename = "CRUISE")]
  Cruise,
  #[serde(rename = "BURN")]
  Burn
}

impl Display for FlightMode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      FlightMode::Drift => write!(f, "Drift"),
      FlightMode::Stealth => write!(f, "Stealth"),
      FlightMode::Cruise => write!(f, "Cruise"),
      FlightMode::Burn => write!(f, "Burn"),
    }
  }
}

impl FromStr for FlightMode {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Drift" => Ok(FlightMode::Drift),
        "Stealth" => Ok(FlightMode::Stealth),
        "Cruise" => Ok(FlightMode::Cruise),
        "Burn" => Ok(FlightMode::Burn),
        _ => Err(())
      }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipJumpResponse {
  pub nav: Navigation,
  pub cooldown: Cooldown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipNavigateResponse {
  pub nav: Navigation,
  pub fuel: Fuel
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationResponse {
  pub nav: Navigation
}