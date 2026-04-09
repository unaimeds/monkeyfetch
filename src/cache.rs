use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    dto::{PersonalBest, TestResult, UserStats},
    error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub user_stats: UserStats,
    pub personal_bests: HashMap<String, PersonalBest>,
    pub recent_tests: Vec<TestResult>,
}

impl Cache {
    fn from_file(path: String) -> AppResult<Option<Self>> {
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(AppError::Io(e)),
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let parsed = serde_json::from_str(&contents)?;
        Ok(parsed)
    }

    fn to_file(&self, path: String) -> AppResult<()> {
        let json = serde_json::to_string(self)?;
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

pub struct CacheManager {
    cache_dir: String,
}

impl CacheManager {
    pub fn new() -> Self {
        let home_dir = env::var_os("HOME")
            .and_then(|p| p.into_string().ok())
            .unwrap();
        let cache_dir = env::var_os("XDG_CACHE_HOME")
            .and_then(|p| p.into_string().ok())
            .unwrap_or(format!("{home_dir}/.cache"));
        Self { cache_dir }
    }

    pub fn load(&self) -> AppResult<Option<Cache>> {
        let opt = Cache::from_file(format!("{}/monkeyfetch.json", self.cache_dir))?;
        if let Some(data) = opt {
            if Utc::now() - data.timestamp > Duration::minutes(15) {
                Ok(None)
            } else {
                Ok(Some(data))
            }
        } else {
            Ok(opt)
        }
    }

    pub fn save(&self, c: Cache) -> AppResult<Cache> {
        c.to_file(format!("{}/monkeyfetch.json", self.cache_dir))?;
        Ok(c)
    }
}
