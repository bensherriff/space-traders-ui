use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
  pub power: Option<i32>,
  pub crew: Option<i32>,
  pub slots: Option<i32>
}