use rust_unofficial_valorant_api::types::ValorantApiError;

#[test]
fn serialize() {
    let code = 104;
    let message = "Invalid region";
    let details = "details";

    let expected = format!("{{\
    \"code\":{code},\
    \"message\":\"{message}\",\
    \"details\":\"{details}\"\
    }}");

    let error = ValorantApiError {
        code,
        message,
        details,
    };
    let actual = serde_json::to_string(&error).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let code = 104;
    let message = "Invalid region";
    let details = "details";

    let json = format!("{{\
    \"code\":{code},\
    \"message\":\"{message}\",\
    \"details\":\"{details}\"\
    }}");

    let expected = ValorantApiError {
        code,
        message,
        details,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
