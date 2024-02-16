use rust_unofficial_valorant_api::types::ValorantApiResults;

#[test]
fn serialize() {
    let total = 420;
    let returned = 10;
    let before = 10;
    let after = 400;

    let expected = format!("{{\
    \"total\":{total},\
    \"returned\":{returned},\
    \"before\":{before},\
    \"after\":{after}\
    }}");

    let valorant_api_results = ValorantApiResults {
        total,
        returned,
        before,
        after,
    };
    let actual = serde_json::to_string(&valorant_api_results).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let total = 420;
    let returned = 10;
    let before = 10;
    let after = 400;

    let json = format!("{{\
    \"total\":{total},\
    \"returned\":{returned},\
    \"before\":{before},\
    \"after\":{after}\
    }}");

    let expected = ValorantApiResults {
        total,
        returned,
        before,
        after,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
