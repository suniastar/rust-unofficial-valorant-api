use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{TeamRoaster, TeamRoasterCustomization};

#[test]
fn serialize() {
    let members = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let name = String::from("Team");
    let tag = String::from("Tag");
    let customization = TeamRoasterCustomization {
        icon: Uuid::new_v4(),
        image_url: Url::parse("https://fenste.de/images/profile.jpg").unwrap(),
        color_primary: 0xFF123456,
        color_secondary: 0xFF789ABC,
        color_tertiary: 0xFFDEF123,
    };

    let expected = format!("{{\
    \"members\":[\
    \"{0}\",\
    \"{1}\",\
    \"{2}\",\
    \"{3}\",\
    \"{4}\"\
    ],\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"customization\":{5}\
    }}", members[0], members[1], members[2], members[3], members[4], serde_json::to_string(&customization).unwrap());

    let roaster = TeamRoaster {
        members,
        name,
        tag,
        customization,
    };
    let actual = serde_json::to_string(&roaster).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let members = vec![Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];
    let name = String::from("Team");
    let tag = String::from("Tag");
    let customization = TeamRoasterCustomization {
        icon: Uuid::new_v4(),
        image_url: Url::parse("https://fenste.de/images/profile.jpg").unwrap(),
        color_primary: 0xFF123456,
        color_secondary: 0xFF789ABC,
        color_tertiary: 0xFFDEF123,
    };

    let json = format!("{{\
    \"members\":[\
    \"{0}\",\
    \"{1}\",\
    \"{2}\",\
    \"{3}\",\
    \"{4}\"\
    ],\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"customization\":{5}\
    }}", members[0], members[1], members[2], members[3], members[4], serde_json::to_string(&customization).unwrap());

    let expected = TeamRoaster {
        members,
        name,
        tag,
        customization,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
