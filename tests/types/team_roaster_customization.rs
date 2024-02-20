use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::TeamRoasterCustomization;

#[test]
fn serialize() {
    let icon = Uuid::new_v4();
    let image_url = Url::parse("https://fenste.de/images/profile.jpg").unwrap();
    let color_primary = 0xFF123456;
    let color_secondary = 0xFF789ABC;
    let color_tertiary = 0xFFDEF123;

    let expected = format!("{{\
    \"icon\":\"{icon}\",\
    \"image\":\"{image_url}\",\
    \"primary_color\":\"#123456\",\
    \"secondary_color\":\"#789abc\",\
    \"tertiary_color\":\"#def123\"\
    }}");

    let customization = TeamRoasterCustomization {
        icon,
        image_url,
        color_primary,
        color_secondary,
        color_tertiary,
    };
    let actual = serde_json::to_string(&customization).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let icon = Uuid::new_v4();
    let image_url = Url::parse("https://fenste.de/images/profile.jpg").unwrap();
    let color_primary = 0xFF123456;
    let color_secondary = 0xFF789ABC;
    let color_tertiary = 0xFFDEF123;

    let json = format!("{{\
    \"icon\":\"{icon}\",\
    \"image\":\"{image_url}\",\
    \"primary_color\":\"#123456\",\
    \"secondary_color\":\"#789abc\",\
    \"tertiary_color\":\"#def123\"\
    }}");

    let expected = TeamRoasterCustomization {
        icon,
        image_url,
        color_primary,
        color_secondary,
        color_tertiary,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
