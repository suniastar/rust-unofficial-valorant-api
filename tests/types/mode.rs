use std::str::FromStr;

use rust_unofficial_valorant_api::types::Mode;

#[test]
fn to_str() {
    assert_eq!("Competitive", Mode::Competitive.to_str());
    assert_eq!("Unrated", Mode::Unrated.to_str());
    assert_eq!("Deathmatch", Mode::Deathmatch.to_str());
    assert_eq!("Team Deathmatch", Mode::TeamDeathmatch.to_str());
    assert_eq!("Swiftplay", Mode::Swiftplay.to_str());
    assert_eq!("Spike Rush", Mode::SpikeRush.to_str());
    assert_eq!("New Map", Mode::NewMap.to_str());
    assert_eq!("Snowball Fight", Mode::SnowballFight.to_str());
    assert_eq!("Custom Game", Mode::CustomGame.to_str());
}

#[test]
fn api() {
    assert_eq!("competitive", Mode::Competitive.api());
    assert_eq!("unrated", Mode::Unrated.api());
    assert_eq!("deathmatch", Mode::Deathmatch.api());
    assert_eq!("teamdeathmatch", Mode::TeamDeathmatch.api());
    assert_eq!("swiftplay", Mode::Swiftplay.api());
    assert_eq!("spikerush", Mode::SpikeRush.api());
    assert_eq!("newmap", Mode::NewMap.api());
    assert_eq!("snowballfight", Mode::SnowballFight.api());
    assert_eq!("custom", Mode::CustomGame.api());
}

#[test]
fn id() {
    assert_eq!("competitive", Mode::Competitive.id());
    assert_eq!("unrated", Mode::Unrated.id());
    assert_eq!("deathmatch", Mode::Deathmatch.id());
    assert_eq!("hurm", Mode::TeamDeathmatch.id());
    assert_eq!("swiftplay", Mode::Swiftplay.id());
    assert_eq!("spikerush", Mode::SpikeRush.id());
    assert_eq!("newmap", Mode::NewMap.id());
    assert_eq!("snowball", Mode::SnowballFight.id());
    assert_eq!("custom", Mode::CustomGame.id());
}

#[test]
fn from_str() {
    assert_eq!(Mode::Competitive, Mode::from_str("Competitive").unwrap());
    assert_eq!(Mode::Unrated, Mode::from_str("Unrated").unwrap());
    assert_eq!(Mode::Deathmatch, Mode::from_str("Deathmatch").unwrap());
    assert_eq!(Mode::TeamDeathmatch, Mode::from_str("Team Deathmatch").unwrap());
    assert_eq!(Mode::Swiftplay, Mode::from_str("Swiftplay").unwrap());
    assert_eq!(Mode::SpikeRush, Mode::from_str("Spike Rush").unwrap());
    assert_eq!(Mode::NewMap, Mode::from_str("New Map").unwrap());
    assert_eq!(Mode::SnowballFight, Mode::from_str("Snowball Fight").unwrap());
    assert_eq!(Mode::CustomGame, Mode::from_str("Custom Game").unwrap());

    assert_eq!(Mode::Competitive, Mode::from_str("competitive").unwrap());
    assert_eq!(Mode::Unrated, Mode::from_str("unrated").unwrap());
    assert_eq!(Mode::Deathmatch, Mode::from_str("deathmatch").unwrap());
    assert_eq!(Mode::TeamDeathmatch, Mode::from_str("teamdeathmatch").unwrap());
    assert_eq!(Mode::Swiftplay, Mode::from_str("swiftplay").unwrap());
    assert_eq!(Mode::SpikeRush, Mode::from_str("spikerush").unwrap());
    assert_eq!(Mode::NewMap, Mode::from_str("newmap").unwrap());
    assert_eq!(Mode::SnowballFight, Mode::from_str("snowballfight").unwrap());
    assert_eq!(Mode::CustomGame, Mode::from_str("custom").unwrap());

    assert_eq!(Mode::Competitive, Mode::from_str("competitive").unwrap());
    assert_eq!(Mode::Unrated, Mode::from_str("unrated").unwrap());
    assert_eq!(Mode::Deathmatch, Mode::from_str("deathmatch").unwrap());
    assert_eq!(Mode::TeamDeathmatch, Mode::from_str("hurm").unwrap());
    assert_eq!(Mode::Swiftplay, Mode::from_str("swiftplay").unwrap());
    assert_eq!(Mode::SpikeRush, Mode::from_str("spikerush").unwrap());
    assert_eq!(Mode::NewMap, Mode::from_str("newmap").unwrap());
    assert_eq!(Mode::SnowballFight, Mode::from_str("snowball").unwrap());
    assert_eq!(Mode::CustomGame, Mode::from_str("custom").unwrap());

    assert_eq!(Mode::Competitive, Mode::from_str("COMPETITIVE").unwrap());
    assert_eq!(Mode::Unrated, Mode::from_str("UNRATED").unwrap());
    assert_eq!(Mode::Deathmatch, Mode::from_str("DEATHMATCH").unwrap());
    assert_eq!(Mode::TeamDeathmatch, Mode::from_str("HURM").unwrap());
    assert_eq!(Mode::Swiftplay, Mode::from_str("SWIFTPLAY").unwrap());
    assert_eq!(Mode::SpikeRush, Mode::from_str("SPIKERUSH").unwrap());
    assert_eq!(Mode::NewMap, Mode::from_str("NEWMAP").unwrap());
    assert_eq!(Mode::SnowballFight, Mode::from_str("SNOWBALL").unwrap());
    assert_eq!(Mode::CustomGame, Mode::from_str("CUSTOM").unwrap());

    assert!(Mode::from_str("not exist").is_err());
    assert!(Mode::from_str("").is_err());
    assert!(Mode::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"competitive\"", serde_json::to_string(&Mode::Competitive).unwrap());
    assert_eq!("\"unrated\"", serde_json::to_string(&Mode::Unrated).unwrap());
    assert_eq!("\"deathmatch\"", serde_json::to_string(&Mode::Deathmatch).unwrap());
    assert_eq!("\"hurm\"", serde_json::to_string(&Mode::TeamDeathmatch).unwrap());
    assert_eq!("\"swiftplay\"", serde_json::to_string(&Mode::Swiftplay).unwrap());
    assert_eq!("\"spikerush\"", serde_json::to_string(&Mode::SpikeRush).unwrap());
    assert_eq!("\"newmap\"", serde_json::to_string(&Mode::NewMap).unwrap());
    assert_eq!("\"snowball\"", serde_json::to_string(&Mode::SnowballFight).unwrap());
    assert_eq!("\"custom\"", serde_json::to_string(&Mode::CustomGame).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Mode::Competitive, serde_json::from_str("\"Competitive\"").unwrap());
    assert_eq!(Mode::Unrated, serde_json::from_str("\"Unrated\"").unwrap());
    assert_eq!(Mode::Deathmatch, serde_json::from_str("\"Deathmatch\"").unwrap());
    assert_eq!(Mode::TeamDeathmatch, serde_json::from_str("\"Team Deathmatch\"").unwrap());
    assert_eq!(Mode::Swiftplay, serde_json::from_str("\"Swiftplay\"").unwrap());
    assert_eq!(Mode::SpikeRush, serde_json::from_str("\"Spike Rush\"").unwrap());
    assert_eq!(Mode::NewMap, serde_json::from_str("\"New Map\"").unwrap());
    assert_eq!(Mode::SnowballFight, serde_json::from_str("\"Snowball Fight\"").unwrap());
    assert_eq!(Mode::CustomGame, serde_json::from_str("\"Custom Game\"").unwrap());

    assert_eq!(Mode::Competitive, serde_json::from_str("\"competitive\"").unwrap());
    assert_eq!(Mode::Unrated, serde_json::from_str("\"unrated\"").unwrap());
    assert_eq!(Mode::Deathmatch, serde_json::from_str("\"deathmatch\"").unwrap());
    assert_eq!(Mode::TeamDeathmatch, serde_json::from_str("\"teamdeathmatch\"").unwrap());
    assert_eq!(Mode::Swiftplay, serde_json::from_str("\"swiftplay\"").unwrap());
    assert_eq!(Mode::SpikeRush, serde_json::from_str("\"spikerush\"").unwrap());
    assert_eq!(Mode::NewMap, serde_json::from_str("\"newmap\"").unwrap());
    assert_eq!(Mode::SnowballFight, serde_json::from_str("\"snowballfight\"").unwrap());
    assert_eq!(Mode::CustomGame, serde_json::from_str("\"custom\"").unwrap());

    assert_eq!(Mode::Competitive, serde_json::from_str("\"competitive\"").unwrap());
    assert_eq!(Mode::Unrated, serde_json::from_str("\"unrated\"").unwrap());
    assert_eq!(Mode::Deathmatch, serde_json::from_str("\"deathmatch\"").unwrap());
    assert_eq!(Mode::TeamDeathmatch, serde_json::from_str("\"hurm\"").unwrap());
    assert_eq!(Mode::Swiftplay, serde_json::from_str("\"swiftplay\"").unwrap());
    assert_eq!(Mode::SpikeRush, serde_json::from_str("\"spikerush\"").unwrap());
    assert_eq!(Mode::NewMap, serde_json::from_str("\"newmap\"").unwrap());
    assert_eq!(Mode::SnowballFight, serde_json::from_str("\"snowball\"").unwrap());
    assert_eq!(Mode::CustomGame, serde_json::from_str("\"custom\"").unwrap());

    assert_eq!(Mode::Competitive, serde_json::from_str("\"COMPETITIVE\"").unwrap());
    assert_eq!(Mode::Unrated, serde_json::from_str("\"UNRATED\"").unwrap());
    assert_eq!(Mode::Deathmatch, serde_json::from_str("\"DEATHMATCH\"").unwrap());
    assert_eq!(Mode::TeamDeathmatch, serde_json::from_str("\"HURM\"").unwrap());
    assert_eq!(Mode::Swiftplay, serde_json::from_str("\"SWIFTPLAY\"").unwrap());
    assert_eq!(Mode::SpikeRush, serde_json::from_str("\"SPIKERUSH\"").unwrap());
    assert_eq!(Mode::NewMap, serde_json::from_str("\"NEWMAP\"").unwrap());
    assert_eq!(Mode::SnowballFight, serde_json::from_str("\"SNOWBALL\"").unwrap());
    assert_eq!(Mode::CustomGame, serde_json::from_str("\"CUSTOM\"").unwrap());

    assert!(serde_json::from_str::<Mode>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Mode>("\"\"").is_err());
    assert!(serde_json::from_str::<Mode>("\"    \"").is_err());
}
