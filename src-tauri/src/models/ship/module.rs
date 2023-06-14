use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub symbol: ModuleType,
    pub capacity: Option<u64>,
    pub range: Option<u64>,
    pub name: String,
    pub description: String,
    pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleType {
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    MineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    CargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    CrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    EnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    PassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    MicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    OreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    FuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    JumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    JumpDriveII,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    JumpDriveIII,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    WarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    WarpDriveII,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    WarpDriveIII,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ShieldGeneratorII,
}

impl Display for ModuleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleType::MineralProcessorI => write!(f, "Mineral Processor I"),
            ModuleType::CargoHoldI => write!(f, "Cargo Hold I"),
            ModuleType::CrewQuartersI => write!(f, "Crew Quarters I"),
            ModuleType::EnvoyQuartersI => write!(f, "Envoy Quarters I"),
            ModuleType::PassengerCabinI => write!(f, "Passenger Cabin I"),
            ModuleType::MicroRefineryI => write!(f, "Micro Refinery I"),
            ModuleType::OreRefineryI => write!(f, "Ore Refinery I"),
            ModuleType::FuelRefineryI => write!(f, "Fuel Refinery I"),
            ModuleType::ScienceLabI => write!(f, "Science Lab I"),
            ModuleType::JumpDriveI => write!(f, "Jump Drive I"),
            ModuleType::JumpDriveII => write!(f, "Jump Drive II"),
            ModuleType::JumpDriveIII => write!(f, "Jump Drive III"),
            ModuleType::WarpDriveI => write!(f, "Warp Drive I"),
            ModuleType::WarpDriveII => write!(f, "Warp Drive II"),
            ModuleType::WarpDriveIII => write!(f, "Warp Drive III"),
            ModuleType::ShieldGeneratorI => write!(f, "Shield Generator I"),
            ModuleType::ShieldGeneratorII => write!(f, "Shield Generator II"),
        }
    }
}

impl FromStr for ModuleType {
    type Err = ();
  
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          "Mineral Processor I" => Ok(ModuleType::MineralProcessorI),
          "Cargo Hold I" => Ok(ModuleType::CargoHoldI),
          "Crew Quarters I" => Ok(ModuleType::CrewQuartersI),
          "Envoy Quarters I" => Ok(ModuleType::EnvoyQuartersI),
          "Passenger Cabin I" => Ok(ModuleType::PassengerCabinI),
          "Micro Refinery I" => Ok(ModuleType::MicroRefineryI),
          "Ore Refinery I" => Ok(ModuleType::OreRefineryI),
          "Fuel Refinery I" => Ok(ModuleType::FuelRefineryI),
          "Science Lab I" => Ok(ModuleType::ScienceLabI),
          "Jump Drive I" => Ok(ModuleType::JumpDriveI),
          "Jump Drive II" => Ok(ModuleType::JumpDriveII),
          "Jump Drive III" => Ok(ModuleType::JumpDriveIII),
          "Warp Drive I" => Ok(ModuleType::WarpDriveI),
          "Warp Drive II" => Ok(ModuleType::WarpDriveII),
          "Warp Drive III" => Ok(ModuleType::WarpDriveIII),
          "Shield Generator I" => Ok(ModuleType::ShieldGeneratorI),
          "Shield Generator II" => Ok(ModuleType::ShieldGeneratorII),
          _ => Err(())
        }
    }
  }