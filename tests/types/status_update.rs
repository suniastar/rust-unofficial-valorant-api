use chrono::{FixedOffset, SecondsFormat, TimeZone};
use rand::Rng;

use rust_unofficial_valorant_api::types::{Localized, StatusUpdate};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let publish: bool = rng.gen();
    let author = "Frederik Enste";
    let created = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 12, 0, 0)
        .unwrap();
    let updated = Some(FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 18, 14, 53)
        .unwrap()
    );
    let publish_locations: Vec<&str> = vec![
        "client",
        "website",
    ];
    let translations: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];

    let expected = format!("{{\
    \"id\":{id},\
    \"publish\":{publish},\
    \"author\":\"{author}\",\
    \"created_at\":\"{0}\",\
    \"updated_at\":\"{1}\",\
    \"publish_locations\":{2},\
    \"translations\":{3}\
    }}",
                           created.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           updated.unwrap().to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           serde_json::to_string(&publish_locations).unwrap(),
                           serde_json::to_string(&translations).unwrap(),
    );

    let status_update = StatusUpdate {
        id,
        publish,
        author,
        created,
        updated,
        publish_locations,
        translations,
    };
    let actual = serde_json::to_string(&status_update).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let publish: bool = rng.gen();
    let author = "Frederik Enste";
    let created = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 12, 0, 0)
        .unwrap();
    let updated = Some(FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 18, 14, 53)
        .unwrap());
    let publish_locations: Vec<&str> = vec![
        "client",
        "website",
    ];
    let translations: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];

    let json = format!("{{\
    \"id\":{id},\
    \"publish\":{publish},\
    \"author\":\"{author}\",\
    \"created_at\":\"{0}\",\
    \"updated_at\":\"{1}\",\
    \"publish_locations\":{2},\
    \"translations\":{3}\
    }}",
                       created.to_rfc3339(),
                       updated.unwrap().to_rfc3339(),
                       serde_json::to_string(&publish_locations).unwrap(),
                       serde_json::to_string(&translations).unwrap(),
    );

    let expected = StatusUpdate {
        id,
        publish,
        author,
        created,
        updated,
        publish_locations,
        translations,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn serialize_null() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let publish: bool = rng.gen();
    let author = "Frederik Enste";
    let created = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 12, 0, 0)
        .unwrap();
    let updated = None;
    let publish_locations: Vec<&str> = vec![
        "client",
        "website",
    ];
    let translations: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];

    let expected = format!("{{\
    \"id\":{id},\
    \"publish\":{publish},\
    \"author\":\"{author}\",\
    \"created_at\":\"{0}\",\
    \"updated_at\":{1},\
    \"publish_locations\":{2},\
    \"translations\":{3}\
    }}",
                           created.to_rfc3339(),
                           "null",
                           serde_json::to_string(&publish_locations).unwrap(),
                           serde_json::to_string(&translations).unwrap(),
    );

    let status_update = StatusUpdate {
        id,
        publish,
        author,
        created,
        updated,
        publish_locations,
        translations,
    };
    let actual = serde_json::to_string(&status_update).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_null() {
    let mut rng = rand::thread_rng();

    let id: u32 = rng.gen();
    let publish: bool = rng.gen();
    let author = "Frederik Enste";
    let created = FixedOffset::east_opt(2 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 09, 22, 12, 0, 0)
        .unwrap();
    let updated = None;
    let publish_locations: Vec<&str> = vec![
        "client",
        "website",
    ];
    let translations: Vec<Localized<&str>> = vec![
        Localized {
            content: "Emergency Maintenance In Progress",
            locale: "en_US",
        },
    ];

    let json = format!("{{\
    \"id\":{id},\
    \"publish\":{publish},\
    \"author\":\"{author}\",\
    \"created_at\":\"{0}\",\
    \"updated_at\":{1},\
    \"publish_locations\":{2},\
    \"translations\":{3}\
    }}",
                       created.to_rfc3339(),
                       "null",
                       serde_json::to_string(&publish_locations).unwrap(),
                       serde_json::to_string(&translations).unwrap(),
    );

    let expected = StatusUpdate {
        id,
        publish,
        author,
        created,
        updated,
        publish_locations,
        translations,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
