use rand::Rng;

use rust_unofficial_valorant_api::types::PlayerBehaviorFriendlyFire;

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let incoming: u32 = rng.gen();
    let outgoing: u32 = rng.gen();

    let expected = format!("{{\
    \"incoming\":{incoming},\
    \"outgoing\":{outgoing}\
    }}");

    let friendly_fire = PlayerBehaviorFriendlyFire {
        incoming,
        outgoing,
    };
    let actual = serde_json::to_string(&friendly_fire).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let incoming: u32 = rng.gen();
    let outgoing: u32 = rng.gen();

    let json = format!("{{\
    \"incoming\":{incoming},\
    \"outgoing\":{outgoing}\
    }}");

    let expected = PlayerBehaviorFriendlyFire {
        incoming,
        outgoing,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
