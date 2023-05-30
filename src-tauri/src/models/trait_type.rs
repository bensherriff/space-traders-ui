use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TraitType {
  #[serde(rename = "UNCHARTED")]
  Uncharted,
  #[serde(rename = "MARKETPLACE")]
  Marketplace,
  #[serde(rename = "SHIPYARD")]
  Shipyard,
  #[serde(rename = "OUTPOST")]
  Outpost,
  #[serde(rename = "SCATTERED_SETTLEMENTS")]
  ScatteredSettlements,
  #[serde(rename = "SPRAWLING_CITIES")]
  SprawlingCities,
  #[serde(rename = "MEGA_STRUCTURES")]
  MegaStructures,
  #[serde(rename = "OVERCROWDED")]
  Overcrowded,
  #[serde(rename = "HIGH_TECH")]
  HighTech,
  #[serde(rename = "CORRUPT")]
  Corrupt,
  #[serde(rename = "BUREAUCRATIC")]
  Bureaucratic,
  #[serde(rename = "TRADING_HUB")]
  TradingHub,
  #[serde(rename = "INDUSTRIAL")]
  Industrial,
  #[serde(rename = "BLACK_MARKET")]
  BlackMarket,
  #[serde(rename = "RESEARCH_FACILITY")]
  ResearchFacility,
  #[serde(rename = "MILITARY_BASE")]
  MilitaryBase,
  #[serde(rename = "SURVEILLANCE_OUTPOST")]
  SurveillanceOutpost,
  #[serde(rename = "EXPLORATION_OUTPOST")]
  ExplorationOutpost,
  #[serde(rename = "MINERAL_DEPOSITS")]
  MineralDeposits,
  #[serde(rename = "COMMON_METAL_DEPOSITS")]
  CommonMetalDeposits,
  #[serde(rename = "PRECIOUS_METAL_DEPOSITS")]
  PreciousMetalDeposits,
  #[serde(rename = "RARE_METAL_DEPOSITS")]
  RareMetalDeposits,
  #[serde(rename = "METHANE_POOLS")]
  MethanePools,
  #[serde(rename = "ICE_CRYSTALS")]
  IceCrystals,
  #[serde(rename = "EXPLOSIVE_GASES")]
  ExplosiveGases,
  #[serde(rename = "STRONG_MAGNETOSPHERE")]
  StrongMagnetosphere,
  #[serde(rename = "VIBRANT_AURORAS")]
  VibrantAuroras,
  #[serde(rename = "SALT_FLATS")]
  SaltFlats,
  #[serde(rename = "CANYONS")]
  Canyons,
  #[serde(rename = "PERPETUAL_DAYLIGHT")]
  PerpetualDaylight,
  #[serde(rename = "PERPETUAL_OVERCAST")]
  PerpetualOvercast,
  #[serde(rename = "DRY_SEABEDS")]
  DrySeabeds,
  #[serde(rename = "MAGMA_SEAS")]
  MagmaSeas,
  #[serde(rename = "SUPERVOLCANOES")]
  SuperVolcanoes,
  #[serde(rename = "ASH_CLOUDS")]
  AshClouds,
  #[serde(rename = "VAST_RUINS")]
  VastRuins,
  #[serde(rename = "MUTATED_FLORA")]
  MutatedFlora,
  #[serde(rename = "TERRAFORMED")]
  Terraformed,
  #[serde(rename = "EXTREME_TEMPERATURES")]
  ExtremeTemperatures,
  #[serde(rename = "EXTREME_PRESSURE")]
  ExtremePressure,
  #[serde(rename = "DIVERSE_LIFE")]
  DiverseLife,
  #[serde(rename = "SCARCE_LIFE")]
  ScarceLife,
  #[serde(rename = "FOSSILS")]
  Fossils,
  #[serde(rename = "WEAK_GRAVITY")]
  WeakGravity,
  #[serde(rename = "STRONG_GRAVITY")]
  StrongGravity,
  #[serde(rename = "CRUSHING_GRAVITY")]
  CrushingGravity,
  #[serde(rename = "TOXIC_ATMOSPHERE")]
  ToxicAtmosphere,
  #[serde(rename = "CORROSIVE_ATMOSPHERE")]
  CorrosiveAtmosphere,
  #[serde(rename = "BREATHABLE_ATMOSPHERE")]
  BreathableAtmosphere,
  #[serde(rename = "JOVIAN")]
  Jovian,
  #[serde(rename = "ROCKY")]
  Rocky,
  #[serde(rename = "VOLCANIC")]
  Volcanic,
  #[serde(rename = "FROZEN")]
  Frozen,
  #[serde(rename = "SWAMP")]
  Swamp,
  #[serde(rename = "BARREN")]
  Barren,
  #[serde(rename = "TEMPERATE")]
  Temperate,
  #[serde(rename = "JUNGLE")]
  Jungle,
  #[serde(rename = "OCEAN")]
  Ocean,
  #[serde(rename = "STRIPPED")]
  Stripped,
  #[serde(rename = "SECRETIVE")]
  Secretive,
  #[serde(rename = "CAPITALISTIC")]
  Capitalistic,
  #[serde(rename = "INDUSTRIOUS")]
  Industrious,
  #[serde(rename = "PEACEFUL")]
  Peaceful,
  #[serde(rename = "DISTRUSTFUL")]
  Distrustful,
  #[serde(rename = "WELCOMING")]
  Welcoming,
  #[serde(rename = "SMUGGLERS")]
  Smugglers,
  #[serde(rename = "SCAVENGERS")]
  Scavengers,
  #[serde(rename = "REBELLIOUS")]
  Rebellious,
  #[serde(rename = "EXILES")]
  Exiles,
  #[serde(rename = "PIRATES")]
  Pirates,
  #[serde(rename = "RAIDERS")]
  Raiders,
  #[serde(rename = "CLAN")]
  Clan,
  #[serde(rename = "GUILD")]
  Guild,
  #[serde(rename = "DOMINION")]
  Dominion,
  #[serde(rename = "FRINGE")]
  Fringe,
  #[serde(rename = "FORSAKEN")]
  Forsaken,
  #[serde(rename = "ISOLATED")]
  Isolated,
  #[serde(rename = "LOCALIZED")]
  Localized,
  #[serde(rename = "ESTABLISHED")]
  Established,
  #[serde(rename = "NOTABLE")]
  Notable,
  #[serde(rename = "DOMINANT")]
  Dominant,
  #[serde(rename = "INESCAPABLE")]
  Inescapable,
  #[serde(rename = "INNOVATIVE")]
  Innovative,
  #[serde(rename = "BOLD")]
  Bold,
  #[serde(rename = "VISIONARY")]
  Visionary,
  #[serde(rename = "CURIOUS")]
  Curious,
  #[serde(rename = "DARING")]
  Daring,
  #[serde(rename = "EXPLORATORY")]
  Exploratory,
  #[serde(rename = "RESOURCEFUL")]
  Resourceful,
  #[serde(rename = "FLEXIBLE")]
  Flexible,
  #[serde(rename = "COOPERATIVE")]
  Cooperative,
  #[serde(rename = "UNITED")]
  United,
  #[serde(rename = "STRATEGIC")]
  Strategic,
  #[serde(rename = "INTELLIGENT")]
  Intelligent,
  #[serde(rename = "RESEARCH_FOCUSED")]
  ResearchFocused,
  #[serde(rename = "COLLABORATIVE")]
  Collaborative,
  #[serde(rename = "PROGRESSIVE")]
  Progressive,
  #[serde(rename = "MILITARISTIC")]
  Militaristic,
  #[serde(rename = "TECHNOLOGICALLY_ADVANCED")]
  TechnologicallyAdvanced,
  #[serde(rename = "AGGRESSIVE")]
  Aggressive,
  #[serde(rename = "IMPERIALISTIC")]
  Imperialistic,
  #[serde(rename = "TREASURE_HUNTERS")]
  TreasureHunters,
  #[serde(rename = "DEXTEROUS")]
  Dexterous,
  #[serde(rename = "UNPREDICTABLE")]
  Unpredictable,
  #[serde(rename = "BRUTAL")]
  Brutal,
  #[serde(rename = "FLEETING")]
  Fleeting,
  #[serde(rename = "ADAPTABLE")]
  Adaptable,
  #[serde(rename = "SELF_SUFFICIENT")]
  SelfSufficient,
  #[serde(rename = "DEFENSIVE")]
  Defensive,
  #[serde(rename = "PROUD")]
  Proud,
  #[serde(rename = "DIVERSE")]
  Diverse,
  #[serde(rename = "INDEPENDENT")]
  Independent,
  #[serde(rename = "SELF_INTERESTED")]
  SelfInterested,
  #[serde(rename = "FRAGMENTED")]
  Fragmented,
  #[serde(rename = "COMMERCIAL")]
  Commercial,
  #[serde(rename = "FREE_MARKETS")]
  FreeMarkets,
  #[serde(rename = "ENTREPRENEURIAL")]
  Entrepreneurial
}