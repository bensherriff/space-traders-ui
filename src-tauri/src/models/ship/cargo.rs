use serde::{Deserialize, Serialize};

use super::cooldown::Cooldown;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cargo {
  pub capacity: i32,
  pub units: i32,
  pub inventory: Vec<CargoItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItem {
  pub symbol: String,
  pub name: String,
  pub description: String,
  pub units: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoWrapper {
  pub cargo: Cargo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoRefinement {
  pub cargo: Cargo,
  pub cooldown: Cooldown,
  pub produced: Vec<CargoItemRefinement>,
  pub consumed: Vec<CargoItemRefinement>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoItemRefinement {
  #[serde(rename = "tradeSymbol")]
  pub trade_symbol: String,
  pub units: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedCargo {
  pub cooldown: Cooldown,
  pub extraction: Extraction,
  pub cargo: Cargo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extraction {
  #[serde(rename = "shipSymbol")]
  pub ship_symbol: String,
  #[serde(rename = "yield")]
  pub extraction_yield: ExtractionYield
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionYield {
  pub symbol: String,
  pub units: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoResponse {
  pub cargo: Cargo
}