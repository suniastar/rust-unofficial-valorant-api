use chrono::{FixedOffset, SecondsFormat, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::Website;

#[test]
fn serialize() {
    let banner_url = String::from("https://google.com/search?q=banner");
    let category = String::from("vct");
    let date = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 2, 23, 12, 0, 6)
        .unwrap()
        .with_nanosecond(342_262_234)
        .unwrap();
    let external_link = Some(String::from("https://google.com/search?q=external"));
    let title = String::from("Sample Title");
    let url = String::from("https://google.com/search?q=ducks");

    let expected = format!("{{\
    \"banner_url\":\"{banner_url}\",\
    \"category\":\"{category}\",\
    \"date\":\"{0}\",\
    \"external_link\":{1},\
    \"title\":\"{title}\",\
    \"url\":\"{url}\"\
    }}",
                           date.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           serde_json::to_string(&external_link).unwrap(),
    );

    let website_date = Website {
        banner_url,
        category,
        date,
        external_link,
        title,
        url,
    };
    let actual = serde_json::to_string(&website_date).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_null() {
    let banner_url = String::from("https://google.com/search?q=banner");
    let category = String::from("vct");
    let date = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 2, 23, 12, 0, 6)
        .unwrap()
        .with_nanosecond(342_262_234)
        .unwrap();
    let external_link = None;
    let title = String::from("Sample Title");
    let url = String::from("https://google.com/search?q=ducks");

    let expected = format!("{{\
    \"banner_url\":\"{banner_url}\",\
    \"category\":\"{category}\",\
    \"date\":\"{0}\",\
    \"external_link\":{1},\
    \"title\":\"{title}\",\
    \"url\":\"{url}\"\
    }}",
                           date.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                           serde_json::to_string(&external_link).unwrap(),
    );

    let website_date = Website {
        banner_url,
        category,
        date,
        external_link,
        title,
        url,
    };
    let actual = serde_json::to_string(&website_date).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let banner_url = String::from("https://google.com/search?q=banner");
    let category = String::from("vct");
    let date = FixedOffset::east_opt(1 * 3600)
        .unwrap()
        .with_ymd_and_hms(2023, 2, 23, 12, 0, 6)
        .unwrap()
        .with_nanosecond(342_262_234)
        .unwrap();
    let external_link = Some(String::from("https://google.com/search?q=external"));
    let title = String::from("Sample Title");
    let url = String::from("https://google.com/search?q=ducks");

    let json = format!("{{\
    \"banner_url\":\"{banner_url}\",\
    \"category\":\"{category}\",\
    \"date\":\"{0}\",\
    \"external_link\":{1},\
    \"title\":\"{title}\",\
    \"url\":\"{url}\"\
    }}",
                       date.to_rfc3339_opts(SecondsFormat::AutoSi, true),
                       serde_json::to_string(&external_link).unwrap(),
    );

    let expected = Website {
        banner_url,
        category,
        date,
        external_link,
        title,
        url,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}