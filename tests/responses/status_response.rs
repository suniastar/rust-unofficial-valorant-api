use chrono::{FixedOffset, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::{Localized, Status, StatusData, StatusUpdate, ValorantApiError, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v1-status/bad_request.json");

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
fn deserialize_ok_empty() {
    let json = read_resource("v1-status/ok_empty.json");

    let expected = ValorantApiResponse {
        status: 200,
        errors: None,
        data: Some(
            StatusData {
                maintenances: vec![],
                incidents: vec![],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_example() {
    let json = read_resource("v1-status/ok_example.json");

    let expected = ValorantApiResponse {
        status: 200,
        errors: None,
        data: Some(
            StatusData {
                maintenances: vec![
                    Status {
                        id: 4175,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                            .unwrap()
                            .with_nanosecond(530_569_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        archived: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 13, 10, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        incident_severity: Some(String::from("warning")),
                        maintenance_status: String::from("in_progress"),
                        titles: vec![
                            Localized {
                                content: String::from("Emergency Maintenance In Progress"),
                                locale: String::from("en_US"),
                            }
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 6658,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                                    .unwrap()
                                    .with_nanosecond(645_499_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("riotclient"),
                                ],
                                translations: vec![
                                    Localized {
                                        content: String::from("The platform is currently unavailable while we perform emergency maintenance."),
                                        locale: String::from("en_US"),
                                    }
                                ],
                            }
                        ],
                    }
                ],
                incidents: vec![
                    Status {
                        id: 4175,
                        created: FixedOffset::east_opt(0)
                            .unwrap()
                            .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                            .unwrap()
                            .with_nanosecond(530_569_000)
                            .unwrap(),
                        updated: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        archived: Some(
                            FixedOffset::east_opt(0)
                                .unwrap()
                                .with_ymd_and_hms(2022, 1, 13, 10, 0, 0)
                                .unwrap()
                                .with_nanosecond(0)
                                .unwrap()
                        ),
                        incident_severity: Some(String::from("warning")),
                        maintenance_status: String::from("in_progress"),
                        titles: vec![
                            Localized {
                                content: String::from("Emergency Maintenance In Progress"),
                                locale: String::from("en_US"),
                            }
                        ],
                        platforms: vec![
                            String::from("windows"),
                        ],
                        updates: vec![
                            StatusUpdate {
                                id: 6658,
                                publish: true,
                                author: String::from("Riot Games"),
                                created: FixedOffset::east_opt(0)
                                    .unwrap()
                                    .with_ymd_and_hms(2022, 1, 12, 22, 11, 7)
                                    .unwrap()
                                    .with_nanosecond(645_499_000)
                                    .unwrap(),
                                updated: Some(
                                    FixedOffset::east_opt(0)
                                        .unwrap()
                                        .with_ymd_and_hms(2022, 1, 12, 23, 0, 0)
                                        .unwrap()
                                        .with_nanosecond(0)
                                        .unwrap()
                                ),
                                publish_locations: vec![
                                    String::from("riotclient"),
                                ],
                                translations: vec![
                                    Localized {
                                        content: String::from("The platform is currently unavailable while we perform emergency maintenance."),
                                        locale: String::from("en_US"),
                                    }
                                ],
                            }
                        ],
                    }
                ],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
