use std::env;

use chrono::Duration;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Observer, Platform, PlatformOs, PlatformType, PlayerTeam};

#[test]
fn serialize() {
    let id = Uuid::new_v4();
    let name = String::from("t00manysecrets");
    let tag = String::from("EUW");
    let platform = Platform {
        type_: PlatformType::PC,
        os: PlatformOs {
            name: String::from(env::consts::OS),
            version: String::from(env::consts::ARCH),
        },
    };
    let playtime = Duration::seconds(2220);
    let team = PlayerTeam::Neutral;
    let level = 420;
    let card_id = Uuid::new_v4();
    let title_id = Uuid::new_v4();
    let party_id = Uuid::new_v4();

    let expected = format!("{{\
    \"puuid\":\"{id}\",\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"platform\":{0},\
    \"session_playtime\":{{\
        \"minutes\":{1},\
        \"seconds\":{2},\
        \"milliseconds\":{3}\
    }},\
    \"team\":\"{team}\",\
    \"level\":{level},\
    \"player_card\":\"{card_id}\",\
    \"player_title\":\"{title_id}\",\
    \"party_id\":\"{party_id}\"\
    }}",
                           serde_json::to_string(&platform).unwrap(),
                           playtime.num_minutes(),
                           playtime.num_seconds(),
                           playtime.num_milliseconds(),
    );

    let observer = Observer {
        id,
        name,
        tag,
        platform,
        playtime,
        team,
        level,
        card_id,
        title_id,
        party_id,
    };
    let actual = serde_json::to_string(&observer).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let id = Uuid::new_v4();
    let name = String::from("t00manysecrets");
    let tag = String::from("EUW");
    let platform = Platform {
        type_: PlatformType::PC,
        os: PlatformOs {
            name: String::from(env::consts::OS),
            version: String::from(env::consts::ARCH),
        },
    };
    let playtime = Duration::seconds(2220);
    let team = PlayerTeam::Neutral;
    let level = 420;
    let card_id = Uuid::new_v4();
    let title_id = Uuid::new_v4();
    let party_id = Uuid::new_v4();

    let json = format!("{{\
    \"puuid\":\"{id}\",\
    \"name\":\"{name}\",\
    \"tag\":\"{tag}\",\
    \"platform\":{0},\
    \"session_playtime\":{{\
        \"minutes\":{1},\
        \"seconds\":{2},\
        \"milliseconds\":{3}\
    }},\
    \"team\":\"{team}\",\
    \"level\":{level},\
    \"player_card\":\"{card_id}\",\
    \"player_title\":\"{title_id}\",\
    \"party_id\":\"{party_id}\"\
    }}",
                       serde_json::to_string(&platform).unwrap(),
                       playtime.num_minutes(),
                       playtime.num_seconds(),
                       playtime.num_milliseconds(),
    );

    let expected = Observer {
        id,
        name,
        tag,
        platform,
        playtime,
        team,
        level,
        card_id,
        title_id,
        party_id,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
