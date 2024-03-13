use chrono::Duration;
use rand::Rng;
use url::Url;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Character, Platform, PlatformOs, PlatformType, Player, PlayerAbilityCasts, PlayerAssets, PlayerAssetsAgent, PlayerAssetsCard, PlayerBehavior, PlayerBehaviorFriendlyFire, PlayerEconomy, PlayerEconomyCredits, PlayerStats, PlayerTeam, Tier};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let id = Uuid::new_v4();
    let name = String::from("t00manysecrets");
    let tag = String::from("EUW");
    let team = PlayerTeam::Blue;
    let level: u32 = rng.gen();
    let character = Character::Killjoy;
    let current_tier = Tier::Unrated;
    let card_id = Uuid::new_v4();
    let title_id = Uuid::new_v4();
    let party_id = Uuid::new_v4();
    let playtime = Duration::seconds(8160);
    let behavior = PlayerBehavior {
        afk_rounds: rng.gen(),
        friendly_fire: PlayerBehaviorFriendlyFire {
            incoming: rng.gen(),
            outgoing: rng.gen(),
        },
        rounds_in_spawn: rng.gen(),
    };
    let platform = Platform {
        type_: PlatformType::PC,
        os: PlatformOs {
            name: String::from("Windows"),
            version: String::from("10.0.19045.1.256.64bit"),
        },
    };
    let ability_casts = PlayerAbilityCasts {
        c: rng.gen(),
        q: rng.gen(),
        e: rng.gen(),
        x: rng.gen(),
    };
    let assets = PlayerAssets {
        card: PlayerAssetsCard {
            small_url: Url::parse("https://google.com/search?q=small").unwrap(),
            large_url: Url::parse("https://google.com/search?q=large").unwrap(),
            wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
        },
        agent: PlayerAssetsAgent {
            small_url: Url::parse("https://google.com/search?q=small").unwrap(),
            full_url: Url::parse("https://google.com/search?q=full").unwrap(),
            bust_url: Url::parse("https://google.com/search?q=bust").unwrap(),
            kill_feed_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
        },
    };
    let stats = PlayerStats {
        score: rng.gen(),
        kills: rng.gen(),
        deaths: rng.gen(),
        assists: rng.gen(),
        body_shots: rng.gen(),
        head_shots: rng.gen(),
        leg_shots: rng.gen(),
    };
    let economy = PlayerEconomy {
        spent: PlayerEconomyCredits {
            overall: rng.gen(),
            average: rng.gen(),
        },
        load_out_value: PlayerEconomyCredits {
            overall: rng.gen(),
            average: rng.gen(),
        },
    };
    let damage_made: u32 = rng.gen();
    let damage_received: u32 = rng.gen();

    let expected = format!("{{\
    \"puuid\":\"{id}\",\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"team\":\"{team}\",\
    \"level\":{level},\
    \"character\":\"{character}\",\
    \"currenttier\":{0},\
    \"currenttier_patched\":\"{1}\",\
    \"player_card\":\"{card_id}\",\
    \"player_title\":\"{title_id}\",\
    \"party_id\":\"{party_id}\",\
    \"session_playtime\":{{\
        \"minutes\":136,\
        \"seconds\":8160,\
        \"milliseconds\":8160000\
    }},\
    \"assets\":{2},\
    \"behaviour\":{3},\
    \"platform\":{4},\
    \"ability_casts\":{5},\
    \"stats\":{6},\
    \"economy\":{7},\
    \"damage_made\":{damage_made},\
    \"damage_received\":{damage_received}\
    }}",
                           current_tier.clone() as u8,
                           current_tier.to_str(),
                           serde_json::to_string(&assets).unwrap(),
                           serde_json::to_string(&behavior).unwrap(),
                           serde_json::to_string(&platform).unwrap(),
                           serde_json::to_string(&ability_casts).unwrap(),
                           serde_json::to_string(&stats).unwrap(),
                           serde_json::to_string(&economy).unwrap(),
    );

    let player = Player {
        id,
        name,
        tag,
        team,
        level,
        character,
        current_tier,
        card_id,
        title_id,
        party_id,
        playtime,
        assets,
        behavior,
        platform,
        ability_casts,
        stats,
        economy,
        damage_made,
        damage_received,
    };
    let actual = serde_json::to_string(&player).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let id = Uuid::new_v4();
    let name = String::from("t00manysecrets");
    let tag = String::from("EUW");
    let team = PlayerTeam::Blue;
    let level: u32 = rng.gen();
    let character = Character::Killjoy;
    let current_tier = Tier::Unrated;
    let card_id = Uuid::new_v4();
    let title_id = Uuid::new_v4();
    let party_id = Uuid::new_v4();
    let playtime = Duration::seconds(8160);
    let behavior = PlayerBehavior {
        afk_rounds: rng.gen(),
        friendly_fire: PlayerBehaviorFriendlyFire {
            incoming: rng.gen(),
            outgoing: rng.gen(),
        },
        rounds_in_spawn: rng.gen(),
    };
    let platform = Platform {
        type_: PlatformType::PC,
        os: PlatformOs {
            name: String::from("Windows"),
            version: String::from("10.0.19045.1.256.64bit"),
        },
    };
    let ability_casts = PlayerAbilityCasts {
        c: rng.gen(),
        q: rng.gen(),
        e: rng.gen(),
        x: rng.gen(),
    };
    let assets = PlayerAssets {
        card: PlayerAssetsCard {
            small_url: Url::parse("https://google.com/search?q=small").unwrap(),
            large_url: Url::parse("https://google.com/search?q=large").unwrap(),
            wide_url: Url::parse("https://google.com/search?q=wide").unwrap(),
        },
        agent: PlayerAssetsAgent {
            small_url: Url::parse("https://google.com/search?q=small").unwrap(),
            full_url: Url::parse("https://google.com/search?q=full").unwrap(),
            bust_url: Url::parse("https://google.com/search?q=bust").unwrap(),
            kill_feed_url: Url::parse("https://google.com/search?q=killfeed").unwrap(),
        },
    };
    let stats = PlayerStats {
        score: rng.gen(),
        kills: rng.gen(),
        deaths: rng.gen(),
        assists: rng.gen(),
        body_shots: rng.gen(),
        head_shots: rng.gen(),
        leg_shots: rng.gen(),
    };
    let economy = PlayerEconomy {
        spent: PlayerEconomyCredits {
            overall: rng.gen(),
            average: rng.gen(),
        },
        load_out_value: PlayerEconomyCredits {
            overall: rng.gen(),
            average: rng.gen(),
        },
    };
    let damage_made: u32 = rng.gen();
    let damage_received: u32 = rng.gen();

    let json = format!("{{\
    \"puuid\":\"{id}\",\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"team\":\"{team}\",\
    \"level\":{level},\
    \"character\":\"{character}\",\
    \"currenttier\":{0},\
    \"currenttier_patched\":\"{1}\",\
    \"player_card\":\"{card_id}\",\
    \"player_title\":\"{title_id}\",\
    \"party_id\":\"{party_id}\",\
    \"session_playtime\":{{\
        \"minutes\":136,\
        \"seconds\":8160,\
        \"milliseconds\":8160000\
    }},\
    \"assets\":{2},\
    \"behaviour\":{3},\
    \"platform\":{4},\
    \"ability_casts\":{5},\
    \"stats\":{6},\
    \"economy\":{7},\
    \"damage_made\":{damage_made},\
    \"damage_received\":{damage_received}\
    }}",
                       current_tier.clone() as u8,
                       current_tier.to_str(),
                       serde_json::to_string(&assets).unwrap(),
                       serde_json::to_string(&behavior).unwrap(),
                       serde_json::to_string(&platform).unwrap(),
                       serde_json::to_string(&ability_casts).unwrap(),
                       serde_json::to_string(&stats).unwrap(),
                       serde_json::to_string(&economy).unwrap(),
    );

    let expected = Player {
        id,
        name,
        tag,
        team,
        level,
        character,
        current_tier,
        card_id,
        title_id,
        party_id,
        playtime,
        assets,
        behavior,
        platform,
        ability_casts,
        stats,
        economy,
        damage_made,
        damage_received,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
