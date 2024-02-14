use std::str::FromStr;

use rust_unofficial_valorant_api::types::Map;

#[test]
fn to_str() {
    assert_eq!("Ascent", Map::Ascent.to_str());
    assert_eq!("Bind", Map::Bind.to_str());
    assert_eq!("Breeze", Map::Breeze.to_str());
    assert_eq!("District", Map::District.to_str());
    assert_eq!("Fracture", Map::Fracture.to_str());
    assert_eq!("Haven", Map::Haven.to_str());
    assert_eq!("Icebox", Map::Icebox.to_str());
    assert_eq!("Kasbah", Map::Kasbah.to_str());
    assert_eq!("Lotus", Map::Lotus.to_str());
    assert_eq!("Pearl", Map::Pearl.to_str());
    assert_eq!("Piazza", Map::Piazza.to_str());
    assert_eq!("Split", Map::Split.to_str());
    assert_eq!("Sunset", Map::Sunset.to_str());
}

#[test]
fn from_str() {
    assert_eq!(Map::Ascent, Map::from_str("Ascent").unwrap());
    assert_eq!(Map::Bind, Map::from_str("Bind").unwrap());
    assert_eq!(Map::Breeze, Map::from_str("Breeze").unwrap());
    assert_eq!(Map::District, Map::from_str("District").unwrap());
    assert_eq!(Map::Fracture, Map::from_str("Fracture").unwrap());
    assert_eq!(Map::Haven, Map::from_str("Haven").unwrap());
    assert_eq!(Map::Icebox, Map::from_str("Icebox").unwrap());
    assert_eq!(Map::Kasbah, Map::from_str("Kasbah").unwrap());
    assert_eq!(Map::Lotus, Map::from_str("Lotus").unwrap());
    assert_eq!(Map::Pearl, Map::from_str("Pearl").unwrap());
    assert_eq!(Map::Piazza, Map::from_str("Piazza").unwrap());
    assert_eq!(Map::Split, Map::from_str("Split").unwrap());
    assert_eq!(Map::Sunset, Map::from_str("Sunset").unwrap());

    assert_eq!(Map::Ascent, Map::from_str("ASCENT").unwrap());
    assert_eq!(Map::Bind, Map::from_str("BIND").unwrap());
    assert_eq!(Map::Breeze, Map::from_str("breeze").unwrap());
    assert_eq!(Map::District, Map::from_str("district").unwrap());
    assert_eq!(Map::Fracture, Map::from_str("FRACTURE").unwrap());
    assert_eq!(Map::Haven, Map::from_str("HAVEN").unwrap());
    assert_eq!(Map::Icebox, Map::from_str("icebox").unwrap());
    assert_eq!(Map::Kasbah, Map::from_str("kasbah").unwrap());
    assert_eq!(Map::Lotus, Map::from_str("LOTUS").unwrap());
    assert_eq!(Map::Pearl, Map::from_str("PEARL").unwrap());
    assert_eq!(Map::Piazza, Map::from_str("piazza").unwrap());
    assert_eq!(Map::Split, Map::from_str("split").unwrap());
    assert_eq!(Map::Sunset, Map::from_str("SUNSET").unwrap());

    assert!(Map::from_str("not exist").is_err());
    assert!(Map::from_str("").is_err());
    assert!(Map::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"Ascent\"", serde_json::to_string(&Map::Ascent).unwrap());
    assert_eq!("\"Bind\"", serde_json::to_string(&Map::Bind).unwrap());
    assert_eq!("\"Breeze\"", serde_json::to_string(&Map::Breeze).unwrap());
    assert_eq!("\"District\"", serde_json::to_string(&Map::District).unwrap());
    assert_eq!("\"Fracture\"", serde_json::to_string(&Map::Fracture).unwrap());
    assert_eq!("\"Haven\"", serde_json::to_string(&Map::Haven).unwrap());
    assert_eq!("\"Icebox\"", serde_json::to_string(&Map::Icebox).unwrap());
    assert_eq!("\"Kasbah\"", serde_json::to_string(&Map::Kasbah).unwrap());
    assert_eq!("\"Lotus\"", serde_json::to_string(&Map::Lotus).unwrap());
    assert_eq!("\"Pearl\"", serde_json::to_string(&Map::Pearl).unwrap());
    assert_eq!("\"Piazza\"", serde_json::to_string(&Map::Piazza).unwrap());
    assert_eq!("\"Split\"", serde_json::to_string(&Map::Split).unwrap());
    assert_eq!("\"Sunset\"", serde_json::to_string(&Map::Sunset).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Map::Ascent, serde_json::from_str("\"Ascent\"").unwrap());
    assert_eq!(Map::Bind, serde_json::from_str("\"Bind\"").unwrap());
    assert_eq!(Map::Breeze, serde_json::from_str("\"Breeze\"").unwrap());
    assert_eq!(Map::District, serde_json::from_str("\"District\"").unwrap());
    assert_eq!(Map::Fracture, serde_json::from_str("\"Fracture\"").unwrap());
    assert_eq!(Map::Haven, serde_json::from_str("\"Haven\"").unwrap());
    assert_eq!(Map::Icebox, serde_json::from_str("\"Icebox\"").unwrap());
    assert_eq!(Map::Kasbah, serde_json::from_str("\"Kasbah\"").unwrap());
    assert_eq!(Map::Lotus, serde_json::from_str("\"Lotus\"").unwrap());
    assert_eq!(Map::Pearl, serde_json::from_str("\"Pearl\"").unwrap());
    assert_eq!(Map::Piazza, serde_json::from_str("\"Piazza\"").unwrap());
    assert_eq!(Map::Split, serde_json::from_str("\"Split\"").unwrap());
    assert_eq!(Map::Sunset, serde_json::from_str("\"Sunset\"").unwrap());

    assert_eq!(Map::Ascent, serde_json::from_str("\"ASCENT\"").unwrap());
    assert_eq!(Map::Bind, serde_json::from_str("\"BIND\"").unwrap());
    assert_eq!(Map::Breeze, serde_json::from_str("\"breeze\"").unwrap());
    assert_eq!(Map::District, serde_json::from_str("\"district\"").unwrap());
    assert_eq!(Map::Fracture, serde_json::from_str("\"FRACTURE\"").unwrap());
    assert_eq!(Map::Haven, serde_json::from_str("\"HAVEN\"").unwrap());
    assert_eq!(Map::Icebox, serde_json::from_str("\"icebox\"").unwrap());
    assert_eq!(Map::Kasbah, serde_json::from_str("\"kasbah\"").unwrap());
    assert_eq!(Map::Lotus, serde_json::from_str("\"LOTUS\"").unwrap());
    assert_eq!(Map::Pearl, serde_json::from_str("\"PEARL\"").unwrap());
    assert_eq!(Map::Piazza, serde_json::from_str("\"piazza\"").unwrap());
    assert_eq!(Map::Split, serde_json::from_str("\"split\"").unwrap());
    assert_eq!(Map::Sunset, serde_json::from_str("\"SUNSET\"").unwrap());

    assert!(serde_json::from_str::<Map>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Map>("\"\"").is_err());
    assert!(serde_json::from_str::<Map>("\"    \"").is_err());
}
