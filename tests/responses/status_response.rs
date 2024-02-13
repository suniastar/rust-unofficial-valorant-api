use chrono::{FixedOffset, Timelike, TimeZone};

use unofficial_valorant_api::types::{Localized, Status, StatusData, StatusUpdate, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_ok() {
    let json = read_resource("status_response_ok.json");

    let expected = ValorantApiResponse {
        status: 200,
        error: None,
        data: Some(StatusData {
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
                    incident_severity: Some("warning"),
                    maintenance_status: "in_progress",
                    titles: vec![
                        Localized {
                            content: "Emergency Maintenance In Progress",
                            locale: "en_US",
                        }
                    ],
                    platforms: vec![
                        "windows",
                    ],
                    updates: vec![
                        StatusUpdate {
                            id: 6658,
                            publish: true,
                            author: "Riot Games",
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
                                "riotclient",
                            ],
                            translations: vec![
                                Localized {
                                    content: "The platform is currently unavailable while we perform emergency maintenance.",
                                    locale: "en_US",
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
                    incident_severity: Some("warning"),
                    maintenance_status: "in_progress",
                    titles: vec![
                        Localized {
                            content: "Emergency Maintenance In Progress",
                            locale: "en_US",
                        }
                    ],
                    platforms: vec![
                        "windows",
                    ],
                    updates: vec![
                        StatusUpdate {
                            id: 6658,
                            publish: true,
                            author: "Riot Games",
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
                                "riotclient",
                            ],
                            translations: vec![
                                Localized {
                                    content: "The platform is currently unavailable while we perform emergency maintenance.",
                                    locale: "en_US",
                                }
                            ],
                        }
                    ],
                }
            ],
        }),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
