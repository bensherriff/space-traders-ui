use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

use super::requirements::Requirements;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
  pub symbol: MountType,
  pub name: String,
  pub description: String,
  pub strength: Option<i32>,
  pub deposits: Option<Vec<DepositType>>,
  pub requirements: Requirements
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MountType {
  #[serde(rename = "MOUNT_GAS_SIPHON_I")]
  GasSiphonI,
  #[serde(rename = "MOUNT_GAS_SIPHON_II")]
  GasSiphonII,
  #[serde(rename = "MOUNT_GAS_SIPHON_III")]
  GasSiphonIII,
  #[serde(rename = "MOUNT_SURVEYOR_I")]
  SurveyorI,
  #[serde(rename = "MOUNT_SURVEYOR_II")]
  SurveyorII,
  #[serde(rename = "MOUNT_SURVEYOR_III")]
  SurveyorIII,
  #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
  SensorArrayI,
  #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
  SensorArrayII,
  #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
  SensorArrayIII,
  #[serde(rename = "MOUNT_MINING_LASER_I")]
  MiningLaserI,
  #[serde(rename = "MOUNT_MINING_LASER_II")]
  MiningLaserII,
  #[serde(rename = "MOUNT_MINING_LASER_III")]
  MiningLaserIII,
  #[serde(rename = "MOUNT_LASER_CANNON_I")]
  LaserCannonI,
  #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
  MissileLauncherI,
  #[serde(rename = "MOUNT_TURRET_I")]
  TurretI,
}

impl Display for MountType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      MountType::GasSiphonI => write!(f, "Gas Siphon I"),
      MountType::GasSiphonII => write!(f, "Gas Siphon II"),
      MountType::GasSiphonIII => write!(f, "Gas Siphon III"),
      MountType::SurveyorI => write!(f, "Surveyor I"),
      MountType::SurveyorII => write!(f, "Surveyor II"),
      MountType::SurveyorIII => write!(f, "Surveyor III"),
      MountType::SensorArrayI => write!(f, "Sensor Array I"),
      MountType::SensorArrayII => write!(f, "Sensor Array II"),
      MountType::SensorArrayIII => write!(f, "Sensor Array III"),
      MountType::MiningLaserI => write!(f, "Mining Laser I"),
      MountType::MiningLaserII => write!(f, "Mining Laser II"),
      MountType::MiningLaserIII => write!(f, "Mining Laser III"),
      MountType::LaserCannonI => write!(f, "Laser Cannon I"),
      MountType::MissileLauncherI => write!(f, "Missile Launcher I"),
      MountType::TurretI => write!(f, "Turret I"),
    }
  }
}

impl FromStr for MountType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Gas Siphon I" => Ok(MountType::GasSiphonI),
        "Gas Siphon II" => Ok(MountType::GasSiphonII),
        "Gas Siphon III" => Ok(MountType::GasSiphonIII),
        "Surveyor I" => Ok(MountType::SurveyorI),
        "Surveyor II" => Ok(MountType::SurveyorII),
        "Surveyor III" => Ok(MountType::SurveyorIII),
        "Sensor Array I" => Ok(MountType::SensorArrayI),
        "Sensor Array II" => Ok(MountType::SensorArrayII),
        "Sensor Array III" => Ok(MountType::SensorArrayIII),
        "Mining Laser I" => Ok(MountType::MiningLaserI),
        "Mining Laser II" => Ok(MountType::MiningLaserII),
        "Mining Laser III" => Ok(MountType::MiningLaserIII),
        "Laser Cannon I" => Ok(MountType::LaserCannonI),
        "Missile Launcher I" => Ok(MountType::MissileLauncherI),
        "Turret I" => Ok(MountType::TurretI),
        _ => Err(())
      }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DepositType {
  #[serde(rename = "QUARTZ_SAND")]
  QuartzSand,
  #[serde(rename = "SILICON_CRYSTALS")]
  SiliconCrystals,
  #[serde(rename = "PRECIOUS_STONES")]
  PreciousStones,
  #[serde(rename = "ICE_WATER")]
  IceWater,
  #[serde(rename = "AMMONIA_ICE")]
  AmmoniaIce,
  #[serde(rename = "IRON_ORE")]
  IronOre,
  #[serde(rename = "COPPER_ORE")]
  CopperOre,
  #[serde(rename = "SILVER_ORE")]
  SilverOre,
  #[serde(rename = "ALUMINUM_ORE")]
  AluminumOre,
  #[serde(rename = "GOLD_ORE")]
  GoldOre,
  #[serde(rename = "PLATINUM_ORE")]
  PlatinumOre,
  #[serde(rename = "DIAMONDS")]
  Diamonds,
  #[serde(rename = "URANITE_ORE")]
  UraniteOre,
  #[serde(rename = "MERITIUM_ORE")]
  MeritiumOre,
}

impl Display for DepositType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        DepositType::QuartzSand => write!(f, "Quartz Sand"),
        DepositType::SiliconCrystals => write!(f, "Silicon Crystals"),
        DepositType::PreciousStones => write!(f, "Precious Stones"),
        DepositType::IceWater => write!(f, "Ice Water"),
        DepositType::AmmoniaIce => write!(f, "Ammonia Ice"),
        DepositType::IronOre => write!(f, "Iron Ore"),
        DepositType::CopperOre => write!(f, "Copper Ore"),
        DepositType::SilverOre => write!(f, "Silver Ore"),
        DepositType::AluminumOre => write!(f, "Aluminum Ore"),
        DepositType::GoldOre => write!(f, "Gold Ore"),
        DepositType::PlatinumOre => write!(f, "Platinum Ore"),
        DepositType::Diamonds => write!(f, "Diamonds"),
        DepositType::UraniteOre => write!(f, "Uranite Ore"),
        DepositType::MeritiumOre => write!(f, "Meritium Ore"),
    }
  }
}

impl FromStr for DepositType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Quartz Sand" => Ok(DepositType::QuartzSand),
        "Silicon Crystals" => Ok(DepositType::SiliconCrystals),
        "Precious Stones" => Ok(DepositType::PreciousStones),
        "Ice Water" => Ok(DepositType::IceWater),
        "Ammonia Ice" => Ok(DepositType::AmmoniaIce),
        "Iron Ore" => Ok(DepositType::IronOre),
        "Copper Ore" => Ok(DepositType::CopperOre),
        "Silver Ore" => Ok(DepositType::SilverOre),
        "Aluminum Ore" => Ok(DepositType::AluminumOre),
        "Gold Ore" => Ok(DepositType::GoldOre),
        "Platinum Ore" => Ok(DepositType::PlatinumOre),
        "Diamonds" => Ok(DepositType::Diamonds),
        "Uranite Ore" => Ok(DepositType::UraniteOre),
        "Meritium Ore" => Ok(DepositType::MeritiumOre),
        _ => Err(())
      }
  }
}