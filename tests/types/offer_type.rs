use std::str::FromStr;

use rust_unofficial_valorant_api::types::{OfferType, Region};

#[test]
fn to_str() {
    assert_eq!("buddy", OfferType::Buddy.to_str());
    assert_eq!("player_card", OfferType::PlayerCard.to_str());
    assert_eq!("player_title", OfferType::PlayerTitle.to_str());
    assert_eq!("skin_chroma", OfferType::SkinChroma.to_str());
    assert_eq!("skin_level", OfferType::SkinLevel.to_str());
    assert_eq!("spray", OfferType::Spray.to_str());
}

#[test]
fn from_str() {
    assert_eq!(OfferType::Buddy, OfferType::from_str("buddy").unwrap());
    assert_eq!(OfferType::PlayerCard, OfferType::from_str("player_card").unwrap());
    assert_eq!(OfferType::PlayerTitle, OfferType::from_str("player_title").unwrap());
    assert_eq!(OfferType::SkinChroma, OfferType::from_str("skin_chroma").unwrap());
    assert_eq!(OfferType::SkinLevel, OfferType::from_str("skin_level").unwrap());
    assert_eq!(OfferType::Spray, OfferType::from_str("spray").unwrap());

    assert_eq!(OfferType::Buddy, OfferType::from_str("BUDDY").unwrap());
    assert_eq!(OfferType::PlayerCard, OfferType::from_str("PLAYER_CARD").unwrap());
    assert_eq!(OfferType::PlayerTitle, OfferType::from_str("PLAYER_TITLE").unwrap());
    assert_eq!(OfferType::SkinChroma, OfferType::from_str("SKIN_CHROMA").unwrap());
    assert_eq!(OfferType::SkinLevel, OfferType::from_str("SKIN_LEVEL").unwrap());
    assert_eq!(OfferType::Spray, OfferType::from_str("SPRAY").unwrap());

    assert!(Region::from_str("not exist").is_err());
    assert!(Region::from_str("").is_err());
    assert!(Region::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"buddy\"", serde_json::to_string(&OfferType::Buddy).unwrap());
    assert_eq!("\"player_card\"", serde_json::to_string(&OfferType::PlayerCard).unwrap());
    assert_eq!("\"player_title\"", serde_json::to_string(&OfferType::PlayerTitle).unwrap());
    assert_eq!("\"skin_chroma\"", serde_json::to_string(&OfferType::SkinChroma).unwrap());
    assert_eq!("\"skin_level\"", serde_json::to_string(&OfferType::SkinLevel).unwrap());
    assert_eq!("\"spray\"", serde_json::to_string(&OfferType::Spray).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(OfferType::Buddy, serde_json::from_str("\"buddy\"").unwrap());
    assert_eq!(OfferType::PlayerCard, serde_json::from_str("\"player_card\"").unwrap());
    assert_eq!(OfferType::PlayerTitle, serde_json::from_str("\"player_title\"").unwrap());
    assert_eq!(OfferType::SkinChroma, serde_json::from_str("\"skin_chroma\"").unwrap());
    assert_eq!(OfferType::SkinLevel, serde_json::from_str("\"skin_level\"").unwrap());
    assert_eq!(OfferType::Spray, serde_json::from_str("\"spray\"").unwrap());

    assert_eq!(OfferType::Buddy, serde_json::from_str("\"BUDDY\"").unwrap());
    assert_eq!(OfferType::PlayerCard, serde_json::from_str("\"PLAYER_CARD\"").unwrap());
    assert_eq!(OfferType::PlayerTitle, serde_json::from_str("\"PLAYER_TITLE\"").unwrap());
    assert_eq!(OfferType::SkinChroma, serde_json::from_str("\"SKIN_CHROMA\"").unwrap());
    assert_eq!(OfferType::SkinLevel, serde_json::from_str("\"SKIN_LEVEL\"").unwrap());
    assert_eq!(OfferType::Spray, serde_json::from_str("\"SPRAY\"").unwrap());

    assert!(Region::from_str("\"not exist\"").is_err());
    assert!(Region::from_str("\"\"").is_err());
    assert!(Region::from_str("\"    \"").is_err());
}
