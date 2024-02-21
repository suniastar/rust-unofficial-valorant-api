use rand::Rng;

use rust_unofficial_valorant_api::types::PlayerEconomyCredits;

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let overall: u32 = rng.gen();
    let average: u32 = rng.gen();

    let expected = format!("{{\
    \"overall\":{overall},\
    \"average\":{average}\
    }}");

    let credits = PlayerEconomyCredits {
        overall,
        average,
    };
    let actual = serde_json::to_string(&credits).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let overall: u32 = rng.gen();
    let average: u32 = rng.gen();

    let json = format!("{{\
    \"overall\":{overall},\
    \"average\":{average}\
    }}");

    let expected = PlayerEconomyCredits {
        overall,
        average,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
