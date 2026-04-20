use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;

use crate::{
    dto::{ApiResponse, PersonalBests, TestResult, UserProfile, UserStats},
    error::AppResult,
};

const API_URL: &str = "https://api.monkeytype.com";

/// Wraps HTTP client and abstracts Monkeytype API endpoints
pub struct Api {
    http: reqwest::blocking::Client,
    token: String,
}

impl Api {
    /// Returns a new API wrapper instance for given token.
    ///
    /// # Arguments
    ///
    /// * `token` - A user-generated ApeKey used to authorize HTTP requests.
    pub fn new(token: &str) -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
            token: token.to_string(),
        }
    }

    /// Fetches current user's personal stats.
    pub fn user_stats(&self) -> AppResult<UserStats> {
        self.get("/users/stats", &[])
    }

    /// Fetches current user's personal bests.
    pub fn personal_bests(&self) -> AppResult<PersonalBests> {
        self.get("/users/personalBests", &[("mode", "time")])
    }

    /// Fetches current user's test results.
    pub fn test_results(&self) -> AppResult<Vec<TestResult>> {
        self.get("/results", &[("limit", "5")])
    }

    /// Fetches current user's username.
    pub fn username(&self) -> AppResult<String> {
        // unaimeds: this is the only way I found to fetch username using API.
        // All test results contain `uid` field which is the UID of current user,
        // we can then use it to fetch user's profile by UID.
        let test = self.get::<TestResult>("/results/last", &[])?;
        let profile = self.get::<UserProfile>(
            &format!("/users/{}/profile", test.user_id),
            &[("isUid", "true")],
        )?;
        Ok(profile.name)
    }

    /// Sends a GET request to API's endpoint with given query.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to which the request should be sent. Must start with '/'.
    /// * `query` - A slice containing query parameters to be sent with request. Can be empty.
    fn get<T>(&self, endpoint: &str, query: &[(&str, &str)]) -> AppResult<T>
    where
        T: DeserializeOwned,
    {
        let res = self
            .http
            .get(format!("{API_URL}{endpoint}"))
            .query(query)
            .header(AUTHORIZATION, format!("ApeKey {}", self.token))
            .send()?;
        let body = res.json::<ApiResponse<T>>()?;
        Ok(body.data)
    }
}
