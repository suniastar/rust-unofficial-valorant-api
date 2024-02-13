use chrono::{FixedOffset, SecondsFormat, TimeZone};
use rand::Rng;

use unofficial_valorant_api::types::{Localized, Status, StatusUpdate};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let created = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 22, 3, 52, 41)
        .unwrap();
    let updated = Some(FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 23, 3, 59, 59)
        .unwrap()
    );
    let archived = Some(FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 23, 4, 0, 0)
        .unwrap()
    );
    let incident_severity = Some("warning");
    let maintenance_status = "in_progress";
    let titles: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];
    let platforms: Vec<&str> = vec![
        "windows",
    ];
    let updates: Vec<StatusUpdate> = vec![
        StatusUpdate {
            id: rng.gen(),
            publish: rng.gen(),
            author: "Frederik Enste",
            created,
            updated: None,
            publish_locations: vec![
                "client",
                "website",
            ],
            translations: vec![
                Localized {
                    content: "Maintenance begin",
                    locale: "en_US",
                },
            ],
        },
    ];

    let expected = format!("{{\
    \"id\":{id},\
    \"created_at\":\"{0}\",\
    \"updated_at\":\"{1}\",\
    \"archive_at\":\"{2}\",\
    \"incident_severity\":\"{3}\",\
    \"maintenance_status\":\"{maintenance_status}\",\
    \"titles\":{4},\
    \"platforms\":{5},\
    \"updates\":{6}\
    }}",
                           created.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           updated.unwrap().to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           archived.unwrap().to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           incident_severity.unwrap(),
                           serde_json::to_string(&titles).unwrap(),
                           serde_json::to_string(&platforms).unwrap(),
                           serde_json::to_string(&updates).unwrap(),
    );

    let status = Status {
        id,
        created,
        updated,
        archived,
        incident_severity,
        maintenance_status,
        titles,
        platforms,
        updates,
    };
    let actual = serde_json::to_string(&status).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let created = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 22, 3, 52, 41)
        .unwrap();
    let updated = Some(FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 23, 3, 59, 59)
        .unwrap()
    );
    let archived = Some(FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 23, 4, 0, 0)
        .unwrap()
    );
    let incident_severity = Some("warning");
    let maintenance_status = "in_progress";
    let titles: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];
    let platforms: Vec<&str> = vec![
        "windows",
    ];
    let updates: Vec<StatusUpdate> = vec![
        StatusUpdate {
            id: rng.gen(),
            publish: rng.gen(),
            author: "Frederik Enste",
            created,
            updated: None,
            publish_locations: vec![
                "client",
                "website",
            ],
            translations: vec![
                Localized {
                    content: "Maintenance begin",
                    locale: "en_US",
                },
            ],
        },
    ];

    let json = format!("{{\
    \"id\":{id},\
    \"created_at\":\"{0}\",\
    \"updated_at\":\"{1}\",\
    \"archive_at\":\"{2}\",\
    \"incident_severity\":\"{3}\",\
    \"maintenance_status\":\"{maintenance_status}\",\
    \"titles\":{4},\
    \"platforms\":{5},\
    \"updates\":{6}\
    }}",
                       created.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                       updated.unwrap().to_rfc3339_opts(SecondsFormat::AutoSi, true),
                       archived.unwrap().to_rfc3339_opts(SecondsFormat::AutoSi, true),
                       incident_severity.unwrap(),
                       serde_json::to_string(&titles).unwrap(),
                       serde_json::to_string(&platforms).unwrap(),
                       serde_json::to_string(&updates).unwrap(),
    );

    let expected = Status {
        id,
        created,
        updated,
        archived,
        incident_severity,
        maintenance_status,
        titles,
        platforms,
        updates,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn serialize_null() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let created = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 22, 3, 52, 41)
        .unwrap();
    let updated = None;
    let archived = None;
    let incident_severity = None;
    let maintenance_status = "in_progress";
    let titles: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];
    let platforms: Vec<&str> = vec![
        "windows",
    ];
    let updates: Vec<StatusUpdate> = vec![
        StatusUpdate {
            id: rng.gen(),
            publish: rng.gen(),
            author: "Frederik Enste",
            created,
            updated: None,
            publish_locations: vec![
                "client",
                "website",
            ],
            translations: vec![
                Localized {
                    content: "Maintenance begin",
                    locale: "en_US",
                },
            ],
        },
    ];

    let expected = format!("{{\
    \"id\":{id},\
    \"created_at\":\"{0}\",\
    \"updated_at\":{1},\
    \"archive_at\":{2},\
    \"incident_severity\":{3},\
    \"maintenance_status\":\"{maintenance_status}\",\
    \"titles\":{4},\
    \"platforms\":{5},\
    \"updates\":{6}\
    }}",
                           created.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           "null",
                           "null",
                           "null",
                           serde_json::to_string(&titles).unwrap(),
                           serde_json::to_string(&platforms).unwrap(),
                           serde_json::to_string(&updates).unwrap(),
    );

    let status = Status {
        id,
        created,
        updated,
        archived,
        incident_severity,
        maintenance_status,
        titles,
        platforms,
        updates,
    };
    let actual = serde_json::to_string(&status).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_null() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let created = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 9, 22, 3, 52, 41)
        .unwrap();
    let updated = None;
    let archived = None;
    let incident_severity = None;
    let maintenance_status = "in_progress";
    let titles: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];
    let platforms: Vec<&str> = vec![
        "windows",
    ];
    let updates: Vec<StatusUpdate> = vec![
        StatusUpdate {
            id: rng.gen(),
            publish: rng.gen(),
            author: "Frederik Enste",
            created,
            updated: None,
            publish_locations: vec![
                "client",
                "website",
            ],
            translations: vec![
                Localized {
                    content: "Maintenance begin",
                    locale: "en_US",
                },
            ],
        },
    ];

    let json = format!("{{\
    \"id\":{id},\
    \"created_at\":\"{0}\",\
    \"updated_at\":{1},\
    \"archive_at\":{2},\
    \"incident_severity\":{3},\
    \"maintenance_status\":\"{maintenance_status}\",\
    \"titles\":{4},\
    \"platforms\":{5},\
    \"updates\":{6}\
    }}",
                       created.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                       "null",
                       "null",
                       "null",
                       serde_json::to_string(&titles).unwrap(),
                       serde_json::to_string(&platforms).unwrap(),
                       serde_json::to_string(&updates).unwrap(),
    );

    let expected = Status {
        id,
        created,
        updated,
        archived,
        incident_severity,
        maintenance_status,
        titles,
        platforms,
        updates,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
