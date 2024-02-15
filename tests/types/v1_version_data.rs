use chrono::{FixedOffset, NaiveDate, SecondsFormat, Timelike, TimeZone};

use rust_unofficial_valorant_api::types::{Region, V1VersionData};

#[test]
fn serialize() {
    let region = Region::Europe;
    let branch = String::from("release");
    let build_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let build_version = String::from("04.69.42.696969");
    let last_checked = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2023, 1, 6, 12, 44, 57)
        .unwrap()
        .with_nanosecond(234_231_233)
        .unwrap();
    let version = 42;
    let version_for_api = String::from("release-full-name");

    let expected = format!("{{\
    \"region\":{0},\
    \"branch\":\"{branch}\",\
    \"build_date\":\"{1}\",\
    \"build_ver\":\"{build_version}\",\
    \"last_checked\":\"{2}\",\
    \"version\":{version},\
    \"version_for_api\":\"{version_for_api}\"\
    }}",
                           serde_json::to_string(&region).unwrap(),
                           build_date.format("%b %e %Y"),
                           last_checked.to_rfc3339_opts(SecondsFormat::AutoSi, true),
    );

    let version_data = V1VersionData {
        region,
        branch,
        build_date,
        build_version,
        last_checked,
        version,
        version_for_api,
    };
    let actual = serde_json::to_string(&version_data).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let region = Region::Europe;
    let branch = String::from("release");
    let build_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    let build_version = String::from("04.69.42.696969");
    let last_checked = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(2023, 1, 6, 12, 44, 57)
        .unwrap()
        .with_nanosecond(234_231_233)
        .unwrap();
    let version = 42;
    let version_for_api = String::from("release-full-name");

    let json = format!("{{\
    \"region\":{0},\
    \"branch\":\"{branch}\",\
    \"build_date\":\"{1}\",\
    \"build_ver\":\"{build_version}\",\
    \"last_checked\":\"{2}\",\
    \"version\":{version},\
    \"version_for_api\":\"{version_for_api}\"\
    }}",
                       serde_json::to_string(&region).unwrap(),
                       build_date.format("%b %e %Y"),
                       last_checked.to_rfc3339_opts(SecondsFormat::AutoSi, true),
    );

    let expected = V1VersionData {
        region,
        branch,
        build_date,
        build_version,
        last_checked,
        version,
        version_for_api,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
