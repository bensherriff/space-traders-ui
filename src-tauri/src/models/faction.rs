use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub headquarters: String,
  pub traits: Vec<FactionTrait>,
  #[serde(rename = "isRecruiting")]
  pub is_recruiting: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionTrait {
  pub symbol: FactionTraitType,
  pub name: String,
  pub description: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactionTraitType {
  #[serde(rename = "BUREAUCRATIC")]
  Bureaucratic,
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

impl Display for FactionTraitType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
        FactionTraitType::Bureaucratic => write!(f, "Bureaucratic"),
        FactionTraitType::Secretive => write!(f, "Secretive"),
        FactionTraitType::Capitalistic => write!(f, "Capitalistic"),
        FactionTraitType::Industrious => write!(f, "Industrious"),
        FactionTraitType::Peaceful => write!(f, "Peaceful"),
        FactionTraitType::Distrustful => write!(f, "Distrustful"),
        FactionTraitType::Welcoming => write!(f, "Welcoming"),
        FactionTraitType::Smugglers => write!(f, "Smugglers"),
        FactionTraitType::Scavengers => write!(f, "Scavengers"),
        FactionTraitType::Rebellious => write!(f, "Rebellious"),
        FactionTraitType::Exiles => write!(f, "Exiles"),
        FactionTraitType::Pirates => write!(f, "Pirates"),
        FactionTraitType::Raiders => write!(f, "Raiders"),
        FactionTraitType::Clan => write!(f, "Clan"),
        FactionTraitType::Guild => write!(f, "Guild"),
        FactionTraitType::Dominion => write!(f, "Dominion"),
        FactionTraitType::Fringe => write!(f, "Fringe"),
        FactionTraitType::Forsaken => write!(f, "Forsaken"),
        FactionTraitType::Isolated => write!(f, "Isolated"),
        FactionTraitType::Localized => write!(f, "Localized"),
        FactionTraitType::Established => write!(f, "Established"),
        FactionTraitType::Notable => write!(f, "Notable"),
        FactionTraitType::Dominant => write!(f, "Dominant"),
        FactionTraitType::Inescapable => write!(f, "Inescapable"),
        FactionTraitType::Innovative => write!(f, "Innovative"),
        FactionTraitType::Bold => write!(f, "Bold"),
        FactionTraitType::Visionary => write!(f, "Visionary"),
        FactionTraitType::Curious => write!(f, "Curious"),
        FactionTraitType::Daring => write!(f, "Daring"),
        FactionTraitType::Exploratory => write!(f, "Exploratory"),
        FactionTraitType::Resourceful => write!(f, "Resourceful"),
        FactionTraitType::Flexible => write!(f, "Flexible"),
        FactionTraitType::Cooperative => write!(f, "Cooperative"),
        FactionTraitType::United => write!(f, "United"),
        FactionTraitType::Strategic => write!(f, "Strategic"),
        FactionTraitType::Intelligent => write!(f, "Intelligent"),
        FactionTraitType::ResearchFocused => write!(f, "Research Focused"),
        FactionTraitType::Collaborative => write!(f, "Collaborative"),
        FactionTraitType::Progressive => write!(f, "Progressive"),
        FactionTraitType::Militaristic => write!(f, "Militaristic"),
        FactionTraitType::TechnologicallyAdvanced => write!(f, "Technologically Advanced"),
        FactionTraitType::Aggressive => write!(f, "Aggressive"),
        FactionTraitType::Imperialistic => write!(f, "Imperialistic"),
        FactionTraitType::TreasureHunters => write!(f, "Treasure Hunters"),
        FactionTraitType::Dexterous => write!(f, "Dexterous"),
        FactionTraitType::Unpredictable => write!(f, "Unpredictable"),
        FactionTraitType::Brutal => write!(f, "Brutal"),
        FactionTraitType::Fleeting => write!(f, "Fleeting"),
        FactionTraitType::Adaptable => write!(f, "Adaptable"),
        FactionTraitType::SelfSufficient => write!(f, "Self Sufficient"),
        FactionTraitType::Defensive => write!(f, "Defensive"),
        FactionTraitType::Proud => write!(f, "Proud"),
        FactionTraitType::Diverse => write!(f, "Diverse"),
        FactionTraitType::Independent => write!(f, "Independent"),
        FactionTraitType::SelfInterested => write!(f, "Self Interested"),
        FactionTraitType::Fragmented => write!(f, "Fragmented"),
        FactionTraitType::Commercial => write!(f, "Commercial"),
        FactionTraitType::FreeMarkets => write!(f, "Free Markets"),
        FactionTraitType::Entrepreneurial => write!(f, "Entrepreneurial"),
    }
  }
}