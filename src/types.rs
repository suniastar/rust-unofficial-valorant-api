use chrono::{DateTime, FixedOffset};
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
pub struct Localized<'l, T> {
    #[serde(rename = "content")]
    pub content: T,

    #[serde(rename = "locale")]
    pub locale: &'l str,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Status<'l> {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "archived_at")]
    pub archived: Option<DateTime<FixedOffset>>,

    #[serde(rename = "incident_severity")]
    pub incident_severity: Option<&'l str>,

    #[serde(rename = "maintenance_status")]
    pub maintenance_status: &'l str,

    #[serde(rename = "titles")]
    pub titles: Vec<Localized<'l, &'l str>>,

    #[serde(rename = "platforms")]
    pub platforms: Vec<&'l str>,

    #[serde(rename = "updates")]
    pub updates: Vec<StatusUpdate<'l>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusUpdate<'l> {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "publish")]
    pub publish: bool,

    #[serde(rename = "author")]
    pub author: &'l str,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "publish_locations")]
    pub publish_locations: Vec<&'l str>,

    #[serde(rename = "translations")]
    pub translations: Vec<Localized<'l, &'l str>>,
}
