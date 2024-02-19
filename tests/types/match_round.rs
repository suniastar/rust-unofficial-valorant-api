use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{MapCoordinates, MapLocation, MatchKill, MatchKillAssistant, MatchKillWeaponAssets, MatchRound, MatchRoundDefuseEvent, MatchRoundEventBy, MatchRoundPlantEvent, MatchRoundPlayerStats, MatchRoundPlayerStatsAbilityCasts, MatchRoundPlayerStatsDamageEvent, MatchRoundPlayerStatsEconomy, MatchRoundPlayerStatsEconomyArmor, MatchRoundPlayerStatsEconomyArmorAssets, MatchRoundPlayerStatsEconomyWeapon, MatchRoundPlayerStatsEconomyWeaponAssets, PlantSite, PlayerTeam, RoundEndType};

#[test]
fn serialize() {
    let winning_team = PlayerTeam::Blue;
    let end_type = RoundEndType::BombDefused;
    let bomb_planted = true;
    let bomb_defused = true;
    let plant_event = MatchRoundPlantEvent {
        plant_coordinates: MapCoordinates {
            x: -1325,
            y: -1326,
        },
        planted_by: MatchRoundEventBy {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Red,
        },
        plant_site: PlantSite::A,
        plant_time_in_round: 26345,
        player_locations: vec![
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
        ],
    };
    let defuse_event = MatchRoundDefuseEvent {
        defuse_coordinates: MapCoordinates {
            x: 1325,
            y: 1326,
        },
        defused_by: MatchRoundEventBy {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Blue,
        },
        defuse_time_in_round: 26345,
        player_locations: vec![
            MapLocation {
                id: Uuid::new_v4(),
                display_name: String::from("Henrik3#EUW"),
                team: PlayerTeam::Red,
                coordinates: MapCoordinates {
                    x: -5177,
                    y: 8908,
                },
                view_radians: 0.5277854f64,
            },
        ],
    };
    let player_stats: Vec<MatchRoundPlayerStats> = vec![
        MatchRoundPlayerStats {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Blue,
            ability_casts: MatchRoundPlayerStatsAbilityCasts {
                c: 4,
                q: 3,
                e: 2,
                x: 1,
            },
            damage_events: vec![
                MatchRoundPlayerStatsDamageEvent {
                    receiver_id: Uuid::new_v4(),
                    receiver_display_name: String::from("Henrik3#EUW"),
                    receiver_team: PlayerTeam::Red,
                    damage: 156,
                    head_shots: 1,
                    body_shots: 3,
                    leg_shots: 0,
                },
            ],
            damage: 282,
            head_shots: 2,
            body_shots: 3,
            leg_shots: 1,
            kill_events: vec![
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
                        display_icon_url: Url::parse("https://google.com/search?q=display1").unwrap(),
                        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed1").unwrap(),
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
            ],
            kills: 1,
            score: 430,
            economy: MatchRoundPlayerStatsEconomy {
                load_out_value: 3900,
                weapon: MatchRoundPlayerStatsEconomyWeapon {
                    id: Uuid::new_v4(),
                    name: String::from("Spectre"),
                    assets: MatchRoundPlayerStatsEconomyWeaponAssets {
                        display_icon_url: Url::parse("https://google.com/search?q=display2").unwrap(),
                        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed2").unwrap(),
                    },
                },
                armor: MatchRoundPlayerStatsEconomyArmor {
                    id: Uuid::new_v4(),
                    name: String::from("Heavy Shields"),
                    assets: MatchRoundPlayerStatsEconomyArmorAssets {
                        display_icon_url: Url::parse("https://google.com/search?q=display_armor").unwrap(),
                    },
                },
                remaining: 5300,
                spent: 1550,
            },
            was_afk: false,
            was_penalized: false,
            stayed_in_spawn: true,
        }
    ];

    let expected = format!("{{\
    \"winning_team\":\"{winning_team}\",\
    \"end_type\":\"{end_type}\",\
    \"bomb_planted\":{bomb_planted},\
    \"bomb_defused\":{bomb_defused},\
    \"plant_events\":{0},\
    \"defuse_events\":{1},\
    \"player_stats\":{2}\
    }}",
                           serde_json::to_string(&plant_event).unwrap(),
                           serde_json::to_string(&defuse_event).unwrap(),
                           serde_json::to_string(&player_stats).unwrap(),
    );

    let round = MatchRound {
        winning_team,
        end_type,
        bomb_planted,
        bomb_defused,
        plant_event,
        defuse_event,
        player_stats,
    };
    let actual = serde_json::to_string(&round).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let winning_team = PlayerTeam::Blue;
    let end_type = RoundEndType::BombDefused;
    let bomb_planted = true;
    let bomb_defused = true;
    let plant_event = MatchRoundPlantEvent {
        plant_coordinates: MapCoordinates {
            x: -1325,
            y: -1326,
        },
        planted_by: MatchRoundEventBy {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Red,
        },
        plant_site: PlantSite::A,
        plant_time_in_round: 26345,
        player_locations: vec![
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
        ],
    };
    let defuse_event = MatchRoundDefuseEvent {
        defuse_coordinates: MapCoordinates {
            x: 1325,
            y: 1326,
        },
        defused_by: MatchRoundEventBy {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Blue,
        },
        defuse_time_in_round: 26345,
        player_locations: vec![
            MapLocation {
                id: Uuid::new_v4(),
                display_name: String::from("Henrik3#EUW"),
                team: PlayerTeam::Red,
                coordinates: MapCoordinates {
                    x: -5177,
                    y: 8908,
                },
                view_radians: 0.5277854f64,
            },
        ],
    };
    let player_stats: Vec<MatchRoundPlayerStats> = vec![
        MatchRoundPlayerStats {
            id: Uuid::new_v4(),
            display_name: String::from("t00manysecrets#EUW"),
            team: PlayerTeam::Blue,
            ability_casts: MatchRoundPlayerStatsAbilityCasts {
                c: 4,
                q: 3,
                e: 2,
                x: 1,
            },
            damage_events: vec![
                MatchRoundPlayerStatsDamageEvent {
                    receiver_id: Uuid::new_v4(),
                    receiver_display_name: String::from("Henrik3#EUW"),
                    receiver_team: PlayerTeam::Red,
                    damage: 156,
                    head_shots: 1,
                    body_shots: 3,
                    leg_shots: 0,
                },
            ],
            damage: 282,
            head_shots: 2,
            body_shots: 3,
            leg_shots: 1,
            kill_events: vec![
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
                        display_icon_url: Url::parse("https://google.com/search?q=display1").unwrap(),
                        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed1").unwrap(),
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
            ],
            kills: 1,
            score: 430,
            economy: MatchRoundPlayerStatsEconomy {
                load_out_value: 3900,
                weapon: MatchRoundPlayerStatsEconomyWeapon {
                    id: Uuid::new_v4(),
                    name: String::from("Spectre"),
                    assets: MatchRoundPlayerStatsEconomyWeaponAssets {
                        display_icon_url: Url::parse("https://google.com/search?q=display2").unwrap(),
                        kill_feed_icon_url: Url::parse("https://google.com/search?q=killfeed2").unwrap(),
                    },
                },
                armor: MatchRoundPlayerStatsEconomyArmor {
                    id: Uuid::new_v4(),
                    name: String::from("Heavy Shields"),
                    assets: MatchRoundPlayerStatsEconomyArmorAssets {
                        display_icon_url: Url::parse("https://google.com/search?q=display_armor").unwrap(),
                    },
                },
                remaining: 5300,
                spent: 1550,
            },
            was_afk: false,
            was_penalized: false,
            stayed_in_spawn: true,
        }
    ];

    let json = format!("{{\
    \"winning_team\":\"{winning_team}\",\
    \"end_type\":\"{end_type}\",\
    \"bomb_planted\":{bomb_planted},\
    \"bomb_defused\":{bomb_defused},\
    \"plant_events\":{0},\
    \"defuse_events\":{1},\
    \"player_stats\":{2}\
    }}",
                       serde_json::to_string(&plant_event).unwrap(),
                       serde_json::to_string(&defuse_event).unwrap(),
                       serde_json::to_string(&player_stats).unwrap(),
    );

    let expected = MatchRound {
        winning_team,
        end_type,
        bomb_planted,
        bomb_defused,
        plant_event,
        defuse_event,
        player_stats,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
