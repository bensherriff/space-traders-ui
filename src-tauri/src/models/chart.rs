use serde::{Serialize, Deserialize};

use super::waypoint::Waypoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
  #[serde(rename= "waypointSymbol")]
  pub waypoint: Option<String>,
  #[serde(rename = "submittedBy")]
  pub submitted_by: Option<String>,
  #[serde(rename = "submittedOn")]
  pub submitted_on: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartResponse {
  pub chart: Chart,
  pub waypoint: Waypoint
}