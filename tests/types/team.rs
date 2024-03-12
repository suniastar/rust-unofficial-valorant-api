use rand::Rng;
use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Team, TeamRoaster, TeamRoasterCustomization};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let has_won: bool = rng.gen();
    let rounds_won: u32 = rng.gen();
    let rounds_lost: u32 = rng.gen();
    let roaster = TeamRoaster {
        members: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
        name: String::from("Team"),
        tag: String::from("Tag"),
        customization: TeamRoasterCustomization {
            icon: Uuid::new_v4(),
            image_url: Url::parse("https://fenste.de/images/profile.jpg").unwrap(),
            color_primary: 0xFF123456,
            color_secondary: 0xFF789ABC,
            color_tertiary: 0xFFDEF123,
        },
    };

    let expected = format!("{{\
    \"has_won\":{has_won},\
    \"rounds_won\":{rounds_won},\
    \"rounds_lost\":{rounds_lost},\
    \"roaster\":{0}\
    }}",
                           serde_json::to_string(&roaster).unwrap(),
    );

    let roaster = Team {
        has_won,
        rounds_won,
        rounds_lost,
        roaster,
    };
    let actual = serde_json::to_string(&roaster).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let has_won: bool = rng.gen();
    let rounds_won: u32 = rng.gen();
    let rounds_lost: u32 = rng.gen();
    let roaster = TeamRoaster {
        members: vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()],
        name: String::from("Team"),
        tag: String::from("Tag"),
        customization: TeamRoasterCustomization {
            icon: Uuid::new_v4(),
            image_url: Url::parse("https://fenste.de/images/profile.jpg").unwrap(),
            color_primary: 0xFF123456,
            color_secondary: 0xFF789ABC,
            color_tertiary: 0xFFDEF123,
        },
    };

    let json = format!("{{\
    \"has_won\":{has_won},\
    \"rounds_won\":{rounds_won},\
    \"rounds_lost\":{rounds_lost},\
    \"roaster\":{0}\
    }}",
                       serde_json::to_string(&roaster).unwrap(),
    );

    let expected = Team {
        has_won,
        rounds_won,
        rounds_lost,
        roaster,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
