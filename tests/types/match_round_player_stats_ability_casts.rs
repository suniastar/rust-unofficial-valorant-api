use rust_unofficial_valorant_api::types::MatchRoundPlayerStatsAbilityCasts;

#[test]
fn serialize() {
    let c = 42;
    let q = 69;
    let e = 420;
    let x = 4;

    let expected = format!("{{\
    \"c_casts\":{c},\
    \"q_casts\":{q},\
    \"e_cast\":{e},\
    \"x_cast\":{x}\
    }}");

    let ability_casts = MatchRoundPlayerStatsAbilityCasts {
        c,
        q,
        e,
        x,
    };
    let actual = serde_json::to_string(&ability_casts).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let c = 42;
    let q = 69;
    let e = 420;
    let x = 4;

    let json = format!("{{\
    \"c_casts\":{c},\
    \"q_casts\":{q},\
    \"e_cast\":{e},\
    \"x_cast\":{x}\
    }}");

    let expected = MatchRoundPlayerStatsAbilityCasts {
        c,
        q,
        e,
        x,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
