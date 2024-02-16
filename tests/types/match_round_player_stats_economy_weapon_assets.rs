use url::Url;

use rust_unofficial_valorant_api::types::MatchRoundPlayerStatsEconomyWeaponAssets;

#[test]
fn serialize() {
    let display_icon_url = Url::parse("https://google.com/search?q=icon").unwrap();
    let kill_feed_icon_url = Url::parse("https://google.com/search?q=killfeed").unwrap();

    let expected = format!("{{\
    \"display_icon\":\"{display_icon_url}\",\
    \"killfeed_icon\":\"{kill_feed_icon_url}\"\
    }}");

    let assets = MatchRoundPlayerStatsEconomyWeaponAssets {
        display_icon_url,
        kill_feed_icon_url,
    };
    let actual = serde_json::to_string(&assets).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let display_icon_url = Url::parse("https://google.com/search?q=icon").unwrap();
    let kill_feed_icon_url = Url::parse("https://google.com/search?q=killfeed").unwrap();

    let json = format!("{{\
    \"display_icon\":\"{display_icon_url}\",\
    \"killfeed_icon\":\"{kill_feed_icon_url}\"\
    }}");

    let expected = MatchRoundPlayerStatsEconomyWeaponAssets {
        display_icon_url,
        kill_feed_icon_url,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
