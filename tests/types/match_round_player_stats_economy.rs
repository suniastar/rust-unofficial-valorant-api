use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MatchRoundPlayerStatsEconomy, MatchRoundPlayerStatsEconomyArmor, MatchRoundPlayerStatsEconomyArmorAssets, MatchRoundPlayerStatsEconomyWeapon, MatchRoundPlayerStatsEconomyWeaponAssets};

#[test]
fn serialize() {
    let load_out_value = 69420;
    let weapon = MatchRoundPlayerStatsEconomyWeapon {
        id: Uuid::new_v4(),
        name: String::from("Weapon Name"),
        assets: MatchRoundPlayerStatsEconomyWeaponAssets {
            display_icon_url: Url::parse("https://google.com/search?q=icon").unwrap(),
            kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
        },
    };
    let armor = MatchRoundPlayerStatsEconomyArmor {
        id: Uuid::new_v4(),
        name: String::from("Armor Name"),
        assets: MatchRoundPlayerStatsEconomyArmorAssets {
            display_icon_url: Url::parse("https://google.com/search?q=display").unwrap(),
        },
    };
    let remaining = 420;
    let spent = 32;

    let expected = format!("{{\
    \"loadout_value\":{load_out_value},\
    \"weapon\":{0},\
    \"armor\":{1},\
    \"remaining\":{remaining},\
    \"spent\":{spent}\
    }}",
                           serde_json::to_string(&weapon).unwrap(),
                           serde_json::to_string(&armor).unwrap(),
    );

    let economy = MatchRoundPlayerStatsEconomy {
        load_out_value,
        weapon,
        armor,
        remaining,
        spent,
    };
    let actual = serde_json::to_string(&economy).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}
