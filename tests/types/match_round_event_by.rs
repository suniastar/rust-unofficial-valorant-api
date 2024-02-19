use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MatchRoundEventBy, PlayerTeam};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("Name");
    let team = PlayerTeam::Blue;

    let expected = format!("{{\
    \"puuid\":\"{id}\",\
    \"display_name\":\"{display_name}\",\
    \"team\":\"{team}\"\
    }}");

    let event_by = MatchRoundEventBy {
        id,
        display_name,
        team,
    };
    let actual = serde_json::to_string(&event_by).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("Name");
    let team = PlayerTeam::Blue;

    let json = format!("{{\
    \"puuid\":\"{id}\",\
    \"display_name\":\"{display_name}\",\
    \"team\":\"{team}\"\
    }}");

    let expected = MatchRoundEventBy {
        id,
        display_name,
        team,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
