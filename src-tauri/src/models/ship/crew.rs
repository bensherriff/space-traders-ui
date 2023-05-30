use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crew {
  pub current: u64,
  pub required: u64,
  pub capacity: u64,
  pub rotation: Rotation,
  pub morale: f64,
  pub wages: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rotation {
  #[serde(rename = "STRICT")]
  Strict,
  #[serde(rename = "RELAXED")]
  Relaxed,
}

impl Display for Rotation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Rotation::Strict => write!(f, "Strict"),
      Rotation::Relaxed => write!(f, "Relaxed"),
    }
  }
}