use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub name: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role: Role
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "FABRICATOR")]
    Fabricator,
    #[serde(rename = "HARVESTER")]
    Harvester,
    #[serde(rename = "HAULER")]
    Hauler,
    #[serde(rename = "INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "EXCAVATOR")]
    Excavator,
    #[serde(rename = "TRANSPORT")]
    Transport,
    #[serde(rename = "REPAIR")]
    Repair,
    #[serde(rename = "SURVEYOR")]
    Surveyor,
    #[serde(rename = "COMMAND")]
    Command,
    #[serde(rename = "CARRIER")]
    Carrier,
    #[serde(rename = "PATROL")]
    Patrol,
    #[serde(rename = "SATELLITE")]
    Satellite,
    #[serde(rename = "EXPLORER")]
    Explorer,
    #[serde(rename = "REFINERY")]
    Refinery
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Fabricator => write!(f, "Fabricator"),
            Role::Harvester => write!(f, "Harvester"),
            Role::Hauler => write!(f, "Hauler"),
            Role::Interceptor => write!(f, "Interceptor"),
            Role::Excavator => write!(f, "Excavator"),
            Role::Transport => write!(f, "Transport"),
            Role::Repair => write!(f, "Repair"),
            Role::Surveyor => write!(f, "Surveyor"),
            Role::Command => write!(f, "Command"),
            Role::Carrier => write!(f, "Carrier"),
            Role::Patrol => write!(f, "Patrol"),
            Role::Satellite => write!(f, "Satellite"),
            Role::Explorer => write!(f, "Explorer"),
            Role::Refinery => write!(f, "Refinery"),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "Fabricator" => Ok(Role::Fabricator),
          "Harvester" => Ok(Role::Harvester),
          "Hauler" => Ok(Role::Hauler),
          "Interceptor" => Ok(Role::Interceptor),
          "Excavator" => Ok(Role::Excavator),
          "Transport" => Ok(Role::Transport),
          "Repair" => Ok(Role::Repair),
          "Surveyor" => Ok(Role::Surveyor),
          "Command" => Ok(Role::Command),
          "Carrier" => Ok(Role::Carrier),
          "Patrol" => Ok(Role::Patrol),
          "Satellite" => Ok(Role::Satellite),
          "Explorer" => Ok(Role::Explorer),
          "Refinery" => Ok(Role::Refinery),
          _ => Err(())
        }
    }
}