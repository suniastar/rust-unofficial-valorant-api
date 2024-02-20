use std::fmt::Formatter;
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

use crate::types::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// ENUMS
////////////////////////////////////////////////////////////////////////////////////////////////////

struct AffinityVisitor;

impl<'de> Visitor<'de> for AffinityVisitor {
    type Value = Affinity;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid affinity")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Affinity::from_str(v).map_err(E::custom)
    }
}

impl Serialize for Affinity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Affinity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(AffinityVisitor)
    }
}

struct MapVisitor;

impl<'de> Visitor<'de> for MapVisitor {
    type Value = Map;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid map")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Map::from_str(v).map_err(E::custom)
    }
}

impl Serialize for Map {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Map {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(MapVisitor)
    }
}

struct ModeVisitor;

impl<'de> Visitor<'de> for ModeVisitor {
    type Value = Mode;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid mode")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Mode::from_str(v).map_err(E::custom)
    }
}

impl Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(ModeVisitor)
    }
}

struct OfferTypeVisitor;

impl<'de> Visitor<'de> for OfferTypeVisitor {
    type Value = OfferType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid offer type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        OfferType::from_str(v).map_err(E::custom)
    }
}

impl Serialize for OfferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for OfferType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(OfferTypeVisitor)
    }
}

struct PlantSiteVisitor;

impl<'de> Visitor<'de> for PlantSiteVisitor {
    type Value = PlantSite;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid plant site")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        PlantSite::from_str(v).map_err(E::custom)
    }
}

impl Serialize for PlantSite {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for PlantSite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(PlantSiteVisitor)
    }
}

struct PlatformVisitor;

impl<'de> Visitor<'de> for PlatformVisitor {
    type Value = Platform;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid platform")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Platform::from_str(v).map_err(E::custom)
    }
}

impl Serialize for Platform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(PlatformVisitor)
    }
}

struct PlayerTeamVisitor;

impl<'de> Visitor<'de> for PlayerTeamVisitor {
    type Value = PlayerTeam;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid player team")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        PlayerTeam::from_str(v).map_err(E::custom)
    }
}

impl Serialize for PlayerTeam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for PlayerTeam {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(PlayerTeamVisitor)
    }
}

struct RegionVisitor;

impl<'de> Visitor<'de> for RegionVisitor {
    type Value = Region;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid region")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Region::from_str(v).map_err(E::custom)
    }
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(RegionVisitor)
    }
}

struct RoundEndTypeVisitor;

impl<'de> Visitor<'de> for RoundEndTypeVisitor {
    type Value = RoundEndType;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid round end type")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        RoundEndType::from_str(v).map_err(E::custom)
    }
}

impl Serialize for RoundEndType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for RoundEndType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(RoundEndTypeVisitor)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// OTHERS
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn serialize_color<S>(color: &u32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    let value = 0x00FFFFFF_u32 & color;
    serializer.serialize_str(format!("#{:x}", value).as_ref())
}

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>
{
    let value = String::deserialize(deserializer)?;
    let color = u32::from_str_radix(&value[1..], 16).map_err(Error::custom)?;
    Ok(0xFF000000_u32 | color)
}

pub fn serialize_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    serializer.serialize_str(date.format("%b %e %Y").to_string().as_str())
}

pub fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>
{
    let value = String::deserialize(deserializer)?;
    let date = NaiveDate::parse_from_str(value.as_ref(), "%b %e %Y")
        .map_err(Error::custom)?;
    Ok(date)
}

pub fn serialize_date_time_utc<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    serializer.serialize_i64(dt.timestamp())
}

pub fn deserialize_date_time_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>
{
    let value = i64::deserialize(deserializer)?;
    match NaiveDateTime::from_timestamp_opt(value, 0) {
        None => Err(Error::custom("not a valid unix timestamp")),
        Some(naive) => Ok(Utc.from_utc_datetime(&naive)),
    }
}
