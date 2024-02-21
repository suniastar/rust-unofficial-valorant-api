use url::Url;

use rust_unofficial_valorant_api::types::{PlayerAssets, PlayerAssetsAgent, PlayerAssetsCard};

#[test]
fn serialize() {
    let card = PlayerAssetsCard {
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        large_url: Url::parse("https://google.com/search?q=large").unwrap(),
        wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
    };
    let agent = PlayerAssetsAgent {
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        full_url: Url::parse("https://google.com/search?q=full").unwrap(),
        bust_url: Url::parse("https://google.com/search?q=bust").unwrap(),
        kill_feed_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
    };

    let expected = format!("{{\
    \"card\":{0},\
    \"agent\":{1}\
    }}",
                           serde_json::to_string(&card).unwrap(),
                           serde_json::to_string(&agent).unwrap(),
    );

    let assets = PlayerAssets {
        card,
        agent,
    };
    let actual = serde_json::to_string(&assets).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let card = PlayerAssetsCard {
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        large_url: Url::parse("https://google.com/search?q=large").unwrap(),
        wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
    };
    let agent = PlayerAssetsAgent {
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        full_url: Url::parse("https://google.com/search?q=full").unwrap(),
        bust_url: Url::parse("https://google.com/search?q=bust").unwrap(),
        kill_feed_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
    };

    let json = format!("{{\
    \"card\":{0},\
    \"agent\":{1}\
    }}",
                       serde_json::to_string(&card).unwrap(),
                       serde_json::to_string(&agent).unwrap(),
    );

    let expected = PlayerAssets {
        card,
        agent,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
