use uuid::Uuid;

use rust_unofficial_valorant_api::types::MatchMetadataPremierInfo;

#[test]
fn serialize() {
    let tournament_id = Uuid::new_v4();
    let matchup_id = Uuid::new_v4();

    let expected = format!("{{\
    \"tournament_id\":\"{tournament_id}\",\
    \"matchup_id\":\"{matchup_id}\"\
    }}");

    let premier_info = MatchMetadataPremierInfo {
        tournament_id,
        matchup_id,
    };
    let actual = serde_json::to_string(&premier_info).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let tournament_id = Uuid::new_v4();
    let matchup_id = Uuid::new_v4();

    let json = format!("{{\
    \"tournament_id\":\"{tournament_id}\",\
    \"matchup_id\":\"{matchup_id}\"\
    }}");

    let expected = MatchMetadataPremierInfo {
        tournament_id,
        matchup_id,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
