use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MatchRoundPlayerStatsDamageEvent, PlayerTeam};

#[test]
fn serialize() {
    let receiver_id = Uuid::new_v4();
    let receiver_display_name = String::from("t00manysecrets#EUW");
    let receiver_team = PlayerTeam::Blue;
    let damage = 282;
    let head_shots = 1;
    let body_shots = 3;
    let leg_shots = 0;

    let expected = format!("{{\
    \"receiver_puuid\":\"{receiver_id}\",\
    \"receiver_display_name\":\"{receiver_display_name}\",\
    \"receiver_team\":\"{receiver_team}\",\
    \"damage\":{damage},\
    \"headshots\":{head_shots},\
    \"bodyshots\":{body_shots},\
    \"legshots\":{leg_shots}\
    }}");

    let damage_event = MatchRoundPlayerStatsDamageEvent {
        receiver_id,
        receiver_display_name,
        receiver_team,
        damage,
        head_shots,
        body_shots,
        leg_shots,
    };
    let actual = serde_json::to_string(&damage_event).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let receiver_id = Uuid::new_v4();
    let receiver_display_name = String::from("t00manysecrets#EUW");
    let receiver_team = PlayerTeam::Blue;
    let damage = 282;
    let head_shots = 1;
    let body_shots = 3;
    let leg_shots = 0;

    let json = format!("{{\
    \"receiver_puuid\":\"{receiver_id}\",\
    \"receiver_display_name\":\"{receiver_display_name}\",\
    \"receiver_team\":\"{receiver_team}\",\
    \"damage\":{damage},\
    \"headshots\":{head_shots},\
    \"bodyshots\":{body_shots},\
    \"legshots\":{leg_shots}\
    }}");

    let expected = MatchRoundPlayerStatsDamageEvent {
        receiver_id,
        receiver_display_name,
        receiver_team,
        damage,
        head_shots,
        body_shots,
        leg_shots,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
