use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, MatchKill, MatchKillAssistant, MatchKillWeaponAssets, MatchRoundPlayerStats, MatchRoundPlayerStatsDamageEvent, MatchRoundPlayerStatsEconomy, MatchRoundPlayerStatsEconomyArmor, MatchRoundPlayerStatsEconomyArmorAssets, MatchRoundPlayerStatsEconomyWeapon, MatchRoundPlayerStatsEconomyWeaponAssets, PlayerTeam};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;
    // let ability_casts = MatchRoundPlayerStatsAbilityCasts {
    //     c: 4,
    //     q: 3,
    //     e: 2,
    //     x: 1,
    // };
    let damage_events: Vec<MatchRoundPlayerStatsDamageEvent> = vec![
        MatchRoundPlayerStatsDamageEvent {
            receiver_id: Uuid::new_v4(),
            receiver_display_name: String::from("Henrik3#EUW"),
            receiver_team: PlayerTeam::Red,
            damage: 156,
            head_shots: 1,
            body_shots: 3,
            leg_shots: 0,
        },
    ];
    let damage = 282;
    let head_shots = 2;
    let body_shots = 3;
    let leg_shots = 1;
    let kill_events: Vec<MatchKill> = vec![
        MatchKill {
            kill_time_in_round: 43163,
            kill_time_in_match: 890501,
            killer_id: Uuid::new_v4(),
            killer_display_name: String::from("t00manysecrets#EUW"),
            killer_team: PlayerTeam::Blue,
            victim_id: Uuid::new_v4(),
            victim_display_name: String::from("Sanguis#EUW"),
            victim_team: PlayerTeam::Red,
            victim_coordinates: MapCoordinates {
                x: 7266,
                y: -5096,
            },
            weapon_id: Uuid::new_v4(),
            weapon_name: String::from("Vandal"),
            weapon_assets: MatchKillWeaponAssets {
                display_icon_url: Url::parse("https://google.com/search?q=display").unwrap(),
                kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
            },
            secondary_fire_mode: true,
            player_locations_on_kill: vec![
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
            ],
            assistants: vec![
                MatchKillAssistant {
                    id: Uuid::new_v4(),
                    display_name: String::from("Henrik3#EUW"),
                    team: PlayerTeam::Blue,
                },
            ],
        },
    ];
    let kills = 1;
    let score = 430;
    let economy = MatchRoundPlayerStatsEconomy {
        load_out_value: 3900,
        weapon: MatchRoundPlayerStatsEconomyWeapon {
            id: Uuid::new_v4(),
            name: String::from("Spectre"),
            assets: MatchRoundPlayerStatsEconomyWeaponAssets {
                display_icon_url: Url::parse("https://google.com/search?q=weapon_icon").unwrap(),
                kill_feed_icon_url: Url::parse("https://google.com/search?q=weapon_killfeed").unwrap(),
            },
        },
        armor: MatchRoundPlayerStatsEconomyArmor {
            id: Uuid::new_v4(),
            name: String::from("Heavy Shields"),
            assets: MatchRoundPlayerStatsEconomyArmorAssets {
                display_icon_url: Url::parse("https://google.com/search?q=heavy_shield").unwrap(),
            },
        },
        remaining: 5300,
        spent: 1550,
    };
    let was_afk = false;
    let was_penalized = false;
    let stayed_in_spawn = true;

    // removed \"ability_casts\":{0},\ due to api limitations on riots end
    let expected = format!("{{\
    \"player_puuid\":\"{id}\",\
    \"player_display_name\":\"{display_name}\",\
    \"player_team\":\"{team}\",\
    \"damage_events\":{0},\
    \"damage\":{damage},\
    \"headshots\":{head_shots},\
    \"bodyshots\":{body_shots},\
    \"legshots\":{leg_shots},\
    \"kill_events\":{1},\
    \"kills\":{kills},\
    \"score\":{score},\
    \"economy\":{2},\
    \"was_afk\":{was_afk},\
    \"was_penalized\":{was_penalized},\
    \"stayed_in_spawn\":{stayed_in_spawn}\
    }}",
                           serde_json::to_string(&damage_events).unwrap(),
                           serde_json::to_string(&kill_events).unwrap(),
                           serde_json::to_string(&economy).unwrap());

    let player_stats = MatchRoundPlayerStats {
        id,
        display_name,
        team,
        damage_events,
        damage,
        head_shots,
        body_shots,
        leg_shots,
        kill_events,
        kills,
        score,
        economy,
        was_afk,
        was_penalized,
        stayed_in_spawn,
    };
    let actual = serde_json::to_string(&player_stats).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let display_name = String::from("t00manysecrets#EUW");
    let team = PlayerTeam::Blue;
    // let ability_casts = MatchRoundPlayerStatsAbilityCasts {
    //     c: 4,
    //     q: 3,
    //     e: 2,
    //     x: 1,
    // };
    let damage_events: Vec<MatchRoundPlayerStatsDamageEvent> = vec![
        MatchRoundPlayerStatsDamageEvent {
            receiver_id: Uuid::new_v4(),
            receiver_display_name: String::from("Henrik3#EUW"),
            receiver_team: PlayerTeam::Red,
            damage: 156,
            head_shots: 1,
            body_shots: 3,
            leg_shots: 0,
        },
    ];
    let damage = 282;
    let head_shots = 2;
    let body_shots = 3;
    let leg_shots = 1;
    let kill_events: Vec<MatchKill> = vec![
        MatchKill {
            kill_time_in_round: 43163,
            kill_time_in_match: 890501,
            killer_id: Uuid::new_v4(),
            killer_display_name: String::from("t00manysecrets#EUW"),
            killer_team: PlayerTeam::Blue,
            victim_id: Uuid::new_v4(),
            victim_display_name: String::from("Sanguis#EUW"),
            victim_team: PlayerTeam::Red,
            victim_coordinates: MapCoordinates {
                x: 7266,
                y: -5096,
            },
            weapon_id: Uuid::new_v4(),
            weapon_name: String::from("Vandal"),
            weapon_assets: MatchKillWeaponAssets {
                display_icon_url: Url::parse("https://google.com/search?q=display").unwrap(),
                kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
            },
            secondary_fire_mode: true,
            player_locations_on_kill: vec![
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
            ],
            assistants: vec![
                MatchKillAssistant {
                    id: Uuid::new_v4(),
                    display_name: String::from("Henrik3#EUW"),
                    team: PlayerTeam::Blue,
                },
            ],
        },
    ];
    let kills = 1;
    let score = 430;
    let economy = MatchRoundPlayerStatsEconomy {
        load_out_value: 3900,
        weapon: MatchRoundPlayerStatsEconomyWeapon {
            id: Uuid::new_v4(),
            name: String::from("Spectre"),
            assets: MatchRoundPlayerStatsEconomyWeaponAssets {
                display_icon_url: Url::parse("https://google.com/search?q=weapon_icon").unwrap(),
                kill_feed_icon_url: Url::parse("https://google.com/search?q=weapon_killfeed").unwrap(),
            },
        },
        armor: MatchRoundPlayerStatsEconomyArmor {
            id: Uuid::new_v4(),
            name: String::from("Heavy Shields"),
            assets: MatchRoundPlayerStatsEconomyArmorAssets {
                display_icon_url: Url::parse("https://google.com/search?q=heavy_shield").unwrap(),
            },
        },
        remaining: 5300,
        spent: 1550,
    };
    let was_afk = false;
    let was_penalized = false;
    let stayed_in_spawn = true;

    // removed \"ability_casts\":{0},\ due to api limitations on riots end
    let json = format!("{{\
    \"player_puuid\":\"{id}\",\
    \"player_display_name\":\"{display_name}\",\
    \"player_team\":\"{team}\",\
    \"damage_events\":{0},\
    \"damage\":{damage},\
    \"headshots\":{head_shots},\
    \"bodyshots\":{body_shots},\
    \"legshots\":{leg_shots},\
    \"kill_events\":{1},\
    \"kills\":{kills},\
    \"score\":{score},\
    \"economy\":{2},\
    \"was_afk\":{was_afk},\
    \"was_penalized\":{was_penalized},\
    \"stayed_in_spawn\":{stayed_in_spawn}\
    }}",
                       serde_json::to_string(&damage_events).unwrap(),
                       serde_json::to_string(&kill_events).unwrap(),
                       serde_json::to_string(&economy).unwrap());

    let expected = MatchRoundPlayerStats {
        id,
        display_name,
        team,
        damage_events,
        damage,
        head_shots,
        body_shots,
        leg_shots,
        kill_events,
        kills,
        score,
        economy,
        was_afk,
        was_penalized,
        stayed_in_spawn,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
