use std::fmt::Formatter;
use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};

use crate::types::*;

struct AffinityVisitor;

impl<'de> Visitor<'de> for AffinityVisitor {
    type Value = Affinity;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid affinity")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Affinity::from_str(v).map_err(|msg| E::custom(msg))
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
        Map::from_str(v).map_err(|msg| E::custom(msg))
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
        Mode::from_str(v).map_err(|msg| E::custom(msg))
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
        OfferType::from_str(v).map_err(|msg| E::custom(msg))
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

struct PlatformVisitor;

impl<'de> Visitor<'de> for PlatformVisitor {
    type Value = Platform;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid platform")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Platform::from_str(v).map_err(|msg| E::custom(msg))
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

struct RegionVisitor;

impl<'de> Visitor<'de> for RegionVisitor {
    type Value = Region;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid region")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Region::from_str(v).map_err(|msg| E::custom(msg))
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

struct NaiveDateVisitor;

impl<'de> Visitor<'de> for NaiveDateVisitor {
    type Value = NaiveDate;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid date like this: Jan  1 2024")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        NaiveDate::parse_from_str(v, "%b %e %Y").map_err(|msg| E::custom(msg))
    }
}

pub fn serialize_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_str(date.format("%b %e %Y").to_string().as_str())
}

pub fn deserialize_naive_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error> where D: Deserializer<'de> {
    deserializer.deserialize_str(NaiveDateVisitor)
}

struct DateTimeUtcVisitor;

impl<'de> Visitor<'de> for DateTimeUtcVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid unix timestamp")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        match NaiveDateTime::from_timestamp_opt(v, 0) {
            None => Err(E::custom("not a valid unix timestamp")),
            Some(naive) => Ok(Utc.from_utc_datetime(&naive)),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        self.visit_i64(v as i64)
    }
}

pub fn serialize_date_time_utc<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    serializer.serialize_i64(dt.timestamp())
}

pub fn deserialize_date_time_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error> where D: Deserializer<'de> {
    deserializer.deserialize_i64(DateTimeUtcVisitor)
}
