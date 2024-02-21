use url::Url;

use rust_unofficial_valorant_api::types::PlayerAssetsAgent;

#[test]
fn serialize() {
    let small = Url::parse("https://google.com/search?q=small").unwrap();
    let full = Url::parse("https://google.com/search?q=full").unwrap();
    let bust = Url::parse("https://google.com/search?q=bust").unwrap();
    let kill_feed = Url::parse("https://google.com/search?q=killfeed").unwrap();

    let expected = format!("{{\
    \"small\":\"{small}\",\
    \"full\":\"{full}\",\
    \"bust\":\"{bust}\",\
    \"killfeed\":\"{kill_feed}\"\
    }}");

    let card = PlayerAssetsAgent {
        small_url: small,
        full_url: full,
        bust_url: bust,
        kill_feed_url: kill_feed,
    };
    let actual = serde_json::to_string(&card).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let small = Url::parse("https://google.com/search?q=small").unwrap();
    let full = Url::parse("https://google.com/search?q=full").unwrap();
    let bust = Url::parse("https://google.com/search?q=bust").unwrap();
    let kill_feed = Url::parse("https://google.com/search?q=killfeed").unwrap();

    let json = format!("{{\
    \"small\":\"{small}\",\
    \"full\":\"{full}\",\
    \"bust\":\"{bust}\",\
    \"killfeed\":\"{kill_feed}\"\
    }}");

    let expected = PlayerAssetsAgent {
        small_url: small,
        full_url: full,
        bust_url: bust,
        kill_feed_url: kill_feed,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
