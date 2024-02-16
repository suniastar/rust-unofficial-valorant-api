use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MatchKillAssistant, PlayerTeam};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;

    let expected = format!("{{\
    \"assistant_puuid\":\"{id}\",\
    \"assistant_display_name\":\"{display_name}\",\
    \"assistant_team\":\"{team}\"\
    }}");

    let assistant = MatchKillAssistant {
        id,
        display_name,
        team,
    };
    let actual = serde_json::to_string(&assistant).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;

    let json = format!("{{\
    \"assistant_puuid\":\"{id}\",\
    \"assistant_display_name\":\"{display_name}\",\
    \"assistant_team\":\"{team}\"\
    }}");

    let expected = MatchKillAssistant {
        id,
        display_name,
        team,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
