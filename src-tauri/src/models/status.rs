use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
    #[serde(rename = "resetDate")]
    pub reset_date: String,
    pub description: String,
    pub stats: Stats,
    pub leaderboards: Leaderboards,
    #[serde(rename = "serverResets")]
    pub server_resets: ServerResets,
    pub announcements: Vec<Announcement>,
    pub links: Vec<Link>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub agents: i64,
    pub ships: i64,
    pub systems: i64,
    pub waypoints: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboards {
    #[serde(rename = "mostCredits")]
    pub most_credits: Vec<LeaderboardCredits>,
    #[serde(rename = "mostSubmittedCharts")]
    pub most_submitted_charts: Vec<LeaderboardSubmittedCharts>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardCredits {
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    pub credits: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardSubmittedCharts {
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    #[serde(rename = "chartCount")]
    pub chart_count: i64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerResets {
    pub next: String,
    pub frequency: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub title: String,
    pub body: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    name: String,
    url: String
}