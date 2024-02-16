use std::time::Duration;

use url::Url;

use crate::errors::Error;
use crate::types::*;

const BASE_URI: &'static str = "https://api.henrikdev.xyz";
const USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), );

pub struct ValorantApiClient {
    base_url: Url,
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
        let base_url = Url::parse(BASE_URI).expect("could not parse base url");

        ValorantApiClient {
            base_url,
            client,
        }
    }

    pub async fn get_v1_account<S>(&self, name: S, tag: S, force_update: Option<bool>) -> Result<ValorantApiResponse<V1AccountData>, Error>
        where
            S: AsRef<str>
    {
        let mut url = self.base_url
            .join(
                &format!(
                    "/valorant/v1/account/{0}/{1}",
                    name.as_ref(),
                    tag.as_ref(),
                )
            )?;
        match force_update {
            None => {}
            Some(force) => {
                url.query_pairs_mut().append_pair("force", &format!("{force}"));
            }
        }

        let reqwest = self.client
            .get(url)
            .send()
            .await;
        println!("{:?}", reqwest);
        let response = reqwest?
            .json::<ValorantApiResponse<V1AccountData>>()
            .await;
        Ok(response?)
    }

    pub async fn get_v1_status(&self, affinity: &Affinity) -> Result<ValorantApiResponse<V1StatusData>, Error> {
        let url = self.base_url
            .join(&format!("/valorant/v1/status/{affinity}"))?;

        let reqwest = self.client
            .get(url)
            .send()
            .await;
        println!("{:?}", reqwest);
        let response = reqwest?
            .json::<ValorantApiResponse<V1StatusData>>()
            .await?;
        Ok(response)
    }

    pub async fn get_v2_store_offers(&self) -> Result<ValorantApiResponse<V2StoreOffersData>, Error> {
        let url = self.base_url
            .join("/valorant/v2/store-offers")?;

        let reqwest = self.client
            .get(url)
            .send()
            .await;
        println!("{:?}", reqwest);
        let response = reqwest?
            .json::<ValorantApiResponse<V2StoreOffersData>>()
            .await;
        Ok(response?)
    }

    pub async fn get_v1_version(&self, affinity: &Affinity) -> Result<ValorantApiResponse<V1VersionData>, Error> {
        let url = self.base_url
            .join(&format!("/valorant/v1/version/{affinity}"))?;

        let reqwest = self.client
            .get(url)
            .send()
            .await;
        println!("{:?}", reqwest);
        let response = reqwest?
            .json::<ValorantApiResponse<V1VersionData>>()
            .await;
        Ok(response?)
    }

    pub async fn get_v1_website(&self, country_code: &str) -> Result<ValorantApiResponse<V1WebsiteData>, Error> {
        let url = self.base_url
            .join(&format!("/valorant/v1/website/{country_code}"))?;

        let reqwest = self.client
            .get(url)
            .send()
            .await;
        println!("{:?}", reqwest);
        let response = reqwest?
            .json::<ValorantApiResponse<V1WebsiteData>>()
            .await;
        Ok(response?)
    }
}
