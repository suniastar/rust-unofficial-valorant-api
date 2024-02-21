use rand::Rng;

use rust_unofficial_valorant_api::types::PlayerStats;

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let score: u32 = rng.gen();
    let kills: u32 = rng.gen();
    let deaths: u32 = rng.gen();
    let assists: u32 = rng.gen();
    let body_shots: u32 = rng.gen();
    let head_shots: u32 = rng.gen();
    let leg_shots: u32 = rng.gen();

    let expected = format!("{{\
    \"score\":{score},\
    \"kills\":{kills},\
    \"deaths\":{deaths},\
    \"assists\":{assists},\
    \"bodyshots\":{body_shots},\
    \"headshots\":{head_shots},\
    \"legshots\":{leg_shots}\
    }}");

    let stats = PlayerStats {
        score,
        kills,
        deaths,
        assists,
        body_shots,
        head_shots,
        leg_shots,
    };
    let actual = serde_json::to_string(&stats).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let score: u32 = rng.gen();
    let kills: u32 = rng.gen();
    let deaths: u32 = rng.gen();
    let assists: u32 = rng.gen();
    let body_shots: u32 = rng.gen();
    let head_shots: u32 = rng.gen();
    let leg_shots: u32 = rng.gen();

    let json = format!("{{\
    \"score\":{score},\
    \"kills\":{kills},\
    \"deaths\":{deaths},\
    \"assists\":{assists},\
    \"bodyshots\":{body_shots},\
    \"headshots\":{head_shots},\
    \"legshots\":{leg_shots}\
    }}");

    let expected = PlayerStats {
        score,
        kills,
        deaths,
        assists,
        body_shots,
        head_shots,
        leg_shots,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
