use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
  pub power: Option<i64>,
  pub crew: Option<i64>,
  pub slots: Option<i64>
}