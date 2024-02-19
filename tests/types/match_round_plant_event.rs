use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, MatchRoundEventBy, MatchRoundPlantEvent, PlantSite, PlayerTeam};

#[test]
fn serialize() {
    let plant_coordinates = MapCoordinates {
        x: -1325,
        y: -1326,
    };
    let planted_by = MatchRoundEventBy {
        id: Uuid::new_v4(),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Red,
    };
    let plant_site = PlantSite::A;
    let plant_time_in_round = 26345;
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
    \"plant_location\":{0},\
    \"planted_by\":{1},\
    \"plant_site\":\"{plant_site}\",\
    \"plant_time_in_round\":{plant_time_in_round},\
    \"player_locations_on_plant\":{2}\
    }}",
                           serde_json::to_string(&plant_coordinates).unwrap(),
                           serde_json::to_string(&planted_by).unwrap(),
                           serde_json::to_string(&player_locations).unwrap(),
    );

    let plant_event = MatchRoundPlantEvent {
        plant_coordinates,
        planted_by,
        plant_site,
        plant_time_in_round,
        player_locations,
    };
    let actual = serde_json::to_string(&plant_event).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let plant_coordinates = MapCoordinates {
        x: -1325,
        y: -1326,
    };
    let planted_by = MatchRoundEventBy {
        id: Uuid::new_v4(),
        display_name: String::from("t00manysecrets#EUW"),
        team: PlayerTeam::Red,
    };
    let plant_site = PlantSite::A;
    let plant_time_in_round = 26345;
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
    \"plant_location\":{0},\
    \"planted_by\":{1},\
    \"plant_site\":\"{plant_site}\",\
    \"plant_time_in_round\":{plant_time_in_round},\
    \"player_locations_on_plant\":{2}\
    }}",
                       serde_json::to_string(&plant_coordinates).unwrap(),
                       serde_json::to_string(&planted_by).unwrap(),
                       serde_json::to_string(&player_locations).unwrap(),
    );

    let expected = MatchRoundPlantEvent {
        plant_coordinates,
        planted_by,
        plant_site,
        plant_time_in_round,
        player_locations,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
