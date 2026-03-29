// Module with wrapper around the monkeytype API

use reqwest::header::AUTHORIZATION;

use crate::{errors::AppResult, models::UserStats};

const API_URL: &str = "https://api.monkeytype.com";

pub struct Api {
    http: reqwest::blocking::Client,
    token: String,
}

impl Api {
    pub fn new(token: &str) -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
            token: token.to_string(),
        }
    }

    pub fn user_stats(&self) -> AppResult<UserStats> {
        let res = self.http
            .get(format!("{API_URL}/users/stats"))
            .header(AUTHORIZATION, format!("ApeKey {}", self.token))
            .send()?;
        let body = res.json()?;
        Ok(body)
    }
}