use std::{fmt::{Display, Formatter}, str::FromStr};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crew {
  pub current: i32,
  pub required: i32,
  pub capacity: i32,
  pub rotation: Rotation,
  pub morale: f32,
  pub wages: i32
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

impl FromStr for Rotation {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s {
        "Strict" => Ok(Rotation::Strict),
        "Relaxed" => Ok(Rotation::Relaxed),
        _ => Err(())
      }
  }
}