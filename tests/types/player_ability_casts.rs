use rand::Rng;

use rust_unofficial_valorant_api::types::PlayerAbilityCasts;

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let c: u32 = rng.gen();
    let q: u32 = rng.gen();
    let e: u32 = rng.gen();
    let x: u32 = rng.gen();

    let expected = format!("{{\
    \"c_cast\":{c},\
    \"q_cast\":{q},\
    \"e_cast\":{e},\
    \"x_cast\":{x}\
    }}");

    let ability_casts = PlayerAbilityCasts {
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
    let mut rng = rand::thread_rng();

    let c: u32 = rng.gen();
    let q: u32 = rng.gen();
    let e: u32 = rng.gen();
    let x: u32 = rng.gen();

    let json = format!("{{\
    \"c_cast\":{c},\
    \"q_cast\":{q},\
    \"e_cast\":{e},\
    \"x_cast\":{x}\
    }}");

    let expected = PlayerAbilityCasts {
        c,
        q,
        e,
        x,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
