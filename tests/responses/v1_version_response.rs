use chrono::{FixedOffset, NaiveDate, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::{Region, V1StatusData, V1VersionData, ValorantApiError, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-version/bad_request.json");

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
    let json = read_resource("v1-version/ok_example.json");

    let expected: ValorantApiResponse<V1VersionData> = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V1VersionData {
                region: Region::Europe,
                branch: String::from("release-08.00"),
                build_date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                build_version: String::from("08.00.00.2191955"),
                last_checked: FixedOffset::east_opt(0)
                    .unwrap()
                    .with_ymd_and_hms(2024, 1, 21, 7, 12, 9)
                    .unwrap()
                    .with_nanosecond(043_000_000)
                    .unwrap(),
                version: 14,
                version_for_api: String::from("release-08.00-shipping-14-2191955"),
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
