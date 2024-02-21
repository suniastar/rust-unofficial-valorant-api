use std::str::FromStr;

use rust_unofficial_valorant_api::types::PlatformType;

#[test]
fn to_str() {
    assert_eq!("PC", PlatformType::PC.to_str());
    assert_eq!("Console", PlatformType::Console.to_str());
}

#[test]
fn from_str() {
    assert_eq!(PlatformType::PC, PlatformType::from_str("PC").unwrap());
    assert_eq!(PlatformType::Console, PlatformType::from_str("Console").unwrap());

    assert_eq!(PlatformType::PC, PlatformType::from_str("pc").unwrap());
    assert_eq!(PlatformType::Console, PlatformType::from_str("CONSOLE").unwrap());

    assert!(PlatformType::from_str("not exist").is_err());
    assert!(PlatformType::from_str("").is_err());
    assert!(PlatformType::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"PC\"", serde_json::to_string(&PlatformType::PC).unwrap());
    assert_eq!("\"Console\"", serde_json::to_string(&PlatformType::Console).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(PlatformType::PC, serde_json::from_str("\"PC\"").unwrap());
    assert_eq!(PlatformType::Console, serde_json::from_str("\"Console\"").unwrap());

    assert_eq!(PlatformType::PC, serde_json::from_str("\"pc\"").unwrap());
    assert_eq!(PlatformType::Console, serde_json::from_str("\"CONSOLE\"").unwrap());

    assert!(serde_json::from_str::<PlatformType>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<PlatformType>("\"\"").is_err());
    assert!(serde_json::from_str::<PlatformType>("\"    \"").is_err());
}
