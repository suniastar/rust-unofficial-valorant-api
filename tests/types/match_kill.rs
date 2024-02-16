use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, MatchKill, MatchKillAssistant, MatchKillWeaponAssets, PlayerTeam};

#[test]
fn serialize() {
    let kill_time_in_round = 43163;
    let kill_time_in_match = 890501;
    let killer_id = Uuid::new_v4();
    let killer_display_name = String::from("t00manysecrets#EUW");
    let killer_team = PlayerTeam::Blue;
    let victim_id = Uuid::new_v4();
    let victim_display_name = String::from("Sanguis#EUW");
    let victim_team = PlayerTeam::Red;
    let victim_coordinates = MapCoordinates {
        x: 7266,
        y: -5096,
    };
    let weapon_id = Uuid::new_v4();
    let weapon_name = String::from("Vandal");
    let weapon_assets = MatchKillWeaponAssets {
        display_icon_url: Url::parse("https://google.com/search?q=display").unwrap(),
        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
    };
    let secondary_fire_mode = true;
    let player_locations_on_kill: Vec<MapLocation> = vec![
        MapLocation {
            id: Uuid::new_v4(),
            display_name: String::from("t00manyesecrets#EUW"),
            team: PlayerTeam::Blue,
            coordinates: MapCoordinates {
                x: 5177,
                y: -8908,
            },
            view_radians: 0.5277854f64,
        },
    ];
    let assistants = vec![
        MatchKillAssistant {
            id: Uuid::new_v4(),
            display_name: String::from("Henrik3#EUW"),
            team: PlayerTeam::Blue,
        },
    ];

    let expected = format!("{{\
    \"kill_time_in_round\":{kill_time_in_round},\
    \"kill_time_in_match\":{kill_time_in_match},\
    \"killer_puuid\":\"{killer_id}\",\
    \"killer_display_name\":\"{killer_display_name}\",\
    \"killer_team\":\"{killer_team}\",\
    \"victim_puuid\":\"{victim_id}\",\
    \"victim_display_name\":\"{victim_display_name}\",\
    \"victim_team\":\"{victim_team}\",\
    \"victim_death_location\":{0},\
    \"damage_weapon_id\":\"{weapon_id}\",\
    \"damage_weapon_name\":\"{weapon_name}\",\
    \"damage_weapon_assets\":{1},\
    \"secondary_fire_mode\":{secondary_fire_mode},\
    \"player_locations_on_kill\":{2},\
    \"assistants\":{3}\
    }}",
                           serde_json::to_string(&victim_coordinates).unwrap(),
                           serde_json::to_string(&weapon_assets).unwrap(),
                           serde_json::to_string(&player_locations_on_kill).unwrap(),
                           serde_json::to_string(&assistants).unwrap(),
    );

    let kill = MatchKill {
        kill_time_in_round,
        kill_time_in_match,
        killer_id,
        killer_display_name,
        killer_team,
        victim_id,
        victim_display_name,
        victim_team,
        victim_coordinates,
        weapon_id,
        weapon_name,
        weapon_assets,
        secondary_fire_mode,
        player_locations_on_kill,
        assistants,
    };
    let actual = serde_json::to_string(&kill).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let kill_time_in_round = 43163;
    let kill_time_in_match = 890501;
    let killer_id = Uuid::new_v4();
    let killer_display_name = String::from("t00manysecrets#EUW");
    let killer_team = PlayerTeam::Blue;
    let victim_id = Uuid::new_v4();
    let victim_display_name = String::from("Sanguis#EUW");
    let victim_team = PlayerTeam::Red;
    let victim_coordinates = MapCoordinates {
        x: 7266,
        y: -5096,
    };
    let weapon_id = Uuid::new_v4();
    let weapon_name = String::from("Vandal");
    let weapon_assets = MatchKillWeaponAssets {
        display_icon_url: Url::parse("https://google.com/search?q=display").unwrap(),
        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
    };
    let secondary_fire_mode = true;
    let player_locations_on_kill: Vec<MapLocation> = vec![
        MapLocation {
            id: Uuid::new_v4(),
            display_name: String::from("t00manyesecrets#EUW"),
            team: PlayerTeam::Blue,
            coordinates: MapCoordinates {
                x: 5177,
                y: -8908,
            },
            view_radians: 0.5277854f64,
        },
    ];
    let assistants = vec![
        MatchKillAssistant {
            id: Uuid::new_v4(),
            display_name: String::from("Henrik3#EUW"),
            team: PlayerTeam::Blue,
        },
    ];

    let json = format!("{{\
    \"kill_time_in_round\":{kill_time_in_round},\
    \"kill_time_in_match\":{kill_time_in_match},\
    \"killer_puuid\":\"{killer_id}\",\
    \"killer_display_name\":\"{killer_display_name}\",\
    \"killer_team\":\"{killer_team}\",\
    \"victim_puuid\":\"{victim_id}\",\
    \"victim_display_name\":\"{victim_display_name}\",\
    \"victim_team\":\"{victim_team}\",\
    \"victim_death_location\":{0},\
    \"damage_weapon_id\":\"{weapon_id}\",\
    \"damage_weapon_name\":\"{weapon_name}\",\
    \"damage_weapon_assets\":{1},\
    \"secondary_fire_mode\":{secondary_fire_mode},\
    \"player_locations_on_kill\":{2},\
    \"assistants\":{3}\
    }}",
                       serde_json::to_string(&victim_coordinates).unwrap(),
                       serde_json::to_string(&weapon_assets).unwrap(),
                       serde_json::to_string(&player_locations_on_kill).unwrap(),
                       serde_json::to_string(&assistants).unwrap(),
    );

    let expected = MatchKill {
        kill_time_in_round,
        kill_time_in_match,
        killer_id,
        killer_display_name,
        killer_team,
        victim_id,
        victim_display_name,
        victim_team,
        victim_coordinates,
        weapon_id,
        weapon_name,
        weapon_assets,
        secondary_fire_mode,
        player_locations_on_kill,
        assistants,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
