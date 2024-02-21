use url::Url;

use rust_unofficial_valorant_api::types::PlayerAssetsCard;

#[test]
fn serialize() {
    let small = Url::parse("https://google.com/search?q=small").unwrap();
    let large = Url::parse("https://google.com/search?q=large").unwrap();
    let wide = Url::parse("https://google.com/search?q=wide").unwrap();

    let expected = format!("{{\
    \"small\":\"{small}\",\
    \"large\":\"{large}\",\
    \"wide\":\"{wide}\"\
    }}");

    let card = PlayerAssetsCard {
        small_url: small,
        large_url: large,
        wide_url: wide,
    };
    let actual = serde_json::to_string(&card).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let small = Url::parse("https://google.com/search?q=small").unwrap();
    let large = Url::parse("https://google.com/search?q=large").unwrap();
    let wide = Url::parse("https://google.com/search?q=wide").unwrap();

    let json = format!("{{\
    \"small\":\"{small}\",\
    \"large\":\"{large}\",\
    \"wide\":\"{wide}\"\
    }}");

    let expected = PlayerAssetsCard {
        small_url: small,
        large_url: large,
        wide_url: wide,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
