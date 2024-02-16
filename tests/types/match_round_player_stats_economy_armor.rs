use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MatchRoundPlayerStatsEconomyArmor, MatchRoundPlayerStatsEconomyArmorAssets};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let name = String::from("Name");
    let assets = MatchRoundPlayerStatsEconomyArmorAssets {
        display_icon_url: Url::parse("https://google.com/search?q=icon").unwrap(),
    };

    let expected = format!("{{\
    \"id\":\"{id}\",\
    \"name\":\"{name}\",\
    \"assets\":{0}\
    }}",
                           serde_json::to_string(&assets).unwrap(),
    );

    let armor = MatchRoundPlayerStatsEconomyArmor {
        id,
        name,
        assets,
    };
    let actual = serde_json::to_string(&armor).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let name = String::from("Name");
    let assets = MatchRoundPlayerStatsEconomyArmorAssets {
        display_icon_url: Url::parse("https://google.com/search?q=icon").unwrap(),
    };

    let json = format!("{{\
    \"id\":\"{id}\",\
    \"name\":\"{name}\",\
    \"assets\":{0}\
    }}",
                       serde_json::to_string(&assets).unwrap(),
    );

    let expected = MatchRoundPlayerStatsEconomyArmor {
        id,
        name,
        assets,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}