use std::time::Duration;

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

    pub async fn get_v1_status(&self, affinity: &Affinity) -> Result<ValorantApiResponse<V1StatusData>, reqwest::Error> {
        let aff = affinity.to_str();
        let response = self.client
            .get(format!("{BASE_URI}/valorant/v1/status/{aff}"))
            .send()
            .await;
        println!("{:?}", response);
        let json = response?
            .json::<ValorantApiResponse<V1StatusData>>()
            .await;
        Ok(json?)
    }

    pub async fn get_v2_store_offers(&self) -> Result<ValorantApiResponse<V2StoreOffersData>, reqwest::Error> {
        let response = self.client
            .get(format!("{BASE_URI}/valorant/v2/store-offers"))
            .send()
            .await;
        println!("{:?}", response);
        let json = response?
            .json::<ValorantApiResponse<V2StoreOffersData>>()
            .await;
        Ok(json?)
    }

    pub async fn get_v1_version(&self, affinity: &Affinity) -> Result<ValorantApiResponse<V1VersionData>, reqwest::Error> {
        let aff = affinity.to_str();
        let response = self.client
            .get(format!("{BASE_URI}/valorant/v1/version/{aff}"))
            .send()
            .await;
        println!("{:?}", response);
        let json = response?
            .json::<ValorantApiResponse<V1VersionData>>()
            .await;
        Ok(json?)
    }

    pub async fn get_v1_website(&self, country_code: &str) -> Result<ValorantApiResponse<V1WebsiteData>, reqwest::Error> {
        let response = self.client
            .get(format!("{BASE_URI}/valorant/v1/website/{country_code}"))
            .send()
            .await;
        println!("{:?}", response);
        let json = response?
            .json::<ValorantApiResponse<V1WebsiteData>>()
            .await;
        Ok(json?)
    }
}
