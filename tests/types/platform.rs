use rust_unofficial_valorant_api::types::{Platform, PlatformOs, PlatformType};

#[test]
fn serialize() {
    let type_ = PlatformType::PC;
    let os = PlatformOs {
        name: String::from("Windows"),
        version: String::from("10.0.22000.1.768.64bit"),
    };

    let expected = format!("{{\
    \"type\":\"{type_}\",\
    \"os\":{0}\
    }}",
                           serde_json::to_string(&os).unwrap(),
    );

    let platform = Platform {
        type_,
        os,
    };
    let actual = serde_json::to_string(&platform).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let type_ = PlatformType::PC;
    let os = PlatformOs {
        name: String::from("Windows"),
        version: String::from("10.0.22000.1.768.64bit"),
    };

    let json = format!("{{\
    \"type\":\"{type_}\",\
    \"os\":{0}\
    }}",
                       serde_json::to_string(&os).unwrap(),
    );

    let expected = Platform {
        type_,
        os,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
