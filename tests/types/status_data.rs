use chrono::{FixedOffset, Timelike, TimeZone};
use rand::Rng;

use rust_unofficial_valorant_api::types::{Localized, Status, StatusData, StatusUpdate};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let maintenances = vec![
        Status {
            id: rng.gen(),
            created: FixedOffset::east_opt(1 * 3600)
                .unwrap()
                .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                .unwrap()
                .with_nanosecond(412_356_412)
                .unwrap(),
            updated: Some(
                FixedOffset::east_opt(1 * 3600)
                    .unwrap()
                    .with_ymd_and_hms(2023, 12, 25, 7, 13, 45)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            ),
            archived: None,
            incident_severity: Some(String::from("warning")),
            maintenance_status: String::from("in_progress"),
            titles: vec![
                Localized {
                    content: String::from("Emergency Maintenance In Progress"),
                    locale: String::from("en_US"),
                },
            ],
            platforms: vec![
                String::from("windows"),
            ],
            updates: vec![
                StatusUpdate {
                    id: rng.gen(),
                    publish: false,
                    author: String::from("Riot Games"),
                    created: FixedOffset::east_opt(1 * 3600)
                        .unwrap()
                        .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                        .unwrap()
                        .with_nanosecond(942_466_466)
                        .unwrap(),
                    updated: None,
                    publish_locations: vec![
                        String::from("client"),
                        String::from("website"),
                    ],
                    translations: vec![
                        Localized {
                            content: String::from("Maintenance begin"),
                            locale: String::from("en_US"),
                        },
                    ],
                }
            ],
        },
    ];
    let incidents = vec![
        Status {
            id: rng.gen(),
            created: FixedOffset::east_opt(1 * 3600)
                .unwrap()
                .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                .unwrap()
                .with_nanosecond(412_356_412)
                .unwrap(),
            updated: Some(
                FixedOffset::east_opt(1 * 3600)
                    .unwrap()
                    .with_ymd_and_hms(2023, 12, 25, 7, 14, 45)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            ),
            archived: None,
            incident_severity: Some(String::from("warning")),
            maintenance_status: String::from("in_progress"),
            titles: vec![
                Localized {
                    content: String::from("Emergency Maintenance In Progress"),
                    locale: String::from("en_US"),
                },
            ],
            platforms: vec![
                String::from("windows"),
            ],
            updates: vec![
                StatusUpdate {
                    id: rng.gen(),
                    publish: false,
                    author: String::from("Riot Games"),
                    created: FixedOffset::east_opt(1 * 3600)
                        .unwrap()
                        .with_ymd_and_hms(2023, 12, 24, 22, 15, 45)
                        .unwrap()
                        .with_nanosecond(942_466_466)
                        .unwrap(),
                    updated: None,
                    publish_locations: vec![
                        String::from("client"),
                    ],
                    translations: vec![
                        Localized {
                            content: String::from("Maintenance begin"),
                            locale: String::from("en_US"),
                        },
                    ],
                }
            ],
        },
    ];

    let expected = format!("{{\
    \"maintenances\":{0},\
    \"incidents\":{1}\
    }}",
                           serde_json::to_string(&maintenances).unwrap(),
                           serde_json::to_string(&incidents).unwrap());

    let status_data = StatusData {
        maintenances,
        incidents,
    };
    let actual = serde_json::to_string(&status_data).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let maintenances = vec![
        Status {
            id: rng.gen(),
            created: FixedOffset::east_opt(1 * 3600)
                .unwrap()
                .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                .unwrap()
                .with_nanosecond(412_356_412)
                .unwrap(),
            updated: Some(
                FixedOffset::east_opt(1 * 3600)
                    .unwrap()
                    .with_ymd_and_hms(2023, 12, 25, 7, 13, 45)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            ),
            archived: None,
            incident_severity: Some(String::from("warning")),
            maintenance_status: String::from("in_progress"),
            titles: vec![
                Localized {
                    content: String::from("Emergency Maintenance In Progress"),
                    locale: String::from("en_US"),
                },
            ],
            platforms: vec![
                String::from("windows"),
            ],
            updates: vec![
                StatusUpdate {
                    id: rng.gen(),
                    publish: false,
                    author: String::from("Riot Games"),
                    created: FixedOffset::east_opt(1 * 3600)
                        .unwrap()
                        .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                        .unwrap()
                        .with_nanosecond(942_466_466)
                        .unwrap(),
                    updated: None,
                    publish_locations: vec![
                        String::from("client"),
                        String::from("website"),
                    ],
                    translations: vec![
                        Localized {
                            content: String::from("Maintenance begin"),
                            locale: String::from("en_US"),
                        },
                    ],
                }
            ],
        },
    ];
    let incidents = vec![
        Status {
            id: rng.gen(),
            created: FixedOffset::east_opt(1 * 3600)
                .unwrap()
                .with_ymd_and_hms(2023, 12, 24, 22, 13, 45)
                .unwrap()
                .with_nanosecond(412_356_412)
                .unwrap(),
            updated: Some(
                FixedOffset::east_opt(1 * 3600)
                    .unwrap()
                    .with_ymd_and_hms(2023, 12, 25, 7, 14, 45)
                    .unwrap()
                    .with_nanosecond(0)
                    .unwrap()
            ),
            archived: None,
            incident_severity: Some(String::from("warning")),
            maintenance_status: String::from("in_progress"),
            titles: vec![
                Localized {
                    content: String::from("Emergency Maintenance In Progress"),
                    locale: String::from("en_US"),
                },
            ],
            platforms: vec![
                String::from("windows"),
            ],
            updates: vec![
                StatusUpdate {
                    id: rng.gen(),
                    publish: false,
                    author: String::from("Riot Games"),
                    created: FixedOffset::east_opt(1 * 3600)
                        .unwrap()
                        .with_ymd_and_hms(2023, 12, 24, 22, 15, 45)
                        .unwrap()
                        .with_nanosecond(942_466_466)
                        .unwrap(),
                    updated: None,
                    publish_locations: vec![
                        String::from("client"),
                    ],
                    translations: vec![
                        Localized {
                            content: String::from("Maintenance begin"),
                            locale: String::from("en_US"),
                        },
                    ],
                }
            ],
        },
    ];

    let json = format!("{{\
    \"maintenances\":{0},\
    \"incidents\":{1}\
    }}",
                       serde_json::to_string(&maintenances).unwrap(),
                       serde_json::to_string(&incidents).unwrap());

    let expected = StatusData {
        maintenances,
        incidents,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
