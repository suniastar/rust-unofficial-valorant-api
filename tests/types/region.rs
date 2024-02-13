use std::str::FromStr;

use rust_unofficial_valorant_api::types::Region;

#[test]
fn to_str() {
    assert_eq!("eu", Region::Europe.to_str());
    assert_eq!("na", Region::NorthAmerica.to_str());
    assert_eq!("ap", Region::AsiaPacific.to_str());
    assert_eq!("kr", Region::Korea.to_str());
}

#[test]
fn from_str() {
    assert_eq!(Region::Europe, Region::from_str("eu").unwrap());
    assert_eq!(Region::NorthAmerica, Region::from_str("na").unwrap());
    assert_eq!(Region::AsiaPacific, Region::from_str("ap").unwrap());
    assert_eq!(Region::Korea, Region::from_str("kr").unwrap());

    assert_eq!(Region::Europe, Region::from_str("EU").unwrap());
    assert_eq!(Region::NorthAmerica, Region::from_str("NA").unwrap());
    assert_eq!(Region::AsiaPacific, Region::from_str("AP").unwrap());
    assert_eq!(Region::Korea, Region::from_str("KR").unwrap());

    assert!(Region::from_str("not exist").is_err());
    assert!(Region::from_str("").is_err());
    assert!(Region::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"eu\"", serde_json::to_string(&Region::Europe).unwrap());
    assert_eq!("\"na\"", serde_json::to_string(&Region::NorthAmerica).unwrap());
    assert_eq!("\"ap\"", serde_json::to_string(&Region::AsiaPacific).unwrap());
    assert_eq!("\"kr\"", serde_json::to_string(&Region::Korea).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Region::Europe, serde_json::from_str("\"eu\"").unwrap());
    assert_eq!(Region::NorthAmerica, serde_json::from_str("\"na\"").unwrap());
    assert_eq!(Region::AsiaPacific, serde_json::from_str("\"ap\"").unwrap());
    assert_eq!(Region::Korea, serde_json::from_str("\"kr\"").unwrap());

    assert_eq!(Region::Europe, serde_json::from_str("\"EU\"").unwrap());
    assert_eq!(Region::NorthAmerica, serde_json::from_str("\"NA\"").unwrap());
    assert_eq!(Region::AsiaPacific, serde_json::from_str("\"AP\"").unwrap());
    assert_eq!(Region::Korea, serde_json::from_str("\"KR\"").unwrap());

    assert!(serde_json::from_str::<Region>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Region>("\"\"").is_err());
    assert!(serde_json::from_str::<Region>("\"   \"").is_err());
}
