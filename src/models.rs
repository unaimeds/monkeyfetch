// Module defining monkeytype API response schemas

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatsData {
    #[serde(rename = "_id")]
    pub id: String,
    pub completed_tests: u32,
    pub started_tests: u32,
    pub time_typing: f32,
}

#[derive(Debug, Deserialize)]
pub struct UserStats {
    pub message: String,
    pub data: UserStatsData,
}