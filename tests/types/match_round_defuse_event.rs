use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, MatchRoundDefuseEvent, MatchRoundEventBy, PlayerTeam};

#[test]
fn serialize() {
    let defuse_coordinates = MapCoordinates {
        x: -1325,
        y: -1326,
    };
    let defused_by = MatchRoundEventBy {
        id: Uuid::new_v4(),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
    };
    let defuse_time_in_round = 26345;
    let player_locations: Vec<MapLocation> = vec![
        MapLocation {
            id: Uuid::new_v4(),
            display_name: String::from("Henrik3#EUW"),
            team: PlayerTeam::Red,
            coordinates: MapCoordinates {
                x: 5177,
                y: -8908,
            },
            view_radians: 0.5277854f64,
        },
    ];

    let expected = format!("{{\
    \"defuse_location\":{0},\
    \"defused_by\":{1},\
    \"defuse_time_in_round\":{defuse_time_in_round},\
    \"player_locations_on_defuse\":{2}\
    }}",
                           serde_json::to_string(&defuse_coordinates).unwrap(),
                           serde_json::to_string(&defused_by).unwrap(),
                           serde_json::to_string(&player_locations).unwrap(),
    );

    let defuse_event = MatchRoundDefuseEvent {
        defuse_coordinates,
        defused_by,
        defuse_time_in_round,
        player_locations,
    };
    let actual = serde_json::to_string(&defuse_event).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let defuse_coordinates = MapCoordinates {
        x: -1325,
        y: -1326,
    };
    let defused_by = MatchRoundEventBy {
        id: Uuid::new_v4(),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Blue,
    };
    let defuse_time_in_round = 26345;
    let player_locations: Vec<MapLocation> = vec![
        MapLocation {
            id: Uuid::new_v4(),
            display_name: String::from("Henrik3#EUW"),
            team: PlayerTeam::Red,
            coordinates: MapCoordinates {
                x: 5177,
                y: -8908,
            },
            view_radians: 0.5277854f64,
        },
    ];

    let json = format!("{{\
    \"defuse_location\":{0},\
    \"defused_by\":{1},\
    \"defuse_time_in_round\":{defuse_time_in_round},\
    \"player_locations_on_defuse\":{2}\
    }}",
                       serde_json::to_string(&defuse_coordinates).unwrap(),
                       serde_json::to_string(&defused_by).unwrap(),
                       serde_json::to_string(&player_locations).unwrap(),
    );

    let expected = MatchRoundDefuseEvent {
        defuse_coordinates,
        defused_by,
        defuse_time_in_round,
        player_locations,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
