use std::collections::HashMap;

use rust_unofficial_valorant_api::types::{ValorantApiError, ValorantApiResponse};

#[test]
fn serialize_ok_u32() {
    let status = 200;
    let errors = None;
    let data = Some(u32::MAX);

    let expected = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":{0}\
    }}", data.unwrap());

    let response = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_ok_str() {
    let status = 200;
    let errors = None;
    let data = Some("test");

    let expected = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":\"{0}\"\
    }}", data.unwrap());

    let response = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_ok_map() {
    let status = 200;
    let errors = None;
    let data = Some(HashMap::from([("test", "me")]));

    let expected = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":{0}\
    }}", serde_json::to_string(&data).unwrap());

    let response = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_ok_error() {
    let status = 400;
    let errors = Some(
        vec![
            ValorantApiError {
                code: 104,
                message: "Invalid region",
                details: "details",
            },
        ]
    );
    let data = None::<()>;

    let expected = format!("{{\
    \"status\":{status},\
    \"errors\":{0},\
    \"data\":null\
    }}", serde_json::to_string(&errors).unwrap());

    let response = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_u32() {
    let status = 200;
    let errors = None;
    let data = Some(u32::MAX);

    let json = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":{0}\
    }}", data.unwrap());

    let expected = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_str() {
    let status = 200;
    let errors = None;
    let data = Some("test");

    let json = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":\"{0}\"\
    }}", data.unwrap());

    let expected = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_map() {
    let status = 200;
    let errors = None;
    let data = Some(HashMap::from([("test", "me")]));

    let json = format!("{{\
    \"status\":{status},\
    \"errors\":null,\
    \"data\":{0}\
    }}", serde_json::to_string(&data).unwrap());

    let expected = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_error() {
    let status = 400;
    let errors = Some(
        vec![
            ValorantApiError {
                code: 104,
                message: "Invalid region",
                details: "details",
            },
        ]
    );
    let data = None::<()>;

    let json = format!("{{\
    \"status\":{status},\
    \"errors\":{0},\
    \"data\":null\
    }}", serde_json::to_string(&errors).unwrap());

    let expected = ValorantApiResponse {
        status,
        errors,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
