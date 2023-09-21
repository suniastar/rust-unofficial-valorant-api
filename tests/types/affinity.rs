use std::str::FromStr;

use unofficial_valorant_api::types::Affinity;

#[test]
fn test_to_str() {
    assert_eq!("eu", Affinity::Europe.to_str());
    assert_eq!("na", Affinity::NorthAmerica.to_str());
    assert_eq!("latam", Affinity::LatinAmerica.to_str());
    assert_eq!("br", Affinity::Brazil.to_str());
    assert_eq!("ap", Affinity::AsiaPacific.to_str());
    assert_eq!("kr", Affinity::Korea.to_str());
}

#[test]
fn test_from_str() {
    assert_eq!(Affinity::Europe, Affinity::from_str("eu").unwrap());
    assert_eq!(Affinity::NorthAmerica, Affinity::from_str("na").unwrap());
    assert_eq!(Affinity::LatinAmerica, Affinity::from_str("latam").unwrap());
    assert_eq!(Affinity::Brazil, Affinity::from_str("br").unwrap());
    assert_eq!(Affinity::AsiaPacific, Affinity::from_str("ap").unwrap());
    assert_eq!(Affinity::Korea, Affinity::from_str("kr").unwrap());

    assert_eq!(Affinity::Europe, Affinity::from_str("EU").unwrap());
    assert_eq!(Affinity::NorthAmerica, Affinity::from_str("NA").unwrap());
    assert_eq!(Affinity::LatinAmerica, Affinity::from_str("LaTaM").unwrap());
    assert_eq!(Affinity::Brazil, Affinity::from_str("BR").unwrap());
    assert_eq!(Affinity::AsiaPacific, Affinity::from_str("AP").unwrap());
    assert_eq!(Affinity::Korea, Affinity::from_str("KR").unwrap());

    assert!(Affinity::from_str("not exist").is_err());
    assert!(Affinity::from_str("").is_err());
    assert!(Affinity::from_str("    ").is_err());
}

#[test]
fn test_serialize() {
    assert_eq!("\"eu\"", serde_json::to_string(&Affinity::Europe).unwrap());
    assert_eq!("\"na\"", serde_json::to_string(&Affinity::NorthAmerica).unwrap());
    assert_eq!("\"latam\"", serde_json::to_string(&Affinity::LatinAmerica).unwrap());
    assert_eq!("\"br\"", serde_json::to_string(&Affinity::Brazil).unwrap());
    assert_eq!("\"ap\"", serde_json::to_string(&Affinity::AsiaPacific).unwrap());
    assert_eq!("\"kr\"", serde_json::to_string(&Affinity::Korea).unwrap());
}

#[test]
fn test_deserialize() {
    assert_eq!(Affinity::Europe, serde_json::from_str("\"eu\"").unwrap());
    assert_eq!(Affinity::NorthAmerica, serde_json::from_str("\"na\"").unwrap());
    assert_eq!(Affinity::LatinAmerica, serde_json::from_str("\"latam\"").unwrap());
    assert_eq!(Affinity::Brazil, serde_json::from_str("\"br\"").unwrap());
    assert_eq!(Affinity::AsiaPacific, serde_json::from_str("\"ap\"").unwrap());
    assert_eq!(Affinity::Korea, serde_json::from_str("\"kr\"").unwrap());

    assert_eq!(Affinity::Europe, serde_json::from_str("\"EU\"").unwrap());
    assert_eq!(Affinity::NorthAmerica, serde_json::from_str("\"NA\"").unwrap());
    assert_eq!(Affinity::LatinAmerica, serde_json::from_str("\"LaTaM\"").unwrap());
    assert_eq!(Affinity::Brazil, serde_json::from_str("\"BR\"").unwrap());
    assert_eq!(Affinity::AsiaPacific, serde_json::from_str("\"AP\"").unwrap());
    assert_eq!(Affinity::Korea, serde_json::from_str("\"KR\"").unwrap());

    assert!(serde_json::from_str::<Affinity>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Affinity>("\"\"").is_err());
    assert!(serde_json::from_str::<Affinity>("\"    \"").is_err());
}
