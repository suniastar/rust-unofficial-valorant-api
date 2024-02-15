use chrono::{TimeZone, Utc};
use url::Url;
use uuid::uuid;

use rust_unofficial_valorant_api::types::{Card, Region, StatusData, V1AccountData, ValorantApiError, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-account/bad_request.json");

    let expected: ValorantApiResponse<StatusData> = ValorantApiResponse {
        status: 400,
        errors: Some(
            vec![
                ValorantApiError {
                    code: 0,
                    message: String::from("string"),
                    details: String::from("string"),
                }
            ]
        ),
        data: None,
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_example() {
    let json = read_resource("v1-account/ok_example.json");

    let expected: ValorantApiResponse<V1AccountData> = ValorantApiResponse {
        status: 200,
        errors: None,
        data: Some(
            V1AccountData {
                id: uuid!("f314b09f-7694-5ac0-ba34-5fc27e4d185c"),
                region: Region::Europe,
                account_level: 205,
                name: Some(String::from("t00manysecrets")),
                tag: Some(String::from("EUW")),
                card: Card {
                    id: uuid!("a2bc068a-44ed-9be6-cd77-c3a193e62e68"),
                    small_url: Url::parse("https://media.valorant-api.com/playercards/a2bc068a-44ed-9be6-cd77-c3a193e62e68/smallart.png").unwrap(),
                    large_url: Url::parse("https://media.valorant-api.com/playercards/a2bc068a-44ed-9be6-cd77-c3a193e62e68/largeart.png").unwrap(),
                    wide_url: Url::parse("https://media.valorant-api.com/playercards/a2bc068a-44ed-9be6-cd77-c3a193e62e68/wideart.png").unwrap(),
                },
                last_update: Utc
                    .with_ymd_and_hms(2024, 2, 15, 17, 13, 7)
                    .unwrap(),
                last_update_text: String::from("Now"),
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
