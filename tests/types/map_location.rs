use rust_unofficial_valorant_api::types::MapLocation;

#[test]
fn serialize() {
    let x: i32 = i32::MAX;
    let y: i32 = i32::MIN;

    let expected = format!("{{\
    \"x\":{x},\
    \"y\":{y}\
    }}");

    let map_location = MapLocation {
        x,
        y,
    };
    let actual = serde_json::to_string(&map_location).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let x: i32 = i32::MAX;
    let y: i32 = i32::MIN;

    let json = format!("{{\
    \"x\":{x},\
    \"y\":{y}\
    }}");

    let expected = MapLocation {
        x,
        y,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
