use std::{fmt::{Display, Formatter}, str::FromStr};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Size {
  #[serde(rename = "SMALL")]
  Small = 1,
  #[serde(rename = "MODERATE")]
  Moderate = 2,
  #[serde(rename = "LARGE")]
  Large = 3,
}

impl Display for Size {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Size::Small => write!(f, "Small"),
      Size::Moderate => write!(f, "Moderate"),
      Size::Large => write!(f, "Large"),
    }
  }
}

impl FromStr for Size {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Large" => Ok(Size::Large),
        "Moderate" => Ok(Size::Moderate),
        "Small" => Ok(Size::Small),
        _ => Err(())
      }
  }
}
