use std::str::FromStr;

use rust_unofficial_valorant_api::types::RoundEndType;

#[test]
fn to_str() {
    assert_eq!("Eliminated", RoundEndType::Eliminated.to_str());
    assert_eq!("Bomb detonated", RoundEndType::BombDetonated.to_str());
    assert_eq!("Bomb defused", RoundEndType::BombDefused.to_str());
    assert_eq!("Round timer expired", RoundEndType::RoundTimerExpired.to_str());
}

#[test]
fn from_str() {
    assert_eq!(RoundEndType::Eliminated, RoundEndType::from_str("Eliminated").unwrap());
    assert_eq!(RoundEndType::BombDetonated, RoundEndType::from_str("Bomb detonated").unwrap());
    assert_eq!(RoundEndType::BombDefused, RoundEndType::from_str("Bomb defused").unwrap());
    assert_eq!(RoundEndType::RoundTimerExpired, RoundEndType::from_str("Round timer expired").unwrap());

    assert_eq!(RoundEndType::Eliminated, RoundEndType::from_str("ELIMINATED").unwrap());
    assert_eq!(RoundEndType::BombDetonated, RoundEndType::from_str("Bomb DetonateD").unwrap());
    assert_eq!(RoundEndType::BombDefused, RoundEndType::from_str("bomb defused").unwrap());
    assert_eq!(RoundEndType::RoundTimerExpired, RoundEndType::from_str("Round Timer expired").unwrap());

    assert!(RoundEndType::from_str("not exist").is_err());
    assert!(RoundEndType::from_str("").is_err());
    assert!(RoundEndType::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"Eliminated\"", serde_json::to_string(&RoundEndType::Eliminated).unwrap());
    assert_eq!("\"Bomb detonated\"", serde_json::to_string(&RoundEndType::BombDetonated).unwrap());
    assert_eq!("\"Bomb defused\"", serde_json::to_string(&RoundEndType::BombDefused).unwrap());
    assert_eq!("\"Round timer expired\"", serde_json::to_string(&RoundEndType::RoundTimerExpired).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(RoundEndType::Eliminated, serde_json::from_str("\"Eliminated\"").unwrap());
    assert_eq!(RoundEndType::BombDetonated, serde_json::from_str("\"Bomb detonated\"").unwrap());
    assert_eq!(RoundEndType::BombDefused, serde_json::from_str("\"Bomb defused\"").unwrap());
    assert_eq!(RoundEndType::RoundTimerExpired, serde_json::from_str("\"Round timer expired\"").unwrap());

    assert_eq!(RoundEndType::Eliminated, serde_json::from_str("\"ELIMINATED\"").unwrap());
    assert_eq!(RoundEndType::BombDetonated, serde_json::from_str("\"Bomb DetonateD\"").unwrap());
    assert_eq!(RoundEndType::BombDefused, serde_json::from_str("\"bomb defused\"").unwrap());
    assert_eq!(RoundEndType::RoundTimerExpired, serde_json::from_str("\"Round Timer expired\"").unwrap());

    assert!(serde_json::from_str::<RoundEndType>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<RoundEndType>("\"\"").is_err());
    assert!(serde_json::from_str::<RoundEndType>("\"    \"").is_err());
}
