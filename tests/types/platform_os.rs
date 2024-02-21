use rust_unofficial_valorant_api::types::PlatformOs;

#[test]
fn serialize() {
    let name = String::from("Windows");
    let version = String::from("10.0.22000.1.768.64bit");

    let expected = format!("{{\
    \"name\":\"{name}\",\
    \"version\":\"{version}\"\
    }}");

    let os = PlatformOs {
        name,
        version,
    };
    let actual = serde_json::to_string(&os).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let name = String::from("Windows");
    let version = String::from("10.0.22000.1.768.64bit");

    let json = format!("{{\
    \"name\":\"{name}\",\
    \"version\":\"{version}\"\
    }}");

    let expected = PlatformOs {
        name,
        version,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
