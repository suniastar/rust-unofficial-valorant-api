use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Coach, PlayerTeam};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let team = PlayerTeam::Red;

    let expected = format!("{{\
    \"puuid\":\"{id}\",\
    \"team\":\"{team}\"\
    }}");

    let coach = Coach {
        id,
        team,
    };
    let actual = serde_json::to_string(&coach).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let team = PlayerTeam::Red;

    let json = format!("{{\
    \"puuid\":\"{id}\",\
    \"team\":\"{team}\"\
    }}");

    let expected = Coach {
        id,
        team,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
