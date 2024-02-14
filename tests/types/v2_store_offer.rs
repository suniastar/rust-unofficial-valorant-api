use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{OfferTier, OfferType, V2StoreOffer};

#[test]
fn serialize() {
    let offer_id = Uuid::new_v4();
    let cost = 13142;
    let name = String::from("cool Vandal");
    let icon_url = Some(Url::parse("https://google.com/search?q=vandal").unwrap());
    let type_ = OfferType::SkinLevel;
    let skin_id = Some(Uuid::new_v4());
    let offer_tier = Some(
        OfferTier {
            name: String::from("Cool Tier"),
            dev_name: String::from("cool"),
            icon_url: Url::parse("https://google.com/search?q=tier").unwrap(),
        }
    );

    let expected = format!("{{\
    \"offer_id\":\"{offer_id}\",\
    \"cost\":{cost},\
    \"name\":\"{name}\",\
    \"icon\":{0},\
    \"type\":{1},\
    \"skin_id\":{2},\
    \"content_tier\":{3}\
    }}",
                           serde_json::to_string(&icon_url).unwrap(),
                           serde_json::to_string(&type_).unwrap(),
                           serde_json::to_string(&skin_id).unwrap(),
                           serde_json::to_string(&offer_tier).unwrap(),
    );

    let v2_store_offer = V2StoreOffer {
        offer_id,
        cost,
        name,
        icon_url,
        type_,
        skin_id,
        offer_tier,
    };
    let actual = serde_json::to_string(&v2_store_offer).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_null() {
    let offer_id = Uuid::new_v4();
    let cost = 13142;
    let name = String::from("cool Vandal");
    let icon_url = None;
    let type_ = OfferType::SkinLevel;
    let skin_id = None;
    let offer_tier = None;

    let expected = format!("{{\
    \"offer_id\":\"{offer_id}\",\
    \"cost\":{cost},\
    \"name\":\"{name}\",\
    \"icon\":{0},\
    \"type\":{1},\
    \"skin_id\":{2},\
    \"content_tier\":{3}\
    }}",
                           serde_json::to_string(&icon_url).unwrap(),
                           serde_json::to_string(&type_).unwrap(),
                           serde_json::to_string(&skin_id).unwrap(),
                           serde_json::to_string(&offer_tier).unwrap(),
    );

    let v2_store_offer = V2StoreOffer {
        offer_id,
        cost,
        name,
        icon_url,
        type_,
        skin_id,
        offer_tier,
    };
    let actual = serde_json::to_string(&v2_store_offer).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let offer_id = Uuid::new_v4();
    let cost = 13142;
    let name = String::from("cool Vandal");
    let icon_url = Some(Url::parse("https://google.com/search?q=vandal").unwrap());
    let type_ = OfferType::SkinLevel;
    let skin_id = Some(Uuid::new_v4());
    let offer_tier = Some(
        OfferTier {
            name: String::from("Cool Tier"),
            dev_name: String::from("cool"),
            icon_url: Url::parse("https://google.com/search?q=tier").unwrap(),
        }
    );

    let json = format!("{{\
    \"offer_id\":\"{offer_id}\",\
    \"cost\":{cost},\
    \"name\":\"{name}\",\
    \"icon\":{0},\
    \"type\":{1},\
    \"skin_id\":{2},\
    \"content_tier\":{3}\
    }}",
                       serde_json::to_string(&icon_url).unwrap(),
                       serde_json::to_string(&type_).unwrap(),
                       serde_json::to_string(&skin_id).unwrap(),
                       serde_json::to_string(&offer_tier).unwrap(),
    );

    let expected = V2StoreOffer {
        offer_id,
        cost,
        name,
        icon_url,
        type_,
        skin_id,
        offer_tier,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
