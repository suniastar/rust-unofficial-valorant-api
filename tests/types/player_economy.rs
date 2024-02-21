use rand::Rng;

use rust_unofficial_valorant_api::types::{PlayerEconomy, PlayerEconomyCredits};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let spent = PlayerEconomyCredits {
        overall: rng.gen(),
        average: rng.gen(),
    };
    let load_out_value = PlayerEconomyCredits {
        overall: rng.gen(),
        average: rng.gen(),
    };

    let expected = format!("{{\
    \"spent\":{0},\
    \"loadout_value\":{1}\
    }}",
                           serde_json::to_string(&spent).unwrap(),
                           serde_json::to_string(&load_out_value).unwrap(),
    );

    let economy = PlayerEconomy {
        spent,
        load_out_value,
    };
    let actual = serde_json::to_string(&economy).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let spent = PlayerEconomyCredits {
        overall: rng.gen(),
        average: rng.gen(),
    };
    let load_out_value = PlayerEconomyCredits {
        overall: rng.gen(),
        average: rng.gen(),
    };

    let json = format!("{{\
    \"spent\":{0},\
    \"loadout_value\":{1}\
    }}",
                       serde_json::to_string(&spent).unwrap(),
                       serde_json::to_string(&load_out_value).unwrap(),
    );

    let expected = PlayerEconomy {
        spent,
        load_out_value,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
