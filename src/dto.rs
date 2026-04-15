#![allow(unused)]

use std::collections::HashMap;

use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    #[serde(alias = "_id")]
    pub id: String,
    pub completed_tests: u32,
    pub started_tests: u32,
    pub time_typing: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBest {
    #[serde(alias = "acc")]
    pub accuracy: f32,
    pub consistency: f32,
    pub raw: f32,
    pub wpm: f32,

    #[serde(flatten, skip)]
    _extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalBests(
    #[serde(deserialize_with = "deserialize_pb_map")] pub HashMap<String, PersonalBest>,
);

fn deserialize_pb_map<'de, D>(deserializer: D) -> Result<HashMap<String, PersonalBest>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: HashMap<String, Vec<PersonalBest>> = HashMap::deserialize(deserializer)?;
    raw.into_iter()
        .map(|(k, mut v)| {
            if v.is_empty() {
                Err(serde::de::Error::custom(format!(
                    "empty array for key '{k}'"
                )))
            } else {
                Ok((k, v.swap_remove(0)))
            }
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    pub wpm: f32,
    pub raw_wpm: f32,
    #[serde(alias = "acc")]
    pub accuracy: f32,
    pub consistency: f32,
    pub mode: String, // TODO: use an enum
    pub mode2: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,

    #[serde(alias = "uid")]
    pub user_id: String,

    #[serde(flatten, skip)]
    _extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub name: String,

    #[serde(flatten, skip)]
    _extra: HashMap<String, serde_json::Value>,
}
