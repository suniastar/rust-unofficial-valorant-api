use chrono::{Timelike, TimeZone, Utc};
use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Card, Region, V1AccountData};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let region = Region::Europe;
    let account_level = 1024;
    let name = Some(String::from("t00manysecrets"));
    let tag = Some(String::from("EUW"));
    let card = Card {
        id: Uuid::new_v4(),
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        large_url: Url::parse("https://google.com/search?q=large").unwrap(),
        wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
    };
    let last_update = Utc
        .with_ymd_and_hms(2024, 2, 15, 14, 46, 18)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    let last_update_text = String::from("15 minutes ago");

    let expected = format!("{{\
    \"puuid\":\"{id}\",\
    \"region\":{0},\
    \"account_level\":{account_level},\
    \"name\":{1},\
    \"tag\":{2},\
    \"card\":{3},\
    \"last_update_raw\":{4},\
    \"last_update\":\"{last_update_text}\"\
    }}",
                           serde_json::to_string(&region).unwrap(),
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&card).unwrap(),
                           last_update.timestamp(),
    );

    let v1_account_data = V1AccountData {
        id,
        region,
        account_level,
        name,
        tag,
        card,
        last_update,
        last_update_text,
    };
    let actual = serde_json::to_string(&v1_account_data).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let region = Region::Europe;
    let account_level = 1024;
    let name = Some(String::from("t00manysecrets"));
    let tag = Some(String::from("EUW"));
    let card = Card {
        id: Uuid::new_v4(),
        small_url: Url::parse("https://google.com/search?q=small").unwrap(),
        large_url: Url::parse("https://google.com/search?q=large").unwrap(),
        wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
    };
    let last_update = Utc
        .with_ymd_and_hms(2024, 2, 15, 14, 46, 18)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    let last_update_text = String::from("15 minutes ago");

    let json = format!("{{\
    \"puuid\":\"{id}\",\
    \"region\":{0},\
    \"account_level\":{account_level},\
    \"name\":{1},\
    \"tag\":{2},\
    \"card\":{3},\
    \"last_update_raw\":{4},\
    \"last_update\":\"{last_update_text}\"\
    }}",
                       serde_json::to_string(&region).unwrap(),
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&card).unwrap(),
                       last_update.timestamp(),
    );

    let expected = V1AccountData {
        id,
        region,
        account_level,
        name,
        tag,
        card,
        last_update,
        last_update_text,
    };

    println!("{json}");

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
