use std::hash::{DefaultHasher, Hash, Hasher};

use uuid::{Uuid, uuid};

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, PlayerTeam};

#[test]
fn equals() {
    let location1 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: 69.420_f64,
    };
    let location2 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: 69.420_f64,
    };

    assert_eq!(location1, location2);
}

#[test]
fn reflexivity() {
    let location1 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: f64::NAN,
    };
    let location2 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: f64::NAN,
    };

    assert_eq!(location1, location2);
    assert_eq!(location2, location1);

    let mut hasher1 = DefaultHasher::new();
    location1.hash(&mut hasher1);
    let mut hasher2 = DefaultHasher::new();
    location2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn hash() {
    let location1 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: 69.420_f64,
    };
    let location2 = MapLocation {
        id: uuid!("6784568e-a9ca-43ae-acc0-b7bb6ad456eb"),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
        coordinates: MapCoordinates {
            x: i32::MAX,
            y: i32::MIN,
        },
        view_radians: 69.420_f64,
    };

    let mut hasher1 = DefaultHasher::new();
    location1.hash(&mut hasher1);
    let mut hasher2 = DefaultHasher::new();
    location2.hash(&mut hasher2);

    assert_eq!(hasher1.finish(), hasher2.finish());
}

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;
    let coordinates = MapCoordinates {
        x: i32::MAX,
        y: i32::MIN,
    };
    let view_radians = 0.5277854f64;

    let expected = format!("{{\
    \"player_puuid\":\"{id}\",\
    \"player_display_name\":\"{display_name}\",\
    \"player_team\":\"{team}\",\
    \"location\":{0},\
    \"view_radians\":{view_radians}\
    }}",
                           serde_json::to_string(&coordinates).unwrap(),
    );

    let location = MapLocation {
        id,
        display_name,
        team,
        coordinates,
        view_radians,
    };
    let actual = serde_json::to_string(&location).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;
    let coordinates = MapCoordinates {
        x: i32::MAX,
        y: i32::MIN,
    };
    let view_radians = 0.5277854f64;

    let json = format!("{{\
    \"player_puuid\":\"{id}\",\
    \"player_display_name\":\"{display_name}\",\
    \"player_team\":\"{team}\",\
    \"location\":{0},\
    \"view_radians\":{view_radians}\
    }}",
                       serde_json::to_string(&coordinates).unwrap(),
    );

    let expected = MapLocation {
        id,
        display_name,
        team,
        coordinates,
        view_radians,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
