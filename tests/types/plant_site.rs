use std::str::FromStr;

use rust_unofficial_valorant_api::types::PlantSite;

#[test]
fn to_str() {
    assert_eq!("A", PlantSite::A.to_str());
    assert_eq!("B", PlantSite::B.to_str());
    assert_eq!("C", PlantSite::C.to_str());
}

#[test]
fn from_str() {
    assert_eq!(PlantSite::A, PlantSite::from_str("A").unwrap());
    assert_eq!(PlantSite::B, PlantSite::from_str("B").unwrap());
    assert_eq!(PlantSite::C, PlantSite::from_str("C").unwrap());

    assert_eq!(PlantSite::A, PlantSite::from_str("a").unwrap());
    assert_eq!(PlantSite::B, PlantSite::from_str("b").unwrap());
    assert_eq!(PlantSite::C, PlantSite::from_str("c").unwrap());

    assert!(PlantSite::from_str("not exist").is_err());
    assert!(PlantSite::from_str("").is_err());
    assert!(PlantSite::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"A\"", serde_json::to_string(&PlantSite::A).unwrap());
    assert_eq!("\"B\"", serde_json::to_string(&PlantSite::B).unwrap());
    assert_eq!("\"C\"", serde_json::to_string(&PlantSite::C).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(PlantSite::A, serde_json::from_str("\"A\"").unwrap());
    assert_eq!(PlantSite::B, serde_json::from_str("\"B\"").unwrap());
    assert_eq!(PlantSite::C, serde_json::from_str("\"C\"").unwrap());

    assert_eq!(PlantSite::A, serde_json::from_str("\"a\"").unwrap());
    assert_eq!(PlantSite::B, serde_json::from_str("\"b\"").unwrap());
    assert_eq!(PlantSite::C, serde_json::from_str("\"c\"").unwrap());

    assert!(serde_json::from_str::<PlantSite>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<PlantSite>("\"\"").is_err());
    assert!(serde_json::from_str::<PlantSite>("\"    \"").is_err());
}
