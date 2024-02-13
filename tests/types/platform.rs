use std::str::FromStr;

use rust_unofficial_valorant_api::types::Platform;

#[test]
fn to_str() {
    assert_eq!("PC", Platform::PC.to_str());
    assert_eq!("Console", Platform::Console.to_str());
}

#[test]
fn from_str() {
    assert_eq!(Platform::PC, Platform::from_str("PC").unwrap());
    assert_eq!(Platform::Console, Platform::from_str("Console").unwrap());

    assert_eq!(Platform::PC, Platform::from_str("pc").unwrap());
    assert_eq!(Platform::Console, Platform::from_str("CONSOLE").unwrap());

    assert!(Platform::from_str("not exist").is_err());
    assert!(Platform::from_str("").is_err());
    assert!(Platform::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"PC\"", serde_json::to_string(&Platform::PC).unwrap());
    assert_eq!("\"Console\"", serde_json::to_string(&Platform::Console).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Platform::PC, serde_json::from_str("\"PC\"").unwrap());
    assert_eq!(Platform::Console, serde_json::from_str("\"Console\"").unwrap());

    assert_eq!(Platform::PC, serde_json::from_str("\"pc\"").unwrap());
    assert_eq!(Platform::Console, serde_json::from_str("\"CONSOLE\"").unwrap());

    assert!(serde_json::from_str::<Platform>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Platform>("\"\"").is_err());
    assert!(serde_json::from_str::<Platform>("\"    \"").is_err());
}
