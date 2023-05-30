use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

use super::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
  pub symbol: String,
  pub exports: Vec<MarketItem>,
  pub imports: Vec<MarketItem>,
  pub exchange: Vec<MarketItem>,
  pub transactions: Option<Vec<Transaction>>,
  #[serde(rename = "tradeGoods")]
  pub trade_goods: Option<Vec<TradeGood>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeGood {
  pub symbol: String,
  #[serde(rename = "tradeVolume")]
  pub trade_volume: u64,
  #[serde(rename = "supply")]
  pub supply_type: SupplyType,
  #[serde(rename = "purchasePrice")]
  pub purchase_price: u64,
  #[serde(rename = "sellPrice")]
  pub sell_price: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplyType {
  #[serde(rename = "SCARCE")]
  Scarce,
  #[serde(rename = "LIMITED")]
  Limited,
  #[serde(rename = "MODERATE")]
  Moderate,
  #[serde(rename = "ABUNDANT")]
  Abundant
}

impl Display for SupplyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SupplyType::Scarce => write!(f, "Scarce"),
      SupplyType::Limited => write!(f, "Limited"),
      SupplyType::Moderate => write!(f, "Moderate"),
      SupplyType::Abundant => write!(f, "Abundant"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketItem {
  pub symbol: MarketItemType,
  pub name: String,
  pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketItemType {
  #[serde(rename = "PRECIOUS_STONES")]
PreciousStones,
#[serde(rename = "QUARTZ_SAND")]
QuartzSand,
#[serde(rename = "SILICON_CRYSTALS")]
SiliconCrystals,
#[serde(rename = "AMMONIA_ICE")]
AmmoniaIce,
#[serde(rename = "LIQUID_HYDROGEN")]
LiquidHydrogen,
#[serde(rename = "LIQUID_NITROGEN")]
LiquidNitrogen,
#[serde(rename = "ICE_WATER")]
IceWater,
#[serde(rename = "EXOTIC_MATTER")]
ExoticMatter,
#[serde(rename = "ADVANCED_CIRCUITRY")]
AdvancedCircuitry,
#[serde(rename = "GRAVITON_EMITTERS")]
GravitonEmitters,
#[serde(rename = "IRON")]
Iron,
#[serde(rename = "IRON_ORE")]
IronOre,
#[serde(rename = "COPPER")]
Copper,
#[serde(rename = "COPPER_ORE")]
CopperOre,
#[serde(rename = "ALUMINUM")]
Aluminum,
#[serde(rename = "ALUMINUM_ORE")]
AluminumOre,
#[serde(rename = "SILVER")]
Silver,
#[serde(rename = "SILVER_ORE")]
SilverOre,
#[serde(rename = "GOLD")]
Gold,
#[serde(rename = "GOLD_ORE")]
GoldOre,
#[serde(rename = "PLATINUM")]
Platinum,
#[serde(rename = "PLATINUM_ORE")]
PlatinumOre,
#[serde(rename = "DIAMONDS")]
Diamonds,
#[serde(rename = "URANITE")]
Uranite,
#[serde(rename = "URANITE_ORE")]
UraniteOre,
#[serde(rename = "MERITIUM")]
Meritium,
#[serde(rename = "MERITIUM_ORE")]
MeritiumOre,
#[serde(rename = "HYDROCARBON")]
Hydrocarbon,
#[serde(rename = "ANTIMATTER")]
Antimatter,
#[serde(rename = "FERTILIZERS")]
Fertilizers,
#[serde(rename = "FABRICS")]
Fabrics,
#[serde(rename = "FOOD")]
Food,
#[serde(rename = "JEWELRY")]
Jewelry,
#[serde(rename = "MACHINERY")]
Machinery,
#[serde(rename = "FIREARMS")]
Firearms,
#[serde(rename = "ASSAULT_RIFLES")]
AssaultRifles,
#[serde(rename = "MILITARY_EQUIPMENT")]
MilitaryEquipment,
#[serde(rename = "EXPLOSIVES")]
Explosives,
#[serde(rename = "LAB_INSTRUMENTS")]
LabInstruments,
#[serde(rename = "AMMUNITION")]
Ammunition,
#[serde(rename = "ELECTRONICS")]
Electronics,
#[serde(rename = "SHIP_PLATING")]
ShipPlating,
#[serde(rename = "EQUIPMENT")]
Equipment,
#[serde(rename = "FUEL")]
Fuel,
#[serde(rename = "MEDICINE")]
Medicine,
#[serde(rename = "DRUGS")]
Drugs,
#[serde(rename = "CLOTHING")]
Clothing,
#[serde(rename = "MICROPROCESSORS")]
Microprocessors,
#[serde(rename = "PLASTICS")]
Plastics,
#[serde(rename = "POLYNUCLEOTIDES")]
PolyNeucleotides,
#[serde(rename = "BIOCOMPOSITES")]
BioComposites,
#[serde(rename = "NANOBOTS")]
Nanobots,
#[serde(rename = "AI_MAINFRAMES")]
AiMainframes,
#[serde(rename = "QUANTUM_DRIVES")]
QuantumDrives,
#[serde(rename = "ROBOTIC_DRONES")]
RoboticDrones,
#[serde(rename = "CYBER_IMPLANTS")]
CyberImplants,
#[serde(rename = "GENE_THERAPEUTICS")]
GeneTherapeutics,
#[serde(rename = "NEURAL_CHIPS")]
NeuralChips,
#[serde(rename = "MOOD_REGULATORS")]
MoodRegulators,
#[serde(rename = "VIRAL_AGENTS")]
ViralAgents,
#[serde(rename = "MICRO_FUSION_GENERATORS")]
MicrofusionGenerators,
#[serde(rename = "SUPERGRAINS")]
SuperGrains,
#[serde(rename = "LASER_RIFLES")]
LaserRifles,
#[serde(rename = "HOLOGRAPHICS")]
Holographics,
#[serde(rename = "SHIP_SALVAGE")]
ShipSalvage,
#[serde(rename = "RELIC_TECH")]
RelicTech,
#[serde(rename = "NOVEL_LIFEFORMS")]
NovelLifeforms,
#[serde(rename = "BOTANICAL_SPECIMENS")]
BotanicalSpecimens,
#[serde(rename = "CULTURAL_ARTIFACTS")]
CulturalArtifacts,
#[serde(rename = "REACTOR_SOLAR_I")]
ReactorSolarI,
#[serde(rename = "REACTOR_FUSION_I")]
ReactorFusionI,
#[serde(rename = "REACTOR_FISSION_I")]
ReactorFissionI,
#[serde(rename = "REACTOR_CHEMICAL_I")]
ReactorChemicalI,
#[serde(rename = "REACTOR_ANTIMATTER_I")]
ReactorAntimatterI,
#[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
EngineImpulseDriveI,
#[serde(rename = "ENGINE_ION_DRIVE_I")]
EngineIonDriveI,
#[serde(rename = "ENGINE_ION_DRIVE_II")]
EngineIonDriveII,
#[serde(rename = "ENGINE_HYPER_DRIVE_I")]
EngineHyperDriveI,
#[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
ModuleMineralProcessorI,
#[serde(rename = "MODULE_CARGO_HOLD_I")]
ModuleCargoHoldI,
#[serde(rename = "MODULE_CREW_QUARTERS_I")]
ModuleCrewQuartersI,
#[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
ModuleEnvoyQuartersI,
#[serde(rename = "MODULE_PASSENGER_CABIN_I")]
ModulePassengerCabinI,
#[serde(rename = "MODULE_MICRO_REFINERY_I")]
ModuleMicroRefineryI,
#[serde(rename = "MODULE_ORE_REFINERY_I")]
ModuleOreRefineryI,
#[serde(rename = "MODULE_FUEL_REFINERY_I")]
ModuleFuelRefineryI,
#[serde(rename = "MODULE_SCIENCE_LAB_I")]
ModuleScienceLabI,
#[serde(rename = "MODULE_JUMP_DRIVE_I")]
ModuleJumpDriveI,
#[serde(rename = "MODULE_JUMP_DRIVE_II")]
ModuleJumpDriveII,
#[serde(rename = "MODULE_JUMP_DRIVE_III")]
ModuleJumpDriveIII,
#[serde(rename = "MODULE_WARP_DRIVE_I")]
ModuleWarpDriveI,
#[serde(rename = "MODULE_WARP_DRIVE_II")]
ModuleWarpDriveII,
#[serde(rename = "MODULE_WARP_DRIVE_III")]
ModuleWarpDriveIII,
#[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
ModuleShieldGeneratorI,
#[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
ModuleShieldGeneratorII,
#[serde(rename = "MOUNT_GAS_SIPHON_I")]
MountGasSiphonI,
#[serde(rename = "MOUNT_GAS_SIPHON_II")]
MountGasSiphonII,
#[serde(rename = "MOUNT_GAS_SIPHON_III")]
MountGasSiphonIII,
#[serde(rename = "MOUNT_SURVEYOR_I")]
MountSurveyorI,
#[serde(rename = "MOUNT_SURVEYOR_II")]
MountSurveyorII,
#[serde(rename = "MOUNT_SURVEYOR_III")]
MountSurveyorIII,
#[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
MountSensorArrayI,
#[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
MountSensorArrayII,
#[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
MountSensorArrayIII,
#[serde(rename = "MOUNT_MINING_LASER_I")]
MountMiningLaserI,
#[serde(rename = "MOUNT_MINING_LASER_II")]
MountMiningLaserII,
#[serde(rename = "MOUNT_MINING_LASER_III")]
MountMiningLaserIII,
#[serde(rename = "MOUNT_LASER_CANNON_I")]
MountLaserCannonI,
#[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
MountMissileLauncherI,
#[serde(rename = "MOUNT_TURRET_I")]
MountTurretI,
}

impl Display for MarketItemType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        MarketItemType::PreciousStones => write!(f, "Precious Stones"),
        MarketItemType::QuartzSand => write!(f, "Quartz Sand"),
        MarketItemType::SiliconCrystals => write!(f, "Silicon Crystals"),
        MarketItemType::AmmoniaIce => write!(f, "Ammonia Ice"),
        MarketItemType::LiquidHydrogen => write!(f, "Liquid Hydrogen"),
        MarketItemType::LiquidNitrogen => write!(f, "Liquid Nitrogen"),
        MarketItemType::IceWater => write!(f, "Ice Water"),
        MarketItemType::ExoticMatter => write!(f, "Exotic Matter"),
        MarketItemType::AdvancedCircuitry => write!(f, "Advanced Circuitry"),
        MarketItemType::GravitonEmitters => write!(f, "Graviton Emitters"),
        MarketItemType::Iron => write!(f, "Iron"),
        MarketItemType::IronOre => write!(f, "Iron Ore"),
        MarketItemType::Copper => write!(f, "Copper"),
        MarketItemType::CopperOre => write!(f, "Copper Ore"),
        MarketItemType::Aluminum => write!(f, "Aluminum"),
        MarketItemType::AluminumOre => write!(f, "Aluminum Ore"),
        MarketItemType::Silver => write!(f, "Silver"),
        MarketItemType::SilverOre => write!(f, "Silver Ore"),
        MarketItemType::Gold => write!(f, "Gold"),
        MarketItemType::GoldOre => write!(f, "Gold Ore"),
        MarketItemType::Platinum => write!(f, "Platinum"),
        MarketItemType::PlatinumOre => write!(f, "Platinum Ore"),
        MarketItemType::Diamonds => write!(f, "Diamonds"),
        MarketItemType::Uranite => write!(f, "Uranite"),
        MarketItemType::UraniteOre => write!(f, "Uranite Ore"),
        MarketItemType::Meritium => write!(f, "Meritium"),
        MarketItemType::MeritiumOre => write!(f, "Meritium Ore"),
        MarketItemType::Hydrocarbon => write!(f, "Hydrocarbon"),
        MarketItemType::Antimatter => write!(f, "Antimatter"),
        MarketItemType::Fertilizers => write!(f, "Fertilizers"),
        MarketItemType::Fabrics => write!(f, "Fabrics"),
        MarketItemType::Food => write!(f, "Food"),
        MarketItemType::Jewelry => write!(f, "Jewelry"),
        MarketItemType::Machinery => write!(f, "Machinery"),
        MarketItemType::Firearms => write!(f, "Firearms"),
        MarketItemType::AssaultRifles => write!(f, "Assault Rifles"),
        MarketItemType::MilitaryEquipment => write!(f, "Military Equipment"),
        MarketItemType::Explosives => write!(f, "Explosives"),
        MarketItemType::LabInstruments => write!(f, "Lab Instruments"),
        MarketItemType::Ammunition => write!(f, "Ammunition"),
        MarketItemType::Electronics => write!(f, "Electronics"),
        MarketItemType::ShipPlating => write!(f, "Ship Plating"),
        MarketItemType::Equipment => write!(f, "Equipment"),
        MarketItemType::Fuel => write!(f, "Fuel"),
        MarketItemType::Medicine => write!(f, "Medicine"),
        MarketItemType::Drugs => write!(f, "Drugs"),
        MarketItemType::Clothing => write!(f, "Clothing"),
        MarketItemType::Microprocessors => write!(f, "Microprocessors"),
        MarketItemType::Plastics => write!(f, "Plastics"),
        MarketItemType::PolyNeucleotides => write!(f, "Poly Neucleotides"),
        MarketItemType::BioComposites => write!(f, "Bio Composites"),
        MarketItemType::Nanobots => write!(f, "Nanobots"),
        MarketItemType::AiMainframes => write!(f, "Ai Mainframes"),
        MarketItemType::QuantumDrives => write!(f, "Quantum Drives"),
        MarketItemType::RoboticDrones => write!(f, "Robotic Drones"),
        MarketItemType::CyberImplants => write!(f, "Cyber Implants"),
        MarketItemType::GeneTherapeutics => write!(f, "Gene Therapeutics"),
        MarketItemType::NeuralChips => write!(f, "Neural Chips"),
        MarketItemType::MoodRegulators => write!(f, "Mood Regulators"),
        MarketItemType::ViralAgents => write!(f, "Viral Agents"),
        MarketItemType::MicrofusionGenerators => write!(f, "Microfusion Generators"),
        MarketItemType::SuperGrains => write!(f, "Super Grains"),
        MarketItemType::LaserRifles => write!(f, "Laser Rifles"),
        MarketItemType::Holographics => write!(f, "Holographics"),
        MarketItemType::ShipSalvage => write!(f, "Ship Salvage"),
        MarketItemType::RelicTech => write!(f, "Relic Tech"),
        MarketItemType::NovelLifeforms => write!(f, "Novel Lifeforms"),
        MarketItemType::BotanicalSpecimens => write!(f, "Botanical Specimens"),
        MarketItemType::CulturalArtifacts => write!(f, "Cultural Artifacts"),
        MarketItemType::ReactorSolarI => write!(f, "Solar I Reactor"),
        MarketItemType::ReactorFusionI => write!(f, "Fusion I Reactor"),
        MarketItemType::ReactorFissionI => write!(f, "Fission I Reactor"),
        MarketItemType::ReactorChemicalI => write!(f, "Chemical I Reactor"),
        MarketItemType::ReactorAntimatterI => write!(f, "Antimatter I Reactor"),
        MarketItemType::EngineImpulseDriveI => write!(f, "Impulse Drive I Engine"),
        MarketItemType::EngineIonDriveI => write!(f, "Ion Drive I Engine"),
        MarketItemType::EngineIonDriveII => write!(f, "Ion Drive II Engine"),
        MarketItemType::EngineHyperDriveI => write!(f, "Hyper Drive I Engine"),
        MarketItemType::ModuleMineralProcessorI => write!(f, "Mineral ProcessorI"),
        MarketItemType::ModuleCargoHoldI => write!(f, "Cargo Hold I"),
        MarketItemType::ModuleCrewQuartersI => write!(f, "Crew Quarters I"),
        MarketItemType::ModuleEnvoyQuartersI => write!(f, "Envoy Quarters I"),
        MarketItemType::ModulePassengerCabinI => write!(f, "PassengerCabin I"),
        MarketItemType::ModuleMicroRefineryI => write!(f, "Micro Refinery I"),
        MarketItemType::ModuleOreRefineryI => write!(f, "Ore Refinery I"),
        MarketItemType::ModuleFuelRefineryI => write!(f, "Fuel Refinery I"),
        MarketItemType::ModuleScienceLabI => write!(f, "Science Lab I"),
        MarketItemType::ModuleJumpDriveI => write!(f, "Jump Drive I"),
        MarketItemType::ModuleJumpDriveII => write!(f, "Jump Drive II"),
        MarketItemType::ModuleJumpDriveIII => write!(f, "Jump Drive III"),
        MarketItemType::ModuleWarpDriveI => write!(f, "Warp Drive I"),
        MarketItemType::ModuleWarpDriveII => write!(f, "Warp Drive II"),
        MarketItemType::ModuleWarpDriveIII => write!(f, "Warp Drive III"),
        MarketItemType::ModuleShieldGeneratorI => write!(f, "Shield Generator I"),
        MarketItemType::ModuleShieldGeneratorII => write!(f, "Shield Generator II"),
        MarketItemType::MountGasSiphonI => write!(f, "Gas Siphon I"),
        MarketItemType::MountGasSiphonII => write!(f, "Gas Siphon II"),
        MarketItemType::MountGasSiphonIII => write!(f, "GasSiphonIII"),
        MarketItemType::MountSurveyorI => write!(f, "Surveyor I"),
        MarketItemType::MountSurveyorII => write!(f, "Surveyor II"),
        MarketItemType::MountSurveyorIII => write!(f, "Surveyor III"),
        MarketItemType::MountSensorArrayI => write!(f, "Sensor Array I"),
        MarketItemType::MountSensorArrayII => write!(f, "Sensor Array II"),
        MarketItemType::MountSensorArrayIII => write!(f, "Sensor Array III"),
        MarketItemType::MountMiningLaserI => write!(f, "Mining Laser I"),
        MarketItemType::MountMiningLaserII => write!(f, "Mining Laser II"),
        MarketItemType::MountMiningLaserIII => write!(f, "Mining Laser III"),
        MarketItemType::MountLaserCannonI => write!(f, "Laser Cannon I"),
        MarketItemType::MountMissileLauncherI => write!(f, "Missile Launcher I"),
        MarketItemType::MountTurretI => write!(f, "Turret I"),
    }
  }
}