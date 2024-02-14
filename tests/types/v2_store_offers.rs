use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{OfferTier, OfferType, V2StoreOffer, V2StoreOffersData};

#[test]
fn serialize() {
    let offers = vec![
        V2StoreOffer {
            offer_id: Uuid::new_v4(),
            cost: 13142,
            name: String::from("cool Vandal"),
            icon_url: Some(Url::parse("https://google.com/search?q=vandal").unwrap()),
            type_: OfferType::SkinLevel,
            skin_id: Some(Uuid::new_v4()),
            offer_tier: Some(
                OfferTier {
                    name: String::from("Cool Tier"),
                    dev_name: String::from("cool"),
                    icon_url: Url::parse("https://google.com/search?q=tier").unwrap(),
                }
            ),
        },
    ];

    let expected = format!("{{\
    \"offers\":{0}\
    }}",
                           serde_json::to_string(&offers).unwrap(),
    );

    let v2_store_offers_data = V2StoreOffersData {
        offers
    };
    let actual = serde_json::to_string(&v2_store_offers_data).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let offers = vec![
        V2StoreOffer {
            offer_id: Uuid::new_v4(),
            cost: 13142,
            name: String::from("cool Vandal"),
            icon_url: Some(Url::parse("https://google.com/search?q=vandal").unwrap()),
            type_: OfferType::SkinLevel,
            skin_id: Some(Uuid::new_v4()),
            offer_tier: Some(
                OfferTier {
                    name: String::from("Cool Tier"),
                    dev_name: String::from("cool"),
                    icon_url: Url::parse("https://google.com/search?q=tier").unwrap(),
                }
            ),
        },
    ];

    let json = format!("{{\
    \"offers\":{0}\
    }}",
                       serde_json::to_string(&offers).unwrap(),
    );

    let expected = V2StoreOffersData {
        offers
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
