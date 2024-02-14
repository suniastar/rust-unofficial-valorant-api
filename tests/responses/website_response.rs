use chrono::{FixedOffset, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::{StatusData, ValorantApiError, ValorantApiResponse, Website, WebsiteData};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-website/bad_request.json");

    let expected: ValorantApiResponse<StatusData> = ValorantApiResponse {
        status: 400,
        errors: Some(
            vec![
                ValorantApiError {
                    code: 0,
                    message: "string",
                    details: "string",
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
    let json = read_resource("v1-website/ok_example.json");

    let expected: ValorantApiResponse<WebsiteData> = ValorantApiResponse {
        status: 200,
        errors: None,
        data: Some(
            vec![
                Website {
                    banner_url: "https://images.contentstack.io/v3/assets/bltb6530b271fddd0b1/blta99717d4afdf4126/61ddd1a0997c0e224de0a718/01132022-VANILLE-BTS_Article-banner.jpg",
                    category: "community",
                    date: FixedOffset::east_opt(0)
                        .unwrap()
                        .with_ymd_and_hms(2022, 1, 13, 11, 0, 0)
                        .unwrap()
                        .with_nanosecond(0)
                        .unwrap(),
                    external_link: Some("https://www.youtube.com/watch?v=YRFwQy-dZ4o"),
                    title: "Die Stimme von Neon // VALORANT",
                    url: "https://playvalorant.com/de-de/news/community/meet-the-voice-of-neon-valorant/",
                },
            ]
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}