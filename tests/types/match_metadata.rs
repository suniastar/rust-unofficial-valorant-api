use chrono::{Duration, Timelike, TimeZone, Utc};
use rand::Rng;
use uuid::Uuid;

use rust_unofficial_valorant_api::types::{Map, MatchMetadata, MatchMetadataPremierInfo, Mode, PlatformType, Region};

#[test]
fn serialize() {
    let mut rng = rand::thread_rng();

    let id = Uuid::new_v4();
    let map = Map::Sunset;
    let game_version = String::from("release-03.12-shipping-16-649370");
    let game_length = Duration::seconds(2199);
    let game_start = Utc.with_ymd_and_hms(2022, 1, 11, 21, 52, 00)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    let game_start_patched = String::from("Tuesday, January 11 2022 9:52 PM");
    let rounds_played: u32 = rng.gen();
    let mode = Mode::Premier;
    let queue = String::from("Standard");
    let session_id = Uuid::new_v4();
    let platform = PlatformType::PC;
    let premier_info = MatchMetadataPremierInfo {
        tournament_id: Uuid::new_v4(),
        matchup_id: Uuid::new_v4(),
    };
    let region = Region::Europe;
    let cluster = String::from("Frankfurt");

    let expected = format!("{{\
    \"matchid\":\"{id}\",\
    \"map\":\"{map}\",\
    \"game_version\":\"{game_version}\",\
    \"game_length\":{0},\
    \"game_start\":{1},\
    \"game_start_patched\":\"{game_start_patched}\",\
    \"rounds_played\":{rounds_played},\
    \"mode\":\"{mode}\",\
    \"mode_id\":\"{2}\",\
    \"queue\":\"{queue}\",\
    \"session_id\":\"{session_id}\",\
    \"platform\":\"{platform}\",\
    \"premier_info\":{3},\
    \"region\":\"{region}\",\
    \"cluster\":\"{cluster}\"\
    }}",
                           game_length.num_seconds(),
                           game_start.timestamp(),
                           mode.id(),
                           serde_json::to_string(&premier_info).unwrap(),
    );

    let metadata = MatchMetadata {
        id,
        map,
        game_version,
        game_length,
        game_start,
        rounds_played,
        mode,
        queue,
        session_id,
        platform,
        premier_info,
        region,
        cluster,
    };
    let actual = serde_json::to_string(&metadata).unwrap();

    println!("{actual}");
    assert_eq!(expected, actual);
}

#[test]
fn deserialize() {
    let mut rng = rand::thread_rng();

    let id = Uuid::new_v4();
    let map = Map::Sunset;
    let game_version = String::from("release-03.12-shipping-16-649370");
    let game_length = Duration::seconds(2199);
    let game_start = Utc.with_ymd_and_hms(2022, 1, 11, 21, 52, 00)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    let game_start_patched = String::from("Tuesday, January 11 2022 9:52 PM");
    let rounds_played: u32 = rng.gen();
    let mode = Mode::Premier;
    let queue = String::from("Standard");
    let session_id = Uuid::new_v4();
    let platform = PlatformType::PC;
    let premier_info = MatchMetadataPremierInfo {
        tournament_id: Uuid::new_v4(),
        matchup_id: Uuid::new_v4(),
    };
    let region = Region::Europe;
    let cluster = String::from("Frankfurt");

    let json = format!("{{\
    \"matchid\":\"{id}\",\
    \"map\":\"{map}\",\
    \"game_version\":\"{game_version}\",\
    \"game_length\":{0},\
    \"game_start\":{1},\
    \"game_start_patched\":\"{game_start_patched}\",\
    \"rounds_played\":{rounds_played},\
    \"mode\":\"{mode}\",\
    \"mode_id\":\"{2}\",\
    \"queue\":\"{queue}\",\
    \"session_id\":\"{session_id}\",\
    \"platform\":\"{platform}\",\
    \"premier_info\":{3},\
    \"region\":\"{region}\",\
    \"cluster\":\"{cluster}\"\
    }}",
                       game_length.num_seconds(),
                       game_start.timestamp(),
                       mode.id(),
                       serde_json::to_string(&premier_info).unwrap(),
    );

    let expected = MatchMetadata {
        id,
        map,
        game_version,
        game_length,
        game_start,
        rounds_played,
        mode,
        queue,
        session_id,
        platform,
        premier_info,
        region,
        cluster,
    };
    let actual = serde_json::from_str(&json).unwrap();

    println!("{:?}", actual);
    assert_eq!(expected, actual);
}
