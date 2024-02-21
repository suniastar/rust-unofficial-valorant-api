use rand::Rng;

use rust_unofficial_valorant_api::types::{PlayerBehavior, PlayerBehaviorFriendlyFire};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let afk_rounds: u32 = rng.gen();
    let friendly_fire = PlayerBehaviorFriendlyFire {
        incoming: rng.gen(),
        outgoing: rng.gen(),
    };
    let rounds_in_spawn: u32 = rng.gen();

    let expected = format!("{{\
    \"afk_rounds\":{afk_rounds},\
    \"friendly_fire\":{0},\
    \"rounds_in_spawn\":{rounds_in_spawn}\
    }}",
                           serde_json::to_string(&friendly_fire).unwrap(),
    );

    let behaviour = PlayerBehavior {
        afk_rounds,
        friendly_fire,
        rounds_in_spawn,
    };
    let actual = serde_json::to_string(&behaviour).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let afk_rounds: u32 = rng.gen();
    let friendly_fire = PlayerBehaviorFriendlyFire {
        incoming: rng.gen(),
        outgoing: rng.gen(),
    };
    let rounds_in_spawn: u32 = rng.gen();

    let json = format!("{{\
    \"afk_rounds\":{afk_rounds},\
    \"friendly_fire\":{0},\
    \"rounds_in_spawn\":{rounds_in_spawn}\
    }}",
                       serde_json::to_string(&friendly_fire).unwrap(),
    );

    let expected = PlayerBehavior {
        afk_rounds,
        friendly_fire,
        rounds_in_spawn,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
