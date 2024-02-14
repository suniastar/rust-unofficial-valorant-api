use url::Url;

use rust_unofficial_valorant_api::types::OfferTier;

#[test]
fn serialize() {
    let name = String::from("Example Name");
    let dev_name = String::from("example");
    let icon_url = Url::parse("https://google.com/search?q=example").unwrap();

    let expected = format!("{{\
    \"name\":\"{name}\",\
    \"dev_name\":\"{dev_name}\",\
    \"icon\":\"{icon_url}\"\
    }}");

    let content_tier = OfferTier {
        name,
        dev_name,
        icon_url,
    };
    let actual = serde_json::to_string(&content_tier).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let name = String::from("Example Name");
    let dev_name = String::from("example");
    let icon_url = Url::parse("https://google.com/search?q=example").unwrap();

    let json = format!("{{\
    \"name\":\"{name}\",\
    \"dev_name\":\"{dev_name}\",\
    \"icon\":\"{icon_url}\"\
    }}");

    let expected = OfferTier {
        name,
        dev_name,
        icon_url,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
