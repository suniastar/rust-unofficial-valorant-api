use chrono::Duration;
use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Character, MatchPlayers, Platform, PlatformOs, PlatformType, Player, PlayerAbilityCasts, PlayerAssets, PlayerAssetsAgent, PlayerAssetsCard, PlayerBehavior, PlayerBehaviorFriendlyFire, PlayerEconomy, PlayerEconomyCredits, PlayerStats, PlayerTeam, Tier};

#[test]
fn serialize() {
    let red_player = Player {
        id: Uuid::new_v4(),
        name: String::from("t00manysecrets"),
        tag: String::from("EUW"),
        team: PlayerTeam::Red,
        level: 166,
        character: Character::Skye,
        current_tier: Tier::Unrated,
        card_id: Uuid::new_v4(),
        title_id: Uuid::new_v4(),
        party_id: Uuid::new_v4(),
        playtime: Duration::seconds(4320),
        assets: PlayerAssets {
            card: PlayerAssetsCard {
                small_url: Url::parse("https://google.com/search?q=small_card").unwrap(),
                large_url: Url::parse("https://google.com/search?q=large_card").unwrap(),
                wide_url: Url::parse("https://google.com/search?q=wide_card").unwrap(),
            },
            agent: PlayerAssetsAgent {
                small_url: Url::parse("https://google.com/search?q=small_agent").unwrap(),
                bust_url: Url::parse("https://google.com/search?q=bust_agent").unwrap(),
                full_url: Url::parse("https://google.com/search?q=full_agent").unwrap(),
                kill_feed_url: Url::parse("https://google.com/search?q=killfeed_agent").unwrap(),
            },
        },
        behavior: PlayerBehavior {
            afk_rounds: 0,
            friendly_fire: PlayerBehaviorFriendlyFire {
                incoming: 0,
                outgoing: 0,
            },
            rounds_in_spawn: 0,
        },
        platform: Platform {
            type_: PlatformType::PC,
            os: PlatformOs {
                name: String::from("Windows"),
                version: String::from("10.0.19045.1.256.64bit"),
            },
        },
        ability_casts: PlayerAbilityCasts {
            c: 5,
            q: 7,
            e: 45,
            x: 3,
        },
        stats: PlayerStats {
            score: 4217,
            kills: 15,
            deaths: 14,
            assists: 10,
            body_shots: 19,
            head_shots: 12,
            leg_shots: 0,
        },
        economy: PlayerEconomy {
            spent: PlayerEconomyCredits {
                overall: 50150,
                average: 2508,
            },
            load_out_value: PlayerEconomyCredits {
                overall: 77350,
                average: 3868,
            },
        },
        damage_made: 2721,
        damage_received: 2832,
    };
    let blue_player = Player {
        id: Uuid::new_v4(),
        name: String::from("Sanguis"),
        tag: String::from("8555"),
        team: PlayerTeam::Blue,
        level: 122,
        character: Character::Raze,
        current_tier: Tier::Unrated,
        card_id: Uuid::new_v4(),
        title_id: Uuid::new_v4(),
        party_id: Uuid::new_v4(),
        playtime: Duration::seconds(4380),
        assets: PlayerAssets {
            card: PlayerAssetsCard {
                small_url: Url::parse("https://google.com/search?q=small_card").unwrap(),
                large_url: Url::parse("https://google.com/search?q=large_card").unwrap(),
                wide_url: Url::parse("https://google.com/search?q=wide_card").unwrap(),
            },
            agent: PlayerAssetsAgent {
                small_url: Url::parse("https://google.com/search?q=small_agent").unwrap(),
                bust_url: Url::parse("https://google.com/search?q=bust_agent").unwrap(),
                full_url: Url::parse("https://google.com/search?q=full_agent").unwrap(),
                kill_feed_url: Url::parse("https://google.com/search?q=killfeed_agent").unwrap(),
            },
        },
        behavior: PlayerBehavior {
            afk_rounds: 0,
            friendly_fire: PlayerBehaviorFriendlyFire {
                incoming: 0,
                outgoing: 0,
            },
            rounds_in_spawn: 0,
        },
        platform: Platform {
            type_: PlatformType::PC,
            os: PlatformOs {
                name: String::from("Windows"),
                version: String::from("10.0.19045.1.256.64bit"),
            },
        },
        ability_casts: PlayerAbilityCasts {
            c: 7,
            q: 9,
            e: 12,
            x: 3,
        },
        stats: PlayerStats {
            score: 6500,
            kills: 20,
            deaths: 18,
            assists: 7,
            body_shots: 82,
            head_shots: 9,
            leg_shots: 8,
        },
        economy: PlayerEconomy {
            spent: PlayerEconomyCredits {
                overall: 57400,
                average: 2870,
            },
            load_out_value: PlayerEconomyCredits {
                overall: 68300,
                average: 3415,
            },
        },
        damage_made: 4124,
        damage_received: 3330,
    };
    let all = vec![
        red_player.clone(),
        blue_player.clone(),
    ];
    let red = vec![red_player.clone()];
    let blue = vec![blue_player.clone()];

    let expected = format!("{{\
    \"all_players\":{0},\
    \"red\":{1},\
    \"blue\":{2}\
    }}",
                           serde_json::to_string(&all).unwrap(),
                           serde_json::to_string(&red).unwrap(),
                           serde_json::to_string(&blue).unwrap(),
    );

    let players = MatchPlayers {
        all,
        red,
        blue,
    };
    let actual = serde_json::to_string(&players).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let red_player = Player {
        id: Uuid::new_v4(),
        name: String::from("t00manysecrets"),
        tag: String::from("EUW"),
        team: PlayerTeam::Red,
        level: 166,
        character: Character::Skye,
        current_tier: Tier::Unrated,
        card_id: Uuid::new_v4(),
        title_id: Uuid::new_v4(),
        party_id: Uuid::new_v4(),
        playtime: Duration::seconds(4320),
        assets: PlayerAssets {
            card: PlayerAssetsCard {
                small_url: Url::parse("https://google.com/search?q=small_card").unwrap(),
                large_url: Url::parse("https://google.com/search?q=large_card").unwrap(),
                wide_url: Url::parse("https://google.com/search?q=wide_card").unwrap(),
            },
            agent: PlayerAssetsAgent {
                small_url: Url::parse("https://google.com/search?q=small_agent").unwrap(),
                bust_url: Url::parse("https://google.com/search?q=bust_agent").unwrap(),
                full_url: Url::parse("https://google.com/search?q=full_agent").unwrap(),
                kill_feed_url: Url::parse("https://google.com/search?q=killfeed_agent").unwrap(),
            },
        },
        behavior: PlayerBehavior {
            afk_rounds: 0,
            friendly_fire: PlayerBehaviorFriendlyFire {
                incoming: 0,
                outgoing: 0,
            },
            rounds_in_spawn: 0,
        },
        platform: Platform {
            type_: PlatformType::PC,
            os: PlatformOs {
                name: String::from("Windows"),
                version: String::from("10.0.19045.1.256.64bit"),
            },
        },
        ability_casts: PlayerAbilityCasts {
            c: 5,
            q: 7,
            e: 45,
            x: 3,
        },
        stats: PlayerStats {
            score: 4217,
            kills: 15,
            deaths: 14,
            assists: 10,
            body_shots: 19,
            head_shots: 12,
            leg_shots: 0,
        },
        economy: PlayerEconomy {
            spent: PlayerEconomyCredits {
                overall: 50150,
                average: 2508,
            },
            load_out_value: PlayerEconomyCredits {
                overall: 77350,
                average: 3868,
            },
        },
        damage_made: 2721,
        damage_received: 2832,
    };
    let blue_player = Player {
        id: Uuid::new_v4(),
        name: String::from("Sanguis"),
        tag: String::from("8555"),
        team: PlayerTeam::Blue,
        level: 122,
        character: Character::Raze,
        current_tier: Tier::Unrated,
        card_id: Uuid::new_v4(),
        title_id: Uuid::new_v4(),
        party_id: Uuid::new_v4(),
        playtime: Duration::seconds(4380),
        assets: PlayerAssets {
            card: PlayerAssetsCard {
                small_url: Url::parse("https://google.com/search?q=small_card").unwrap(),
                large_url: Url::parse("https://google.com/search?q=large_card").unwrap(),
                wide_url: Url::parse("https://google.com/search?q=wide_card").unwrap(),
            },
            agent: PlayerAssetsAgent {
                small_url: Url::parse("https://google.com/search?q=small_agent").unwrap(),
                bust_url: Url::parse("https://google.com/search?q=bust_agent").unwrap(),
                full_url: Url::parse("https://google.com/search?q=full_agent").unwrap(),
                kill_feed_url: Url::parse("https://google.com/search?q=killfeed_agent").unwrap(),
            },
        },
        behavior: PlayerBehavior {
            afk_rounds: 0,
            friendly_fire: PlayerBehaviorFriendlyFire {
                incoming: 0,
                outgoing: 0,
            },
            rounds_in_spawn: 0,
        },
        platform: Platform {
            type_: PlatformType::PC,
            os: PlatformOs {
                name: String::from("Windows"),
                version: String::from("10.0.19045.1.256.64bit"),
            },
        },
        ability_casts: PlayerAbilityCasts {
            c: 7,
            q: 9,
            e: 12,
            x: 3,
        },
        stats: PlayerStats {
            score: 6500,
            kills: 20,
            deaths: 18,
            assists: 7,
            body_shots: 82,
            head_shots: 9,
            leg_shots: 8,
        },
        economy: PlayerEconomy {
            spent: PlayerEconomyCredits {
                overall: 57400,
                average: 2870,
            },
            load_out_value: PlayerEconomyCredits {
                overall: 68300,
                average: 3415,
            },
        },
        damage_made: 4124,
        damage_received: 3330,
    };
    let all = vec![
        red_player.clone(),
        blue_player.clone(),
    ];
    let red = vec![red_player.clone()];
    let blue = vec![blue_player.clone()];

    let json = format!("{{\
    \"all_players\":{0},\
    \"blue\":{1},\
    \"red\":{2}\
    }}",
                       serde_json::to_string(&all).unwrap(),
                       serde_json::to_string(&blue).unwrap(),
                       serde_json::to_string(&red).unwrap(),
    );

    let expected = MatchPlayers {
        all,
        red,
        blue,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
