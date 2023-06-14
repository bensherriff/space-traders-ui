use std::{fmt::{Display, Formatter}, str::FromStr};

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

impl Display for TraitType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TraitType::Uncharted => write!(f, "Uncharted"),
      TraitType::Marketplace => write!(f, "Marketplace"),
      TraitType::Shipyard => write!(f, "Shipyard"),
      TraitType::Outpost => write!(f, "Outpost"),
      TraitType::ScatteredSettlements => write!(f, "Scattered Scattered Settlements"),
      TraitType::SprawlingCities => write!(f, "Sprawling Cities"),
      TraitType::MegaStructures => write!(f, "Mega Structures"),
      TraitType::Overcrowded => write!(f, "Overcrowded"),
      TraitType::HighTech => write!(f, "High Tech"),
      TraitType::Corrupt => write!(f, "Corrupt"),
      TraitType::Bureaucratic => write!(f, "Bureaucratic"),
      TraitType::TradingHub => write!(f, "Trading Hub"),
      TraitType::Industrial => write!(f, "Industrial"),
      TraitType::BlackMarket => write!(f, "Black Market"),
      TraitType::ResearchFacility => write!(f, "Research Facility"),
      TraitType::MilitaryBase => write!(f, "Military Base"),
      TraitType::SurveillanceOutpost => write!(f, "Surveillance Outpost"),
      TraitType::ExplorationOutpost => write!(f, "Exploration Outpost"),
      TraitType::MineralDeposits => write!(f, "Mineral Deposits"),
      TraitType::CommonMetalDeposits => write!(f, "Common Metal Deposits"),
      TraitType::PreciousMetalDeposits => write!(f, "Precious Metal Deposits"),
      TraitType::RareMetalDeposits => write!(f, "Rare Metal Deposits"),
      TraitType::MethanePools => write!(f, "Methane Pools"),
      TraitType::IceCrystals => write!(f, "Ice Crystals"),
      TraitType::ExplosiveGases => write!(f, "Explosive Gases"),
      TraitType::StrongMagnetosphere => write!(f, "Strong Magnetosphere"),
      TraitType::VibrantAuroras => write!(f, "Vibrant Auroras"),
      TraitType::SaltFlats => write!(f, "Salt Flats"),
      TraitType::Canyons => write!(f, "Canyons"),
      TraitType::PerpetualDaylight => write!(f, "Perpetual Daylight"),
      TraitType::PerpetualOvercast => write!(f, "Perpetual Overcast"),
      TraitType::DrySeabeds => write!(f, "Dry Seabeds"),
      TraitType::MagmaSeas => write!(f, "Magma Seas"),
      TraitType::SuperVolcanoes => write!(f, "Super Volcanoes"),
      TraitType::AshClouds => write!(f, "Ash Clouds"),
      TraitType::VastRuins => write!(f, "Vast Ruins"),
      TraitType::MutatedFlora => write!(f, "Mutated Flora"),
      TraitType::Terraformed => write!(f, "Terraformed"),
      TraitType::ExtremeTemperatures => write!(f, "Extreme Temperatures"),
      TraitType::ExtremePressure => write!(f, "Extreme Pressure"),
      TraitType::DiverseLife => write!(f, "Diverse Life"),
      TraitType::ScarceLife => write!(f, "Scarce Life"),
      TraitType::Fossils => write!(f, "Fossils"),
      TraitType::WeakGravity => write!(f, "Weak Gravity"),
      TraitType::StrongGravity => write!(f, "Strong Gravity"),
      TraitType::CrushingGravity => write!(f, "Crushing Gravity"),
      TraitType::ToxicAtmosphere => write!(f, "Toxic Atmosphere"),
      TraitType::CorrosiveAtmosphere => write!(f, "Corrosive Atmosphere"),
      TraitType::BreathableAtmosphere => write!(f, "Breathable Atmosphere"),
      TraitType::Jovian => write!(f, "Jovian"),
      TraitType::Rocky => write!(f, "Rocky"),
      TraitType::Volcanic => write!(f, "Volcanic"),
      TraitType::Frozen => write!(f, "Frozen"),
      TraitType::Swamp => write!(f, "Swamp"),
      TraitType::Barren => write!(f, "Barren"),
      TraitType::Temperate => write!(f, "Temperate"),
      TraitType::Jungle => write!(f, "Jungle"),
      TraitType::Ocean => write!(f, "Ocean"),
      TraitType::Stripped => write!(f, "Stripped"),
      TraitType::Secretive => write!(f, "Secretive"),
      TraitType::Capitalistic => write!(f, "Capitalistic"),
      TraitType::Industrious => write!(f, "Industrious"),
      TraitType::Peaceful => write!(f, "Peaceful"),
      TraitType::Distrustful => write!(f, "Distrustful"),
      TraitType::Welcoming => write!(f, "Welcoming"),
      TraitType::Smugglers => write!(f, "Smugglers"),
      TraitType::Scavengers => write!(f, "Scavengers"),
      TraitType::Rebellious => write!(f, "Rebellious"),
      TraitType::Exiles => write!(f, "Exiles"),
      TraitType::Pirates => write!(f, "Pirates"),
      TraitType::Raiders => write!(f, "Raiders"),
      TraitType::Clan => write!(f, "Clan"),
      TraitType::Guild => write!(f, "Guild"),
      TraitType::Dominion => write!(f, "Dominion"),
      TraitType::Fringe => write!(f, "Fringe"),
      TraitType::Forsaken => write!(f, "Forsaken"),
      TraitType::Isolated => write!(f, "Isolated"),
      TraitType::Localized => write!(f, "Localized"),
      TraitType::Established => write!(f, "Established"),
      TraitType::Notable => write!(f, "Notable"),
      TraitType::Dominant => write!(f, "Dominant"),
      TraitType::Inescapable => write!(f, "Inescapable"),
      TraitType::Innovative => write!(f, "Innovative"),
      TraitType::Bold => write!(f, "Bold"),
      TraitType::Visionary => write!(f, "Visionary"),
      TraitType::Curious => write!(f, "Curious"),
      TraitType::Daring => write!(f, "Daring"),
      TraitType::Exploratory => write!(f, "Exploratory"),
      TraitType::Resourceful => write!(f, "Resourceful"),
      TraitType::Flexible => write!(f, "Flexible"),
      TraitType::Cooperative => write!(f, "Cooperative"),
      TraitType::United => write!(f, "United"),
      TraitType::Strategic => write!(f, "Strategic"),
      TraitType::Intelligent => write!(f, "Intelligent"),
      TraitType::ResearchFocused => write!(f, "Research Focused"),
      TraitType::Collaborative => write!(f, "Collaborative"),
      TraitType::Progressive => write!(f, "Progressive"),
      TraitType::Militaristic => write!(f, "Militaristic"),
      TraitType::TechnologicallyAdvanced => write!(f, "Technologically Advanced"),
      TraitType::Aggressive => write!(f, "Aggressive"),
      TraitType::Imperialistic => write!(f, "Imperialistic"),
      TraitType::TreasureHunters => write!(f, "Treasure Hunters"),
      TraitType::Dexterous => write!(f, "Dexterous"),
      TraitType::Unpredictable => write!(f, "Unpredictable"),
      TraitType::Brutal => write!(f, "Brutal"),
      TraitType::Fleeting => write!(f, "Fleeting"),
      TraitType::Adaptable => write!(f, "Adaptable"),
      TraitType::SelfSufficient => write!(f, "Self Sufficient"),
      TraitType::Defensive => write!(f, "Defensive"),
      TraitType::Proud => write!(f, "Proud"),
      TraitType::Diverse => write!(f, "Diverse"),
      TraitType::Independent => write!(f, "Independent"),
      TraitType::SelfInterested => write!(f, "Self Interested"),
      TraitType::Fragmented => write!(f, "Fragmented"),
      TraitType::Commercial => write!(f, "Commercial"),
      TraitType::FreeMarkets => write!(f, "Free Markets"),
      TraitType::Entrepreneurial => write!(f, "Entrepreneurial"),
    }
  }
}

impl FromStr for TraitType {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Uncharted" => Ok(TraitType::Uncharted),
        "Marketplace" => Ok(TraitType::Marketplace),
        "Shipyard" => Ok(TraitType::Shipyard),
        "Outpost" => Ok(TraitType::Outpost),
        "Scattered Scattered Settlements" => Ok(TraitType::ScatteredSettlements),
        "Sprawling Cities" => Ok(TraitType::SprawlingCities),
        "Mega Structures" => Ok(TraitType::MegaStructures),
        "Overcrowded" => Ok(TraitType::Overcrowded),
        "High Tech" => Ok(TraitType::HighTech),
        "Corrupt" => Ok(TraitType::Corrupt),
        "Bureaucratic" => Ok(TraitType::Bureaucratic),
        "Trading Hub" => Ok(TraitType::TradingHub),
        "Industrial" => Ok(TraitType::Industrial),
        "Black Market" => Ok(TraitType::BlackMarket),
        "Research Facility" => Ok(TraitType::ResearchFacility),
        "Military Base" => Ok(TraitType::MilitaryBase),
        "Surveillance Outpost" => Ok(TraitType::SurveillanceOutpost),
        "Exploration Outpost" => Ok(TraitType::ExplorationOutpost),
        "Mineral Deposits" => Ok(TraitType::MineralDeposits),
        "Common Metal Deposits" => Ok(TraitType::CommonMetalDeposits),
        "Precious Metal Deposits" => Ok(TraitType::PreciousMetalDeposits),
        "Rare Metal Deposits" => Ok(TraitType::RareMetalDeposits),
        "Methane Pools" => Ok(TraitType::MethanePools),
        "Ice Crystals" => Ok(TraitType::IceCrystals),
        "Explosive Gases" => Ok(TraitType::ExplosiveGases),
        "Strong Magnetosphere" => Ok(TraitType::StrongMagnetosphere),
        "Vibrant Auroras" => Ok(TraitType::VibrantAuroras),
        "Salt Flats" => Ok(TraitType::SaltFlats),
        "Canyons" => Ok(TraitType::Canyons),
        "Perpetual Daylight" => Ok(TraitType::PerpetualDaylight),
        "Perpetual Overcast" => Ok(TraitType::PerpetualOvercast),
        "Dry Seabeds" => Ok(TraitType::DrySeabeds),
        "Magma Seas" => Ok(TraitType::MagmaSeas),
        "Super Volcanoes" => Ok(TraitType::SuperVolcanoes),
        "Ash Clouds" => Ok(TraitType::AshClouds),
        "Vast Ruins" => Ok(TraitType::VastRuins),
        "Mutated Flora" => Ok(TraitType::MutatedFlora),
        "Terraformed" => Ok(TraitType::Terraformed),
        "Extreme Temperatures" => Ok(TraitType::ExtremeTemperatures),
        "Extreme Pressure" => Ok(TraitType::ExtremePressure),
        "Diverse Life" => Ok(TraitType::DiverseLife),
        "Scarce Life" => Ok(TraitType::ScarceLife),
        "Fossils" => Ok(TraitType::Fossils),
        "Weak Gravity" => Ok(TraitType::WeakGravity),
        "Strong Gravity" => Ok(TraitType::StrongGravity),
        "Crushing Gravity" => Ok(TraitType::CrushingGravity),
        "Toxic Atmosphere" => Ok(TraitType::ToxicAtmosphere),
        "Corrosive Atmosphere" => Ok(TraitType::CorrosiveAtmosphere),
        "Breathable Atmosphere" => Ok(TraitType::BreathableAtmosphere),
        "Jovian" => Ok(TraitType::Jovian),
        "Rocky" => Ok(TraitType::Rocky),
        "Volcanic" => Ok(TraitType::Volcanic),
        "Frozen" => Ok(TraitType::Frozen),
        "Swamp" => Ok(TraitType::Swamp),
        "Barren" => Ok(TraitType::Barren),
        "Temperate" => Ok(TraitType::Temperate),
        "Jungle" => Ok(TraitType::Jungle),
        "Ocean" => Ok(TraitType::Ocean),
        "Stripped" => Ok(TraitType::Stripped),
        "Secretive" => Ok(TraitType::Secretive),
        "Capitalistic" => Ok(TraitType::Capitalistic),
        "Industrious" => Ok(TraitType::Industrious),
        "Peaceful" => Ok(TraitType::Peaceful),
        "Distrustful" => Ok(TraitType::Distrustful),
        "Welcoming" => Ok(TraitType::Welcoming),
        "Smugglers" => Ok(TraitType::Smugglers),
        "Scavengers" => Ok(TraitType::Scavengers),
        "Rebellious" => Ok(TraitType::Rebellious),
        "Exiles" => Ok(TraitType::Exiles),
        "Pirates" => Ok(TraitType::Pirates),
        "Raiders" => Ok(TraitType::Raiders),
        "Clan" => Ok(TraitType::Clan),
        "Guild" => Ok(TraitType::Guild),
        "Dominion" => Ok(TraitType::Dominion),
        "Fringe" => Ok(TraitType::Fringe),
        "Forsaken" => Ok(TraitType::Forsaken),
        "Isolated" => Ok(TraitType::Isolated),
        "Localized" => Ok(TraitType::Localized),
        "Established" => Ok(TraitType::Established),
        "Notable" => Ok(TraitType::Notable),
        "Dominant" => Ok(TraitType::Dominant),
        "Inescapable" => Ok(TraitType::Inescapable),
        "Innovative" => Ok(TraitType::Innovative),
        "Bold" => Ok(TraitType::Bold),
        "Visionary" => Ok(TraitType::Visionary),
        "Curious" => Ok(TraitType::Curious),
        "Daring" => Ok(TraitType::Daring),
        "Exploratory" => Ok(TraitType::Exploratory),
        "Resourceful" => Ok(TraitType::Resourceful),
        "Flexible" => Ok(TraitType::Flexible),
        "Cooperative" => Ok(TraitType::Cooperative),
        "United" => Ok(TraitType::United),
        "Strategic" => Ok(TraitType::Strategic),
        "Intelligent" => Ok(TraitType::Intelligent),
        "Research Focused" => Ok(TraitType::ResearchFocused),
        "Collaborative" => Ok(TraitType::Collaborative),
        "Progressive" => Ok(TraitType::Progressive),
        "Militaristic" => Ok(TraitType::Militaristic),
        "Technologically Advanced" => Ok(TraitType::TechnologicallyAdvanced),
        "Aggressive" => Ok(TraitType::Aggressive),
        "Imperialistic" => Ok(TraitType::Imperialistic),
        "Treasure Hunters" => Ok(TraitType::TreasureHunters),
        "Dexterous" => Ok(TraitType::Dexterous),
        "Unpredictable" => Ok(TraitType::Unpredictable),
        "Brutal" => Ok(TraitType::Brutal),
        "Fleeting" => Ok(TraitType::Fleeting),
        "Adaptable" => Ok(TraitType::Adaptable),
        "Self Sufficient" => Ok(TraitType::SelfSufficient),
        "Defensive" => Ok(TraitType::Defensive),
        "Proud" => Ok(TraitType::Proud),
        "Diverse" => Ok(TraitType::Diverse),
        "Independent" => Ok(TraitType::Independent),
        "Self Interested" => Ok(TraitType::SelfInterested),
        "Fragmented" => Ok(TraitType::Fragmented),
        "Commercial" => Ok(TraitType::Commercial),
        "Free Markets" => Ok(TraitType::FreeMarkets),
        "Entrepreneurial" => Ok(TraitType::Entrepreneurial),
        _ => Err(())
      }
  }
}