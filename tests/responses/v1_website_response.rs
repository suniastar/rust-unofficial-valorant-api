use chrono::{FixedOffset, Timelike, TimeZone};
use url::Url;

use rust_unofficial_valorant_api::types::{V1StatusData, V1WebsiteData, ValorantApiError, ValorantApiResponse, Website};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-website/bad_request.json");

    let expected: ValorantApiResponse<V1StatusData> = ValorantApiResponse {
        status: 400,
        name: None,
        tag: None,
        errors: Some(
            vec![
                ValorantApiError {
                    code: 0,
                    message: String::from("string"),
                    details: String::from("string"),
                }
            ]
        ),
        results: None,
        data: None,
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_example() {
    let json = read_resource("v1-website/ok_example.json");

    let expected: ValorantApiResponse<V1WebsiteData> = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            vec![
                Website {
                    banner_url: Url::parse("https://images.contentstack.io/v3/assets/bltb6530b271fddd0b1/blta99717d4afdf4126/61ddd1a0997c0e224de0a718/01132022-VANILLE-BTS_Article-banner.jpg").unwrap(),
                    category: String::from("community"),
                    date: FixedOffset::east_opt(0)
                        .unwrap()
                        .with_ymd_and_hms(2022, 1, 13, 11, 0, 0)
                        .unwrap()
                        .with_nanosecond(0)
                        .unwrap(),
                    external_link: Some(Url::parse("https://www.youtube.com/watch?v=YRFwQy-dZ4o").unwrap()),
                    title: String::from("Die Stimme von Neon // VALORANT"),
                    url: Url::parse("https://playvalorant.com/de-de/news/community/meet-the-voice-of-neon-valorant/").unwrap(),
                },
            ]
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
