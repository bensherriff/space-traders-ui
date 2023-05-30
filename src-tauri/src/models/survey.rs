use serde::{Deserialize, Serialize};

use super::{ship::cooldown::Cooldown, size::Size};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Survey {
  pub signature: String,
  pub symbol: String,
  pub deposits: Vec<String>,
  pub expiration: String,
  size: Size
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyResponse {
  pub surveys: Vec<Survey>,
  pub cooldown: Cooldown
}