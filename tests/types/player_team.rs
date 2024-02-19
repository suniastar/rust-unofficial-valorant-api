use std::str::FromStr;

use rust_unofficial_valorant_api::types::PlayerTeam;

#[test]
fn to_str() {
    assert_eq!("Red", PlayerTeam::Red.to_str());
    assert_eq!("Blue", PlayerTeam::Blue.to_str());
}

#[test]
fn from_str() {
    assert_eq!(PlayerTeam::Red, PlayerTeam::from_str("Red").unwrap());
    assert_eq!(PlayerTeam::Blue, PlayerTeam::from_str("Blue").unwrap());

    assert_eq!(PlayerTeam::Red, PlayerTeam::from_str("red").unwrap());
    assert_eq!(PlayerTeam::Blue, PlayerTeam::from_str("blue").unwrap());

    assert!(PlayerTeam::from_str("not exist").is_err());
    assert!(PlayerTeam::from_str("").is_err());
    assert!(PlayerTeam::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"Red\"", serde_json::to_string(&PlayerTeam::Red).unwrap());
    assert_eq!("\"Blue\"", serde_json::to_string(&PlayerTeam::Blue).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(PlayerTeam::Red, serde_json::from_str("\"Red\"").unwrap());
    assert_eq!(PlayerTeam::Blue, serde_json::from_str("\"Blue\"").unwrap());

    assert_eq!(PlayerTeam::Red, serde_json::from_str("\"red\"").unwrap());
    assert_eq!(PlayerTeam::Blue, serde_json::from_str("\"blue\"").unwrap());

    assert!(serde_json::from_str::<PlayerTeam>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<PlayerTeam>("\"\"").is_err());
    assert!(serde_json::from_str::<PlayerTeam>("\"    \"").is_err());
}
