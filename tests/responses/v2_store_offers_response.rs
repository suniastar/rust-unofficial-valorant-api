use url::Url;
use uuid::uuid;

use rust_unofficial_valorant_api::types::{OfferTier, OfferType, V1StatusData, V2StoreOffer, V2StoreOffersData, ValorantApiError, ValorantApiResponse};

use crate::util::read_resource;

#[test]
fn deserialize_bad_request() {
    let json = read_resource("v2-store-offers/bad_request.json");

    let expected: ValorantApiResponse<V1StatusData> = ValorantApiResponse {
        status: 400,
        name: None,
        tag: None,
        errors: Some(
            vec![
                ValorantApiError {
                    code: 0,
                    message: String::from("string"),
                    details: String::from("string"),
                }
            ]
        ),
        results: None,
        data: None,
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_example() {
    let json = read_resource("v2-store-offers/ok_example.json");

    let expected: ValorantApiResponse<V2StoreOffersData> = ValorantApiResponse {
        status: 200,
        name: None,
        tag: None,
        errors: None,
        results: None,
        data: Some(
            V2StoreOffersData {
                offers: vec![
                    V2StoreOffer {
                        offer_id: uuid!("a3dba920-44ee-d7c5-5227-99a80aee3bd9"),
                        cost: 2175,
                        name: String::from("Araxys Vandal"),
                        icon_url: Some(Url::parse("https://media.valorant-api.com/weaponskinlevels/a3dba920-44ee-d7c5-5227-99a80aee3bd9/displayicon.png").unwrap()),
                        type_: OfferType::SkinLevel,
                        skin_id: Some(uuid!("4c926aa9-4f26-bc80-c486-9b888333373f")),
                        offer_tier: Some(
                            OfferTier {
                                name: String::from("Deluxe Edition"),
                                dev_name: String::from("Exclusive"),
                                icon_url: Url::parse("https://media.valorant-api.com/contenttiers/e046854e-406c-37f4-6607-19a9ba8426fc/displayicon.png").unwrap(),
                            }
                        ),
                    }
                ],
            }
        ),
    };

    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
