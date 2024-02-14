use std::collections::HashMap;
use std::time::Duration;

use serde_json::Value;

use crate::types::*;

const BASE_URI: &'static str = "https://api.henrikdev.xyz";
const USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), );

pub struct ValorantApiClient {
    client: reqwest::Client,
}

impl ValorantApiClient {
    pub fn new() -> ValorantApiClient {
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert(reqwest::header::ACCEPT, reqwest::header::HeaderValue::from_static(USER_AGENT));
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .gzip(true)
            .brotli(true)
            .deflate(true)
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(5))
            .connection_verbose(true)
            .https_only(true)
            .build()
            .unwrap();

        ValorantApiClient {
            client,
        }
    }

    pub async fn get_status(&self, affinity: &Affinity) -> Result<HashMap<String, Value>, reqwest::Error> {
        let aff = affinity.to_str();
        let response = self.client
            .get(format!("{BASE_URI}/valorant/v1/status/{aff}"))
            .send()
            .await;
        println!("{:?}", response);
        let json = response?
            .json::<HashMap<String, Value>>()
            .await;
        Ok(json?)
    }
}
