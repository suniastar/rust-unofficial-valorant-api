use rust_unofficial_valorant_api::types::Localized;

#[test]
fn serialize_str() {
    let content = "Maintenance";
    let locale = "en_US";

    let expected = format!("{{\
    \"content\":\"{content}\",\
    \"locale\":\"{locale}\"\
    }}");

    let localized = Localized {
        content,
        locale,
    };
    let actual = serde_json::to_string(&localized).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_string() {
    let content = format!("Maintenance");
    let locale = "en_US";

    let expected = format!("{{\
    \"content\":\"{content}\",\
    \"locale\":\"{locale}\"\
    }}");

    let localized = Localized {
        content,
        locale,
    };
    let actual = serde_json::to_string(&localized).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_u128() {
    let content = (1u128 << 127) + 1u128;
    let locale = "en_US";

    let expected = format!("{{\
    \"content\":{content},\
    \"locale\":\"{locale}\"\
    }}");

    let localized = Localized {
        content,
        locale,
    };
    let actual = serde_json::to_string(&localized).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_str() {
    let content = "Maintenance";
    let locale = "en_US";

    let json = format!("{{\
    \"content\":\"{content}\",\
    \"locale\":\"{locale}\"\
    }}");

    let expected = Localized {
        content,
        locale,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_string() {
    let content = format!("Maintenance");
    let locale = "en_US";

    let json = format!("{{\
    \"content\":\"{content}\",\
    \"locale\":\"{locale}\"\
    }}");

    let expected = Localized {
        content,
        locale,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_u128() {
    let content = (1u128 << 127) + 1u128;
    let locale = "en_US";

    let json = format!("{{\
    \"content\":{content},\
    \"locale\":\"{locale}\"\
    }}");

    let expected = Localized {
        content,
        locale,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
