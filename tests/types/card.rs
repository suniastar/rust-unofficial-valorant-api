use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::Card;

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let small_url = Url::parse("https://google.com/search?q=small").unwrap();
    let large_url = Url::parse("https://google.com/search?q=large").unwrap();
    let wide_url = Url::parse("https://google.com/search?q=wide").unwrap();

    let expected = format!("{{\
    \"id\":\"{id}\",\
    \"small\":\"{small_url}\",\
    \"large\":\"{large_url}\",\
    \"wide\":\"{wide_url}\"\
    }}");

    let card = Card {
        id,
        small_url,
        large_url,
        wide_url,
    };
    let actual = serde_json::to_string(&card).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let small_url = Url::parse("https://google.com/search?q=small").unwrap();
    let large_url = Url::parse("https://google.com/search?q=large").unwrap();
    let wide_url = Url::parse("https://google.com/search?q=wide").unwrap();

    let json = format!("{{\
    \"id\":\"{id}\",\
    \"small\":\"{small_url}\",\
    \"large\":\"{large_url}\",\
    \"wide\":\"{wide_url}\"\
    }}");

    let expected = Card {
        id,
        small_url,
        large_url,
        wide_url,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
