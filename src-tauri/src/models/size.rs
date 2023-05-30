use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Size {
  #[serde(rename = "SMALL")]
  Small,
  #[serde(rename = "MEDIUM")]
  Medium,
  #[serde(rename = "LARGE")]
  Large,
}

impl Display for Size {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Size::Small => write!(f, "Small"),
      Size::Medium => write!(f, "Medium"),
      Size::Large => write!(f, "Large"),
    }
  }
}