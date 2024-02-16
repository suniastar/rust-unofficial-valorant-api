use std::collections::HashMap;

use rust_unofficial_valorant_api::types::{ValorantApiError, ValorantApiResponse, ValorantApiResults};

#[test]
fn serialize_ok_u32() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some(u32::MAX);

    let expected = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&errors).unwrap(),
                           serde_json::to_string(&results).unwrap(),
                           serde_json::to_string(&data).unwrap(),
    );

    let response = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_ok_string() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some("test");

    let expected = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&errors).unwrap(),
                           serde_json::to_string(&results).unwrap(),
                           serde_json::to_string(&data).unwrap(),
    );

    let response = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_ok_map() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some(HashMap::from([("test", "me")]));

    let expected = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&errors).unwrap(),
                           serde_json::to_string(&results).unwrap(),
                           serde_json::to_string(&data).unwrap(),
    );

    let response = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_error() {
    let status = 400;
    let name = None;
    let tag = None;
    let errors = Some(
        vec![
            ValorantApiError {
                code: 104,
                message: String::from("Invalid region"),
                details: String::from("details"),
            },
        ]
    );
    let results = None;
    let data = None::<()>;

    let expected = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&errors).unwrap(),
                           serde_json::to_string(&results).unwrap(),
                           serde_json::to_string(&data).unwrap(),
    );

    let response = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn serialize_result_with_player() {
    let status = 200;
    let name = Some(String::from("t00manysecrets"));
    let tag = Some(String::from("EUW"));
    let errors = None;
    let results = Some(
        ValorantApiResults {
            total: 420,
            returned: 10,
            before: 20,
            after: 390,
        }
    );
    let data = Some("data");

    let expected = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                           serde_json::to_string(&name).unwrap(),
                           serde_json::to_string(&tag).unwrap(),
                           serde_json::to_string(&errors).unwrap(),
                           serde_json::to_string(&results).unwrap(),
                           serde_json::to_string(&data).unwrap(),
    );

    let response = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::to_string(&response).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_u32() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some(u32::MAX);

    let json = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&errors).unwrap(),
                       serde_json::to_string(&results).unwrap(),
                       serde_json::to_string(&data).unwrap(),
    );

    let expected = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_str() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some("test");

    let json = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&errors).unwrap(),
                       serde_json::to_string(&results).unwrap(),
                       serde_json::to_string(&data).unwrap(),
    );

    let expected = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_ok_map() {
    let status = 200;
    let name = None;
    let tag = None;
    let errors = None;
    let results = None;
    let data = Some(HashMap::from([("test", "me")]));

    let json = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&errors).unwrap(),
                       serde_json::to_string(&results).unwrap(),
                       serde_json::to_string(&data).unwrap(),
    );

    let expected = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_error() {
    let status = 400;
    let name = None;
    let tag = None;
    let errors = Some(
        vec![
            ValorantApiError {
                code: 104,
                message: String::from("Invalid region"),
                details: String::from("details"),
            },
        ]
    );
    let results = None;
    let data = None::<()>;

    let json = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&errors).unwrap(),
                       serde_json::to_string(&results).unwrap(),
                       serde_json::to_string(&data).unwrap(),
    );

    let expected = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}

#[test]
fn deserialize_result_with_player() {
    let status = 200;
    let name = Some(String::from("t00manysecrets"));
    let tag = Some(String::from("EUW"));
    let errors = None;
    let results = Some(
        ValorantApiResults {
            total: 420,
            returned: 10,
            before: 20,
            after: 390,
        }
    );
    let data = Some("data");

    let json = format!("{{\
    \"status\":{status},\
    \"name\":{0},\
    \"tag\":{1},\
    \"errors\":{2},\
    \"results\":{3},\
    \"data\":{4}\
    }}",
                       serde_json::to_string(&name).unwrap(),
                       serde_json::to_string(&tag).unwrap(),
                       serde_json::to_string(&errors).unwrap(),
                       serde_json::to_string(&results).unwrap(),
                       serde_json::to_string(&data).unwrap(),
    );

    let expected = ValorantApiResponse {
        status,
        name,
        tag,
        errors,
        results,
        data,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
