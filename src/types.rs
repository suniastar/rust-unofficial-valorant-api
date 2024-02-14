use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Affinity {
    Europe,
    NorthAmerica,
    LatinAmerica,
    Brazil,
    AsiaPacific,
    Korea,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Map {
    Ascent,
    Bind,
    Breeze,
    District,
    Fracture,
    Haven,
    Icebox,
    Kasbah,
    Lotus,
    Pearl,
    Piazza,
    Split,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Competitive,
    Unrated,
    Deathmatch,
    TeamDeathmatch,
    Swiftplay,
    SpikeRush,
    NewMap,
    SnowballFight,
    CustomGame,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    PC,
    Console,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Europe,
    NorthAmerica,
    AsiaPacific,
    Korea,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Localized<T> {
    #[serde(rename = "content")]
    pub content: T,

    #[serde(rename = "locale")]
    pub locale: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "archive_at")]
    pub archived: Option<DateTime<FixedOffset>>,

    #[serde(rename = "incident_severity")]
    pub incident_severity: Option<String>,

    #[serde(rename = "maintenance_status")]
    pub maintenance_status: String,

    #[serde(rename = "titles")]
    pub titles: Vec<Localized<String>>,

    #[serde(rename = "platforms")]
    pub platforms: Vec<String>,

    #[serde(rename = "updates")]
    pub updates: Vec<StatusUpdate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusData {
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<Status>,

    #[serde(rename = "incidents")]
    pub incidents: Vec<Status>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusUpdate {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "publish")]
    pub publish: bool,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "publish_locations")]
    pub publish_locations: Vec<String>,

    #[serde(rename = "translations")]
    pub translations: Vec<Localized<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionData {
    #[serde(rename = "region")]
    pub region: Region,

    #[serde(rename = "branch")]
    pub branch: String,

    #[serde(rename = "build_date", serialize_with = "crate::types_serde::serialize_naive_date", deserialize_with = "crate::types_serde::deserialize_naive_date")]
    pub build_date: NaiveDate,

    #[serde(rename = "build_ver")]
    pub build_version: String,

    #[serde(rename = "last_checked")]
    pub last_checked: DateTime<FixedOffset>,

    #[serde(rename = "version")]
    pub version: u32,

    #[serde(rename = "version_for_api")]
    pub version_for_api: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Website {
    #[serde(rename = "banner_url")]
    pub banner_url: String,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "date")]
    pub date: DateTime<FixedOffset>,

    #[serde(rename = "external_link")]
    pub external_link: Option<String>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,
}

pub type WebsiteData = Vec<Website>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValorantApiError {
    #[serde(rename = "code")]
    pub code: u32,

    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "details")]
    pub details: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValorantApiResponse<T> {
    #[serde(rename = "status")]
    pub status: u16,

    #[serde(rename = "errors")]
    pub errors: Option<Vec<ValorantApiError>>,

    #[serde(rename = "data")]
    pub data: Option<T>,
}
