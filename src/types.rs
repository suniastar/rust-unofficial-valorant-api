use chrono::{DateTime, Duration, FixedOffset, NaiveDate, Utc};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////////////////////////////////////////////////////////////////////////////////////////////////
// ENUMS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Affinity {
    Europe,
    NorthAmerica,
    LatinAmerica,
    Brazil,
    AsiaPacific,
    Korea,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Character {
    Brimstone = 1,
    Viper = 2,
    Omen = 3,
    Killjoy = 4,
    Cypher = 5,
    Sova = 6,
    Sage = 7,
    // Agent 8 does not exist (yet)
    Phoenix = 9,
    Jett = 10,
    Reyna = 11,
    Raze = 12,
    Breach = 13,
    Skye = 14,
    Yoru = 15,
    Astra = 16,
    KAYO = 17,
    Chamber = 18,
    Neon = 19,
    Fade = 20,
    Harbor = 21,
    Gekko = 22,
    Deadlock = 23,
    Iso = 24,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Map {
    Ascent,
    Bind,
    Breeze,
    District,
    Fracture,
    Haven,
    Icebox,
    Kasbah,
    Lotus,
    Pearl,
    Piazza,
    Split,
    Sunset,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Mode {
    Competitive,
    Unrated,
    Deathmatch,
    TeamDeathmatch,
    Swiftplay,
    SpikeRush,
    NewMap,
    SnowballFight,
    CustomGame,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OfferType {
    Buddy,
    PlayerCard,
    PlayerTitle,
    SkinChroma,
    SkinLevel,
    Spray,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlantSite {
    A,
    B,
    C,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlayerTeam {
    Neutral,
    Red,
    Blue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlatformType {
    PC,
    Console,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Region {
    Europe,
    NorthAmerica,
    AsiaPacific,
    Korea,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RoundEndType {
    Eliminated,
    BombDetonated,
    BombDefused,
    RoundTimerExpired,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MODEL STRUCTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "small")]
    pub small_url: Url,

    #[serde(rename = "large")]
    pub large_url: Url,

    #[serde(rename = "wide")]
    pub wide_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coach {
    #[serde(rename = "puuid")]
    pub id: Uuid,

    #[serde(rename = "team")]
    pub team: PlayerTeam,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Localized<T> {
    #[serde(rename = "content")]
    pub content: T,

    #[serde(rename = "locale")]
    pub locale: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MapCoordinates {
    #[serde(rename = "x")]
    pub x: i32,

    #[serde(rename = "y")]
    pub y: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MapLocation {
    #[serde(rename = "player_puuid")]
    pub id: Uuid,

    #[serde(rename = "player_display_name")]
    pub display_name: String,

    #[serde(rename = "player_team")]
    pub team: PlayerTeam,

    #[serde(rename = "location")]
    pub coordinates: MapCoordinates,

    #[serde(rename = "view_radians")]
    pub view_radians: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchKill {
    #[serde(rename = "kill_time_in_round")]
    pub kill_time_in_round: u32,

    #[serde(rename = "kill_time_in_match")]
    pub kill_time_in_match: u32,

    #[serde(rename = "killer_puuid")]
    pub killer_id: Uuid,

    #[serde(rename = "killer_display_name")]
    pub killer_display_name: String,

    #[serde(rename = "killer_team")]
    pub killer_team: PlayerTeam,

    #[serde(rename = "victim_puuid")]
    pub victim_id: Uuid,

    #[serde(rename = "victim_display_name")]
    pub victim_display_name: String,

    #[serde(rename = "victim_team")]
    pub victim_team: PlayerTeam,

    #[serde(rename = "victim_death_location")]
    pub victim_coordinates: MapCoordinates,

    #[serde(rename = "damage_weapon_id")]
    pub weapon_id: Uuid,

    #[serde(rename = "damage_weapon_name")]
    pub weapon_name: String,

    #[serde(rename = "damage_weapon_assets")]
    pub weapon_assets: MatchKillWeaponAssets,

    #[serde(rename = "secondary_fire_mode")]
    pub secondary_fire_mode: bool,

    #[serde(rename = "player_locations_on_kill")]
    pub player_locations_on_kill: Vec<MapLocation>,

    #[serde(rename = "assistants")]
    pub assistants: Vec<MatchKillAssistant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchKillAssistant {
    #[serde(rename = "assistant_puuid")]
    pub id: Uuid,

    #[serde(rename = "assistant_display_name")]
    pub display_name: String,

    #[serde(rename = "assistant_team")]
    pub team: PlayerTeam,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchKillWeaponAssets {
    #[serde(rename = "display_icon")]
    pub display_icon_url: Url,

    #[serde(rename = "killfeed_icon")]
    pub kill_feed_icon_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRound {
    #[serde(rename = "winning_team")]
    pub winning_team: PlayerTeam,

    #[serde(rename = "end_type")]
    pub end_type: RoundEndType,

    #[serde(rename = "bomb_planted")]
    pub bomb_planted: bool,

    #[serde(rename = "bomb_defused")]
    pub bomb_defused: bool,

    #[serde(rename = "plant_events")]
    pub plant_event: MatchRoundPlantEvent,

    #[serde(rename = "defuse_events")]
    pub defuse_event: MatchRoundDefuseEvent,

    #[serde(rename = "player_stats")]
    pub player_stats: Vec<MatchRoundPlayerStats>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundDefuseEvent {
    #[serde(rename = "defuse_location")]
    pub defuse_coordinates: MapCoordinates,

    #[serde(rename = "defused_by")]
    pub defused_by: MatchRoundEventBy,

    #[serde(rename = "defuse_time_in_round")]
    pub defuse_time_in_round: u32,

    #[serde(rename = "player_locations_on_defuse")]
    pub player_locations: Vec<MapLocation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundEventBy {
    #[serde(rename = "puuid")]
    pub id: Uuid,

    #[serde(rename = "display_name")]
    pub display_name: String,

    #[serde(rename = "team")]
    pub team: PlayerTeam,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlantEvent {
    #[serde(rename = "plant_location")]
    pub plant_coordinates: MapCoordinates,

    #[serde(rename = "planted_by")]
    pub planted_by: MatchRoundEventBy,

    #[serde(rename = "plant_site")]
    pub plant_site: PlantSite,

    #[serde(rename = "plant_time_in_round")]
    pub plant_time_in_round: u32,

    #[serde(rename = "player_locations_on_plant")]
    pub player_locations: Vec<MapLocation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStats {
    #[serde(rename = "player_puuid")]
    pub id: Uuid,

    #[serde(rename = "player_display_name")]
    pub display_name: String,

    #[serde(rename = "player_team")]
    pub team: PlayerTeam,

    #[serde(rename = "ability_casts")]
    pub ability_casts: MatchRoundPlayerStatsAbilityCasts,

    #[serde(rename = "damage_events")]
    pub damage_events: Vec<MatchRoundPlayerStatsDamageEvent>,

    #[serde(rename = "damage")]
    pub damage: u32,

    #[serde(rename = "headshots")]
    pub head_shots: u32,

    #[serde(rename = "bodyshots")]
    pub body_shots: u32,

    #[serde(rename = "legshots")]
    pub leg_shots: u32,

    #[serde(rename = "kill_events")]
    pub kill_events: Vec<MatchKill>,

    #[serde(rename = "kills")]
    pub kills: u32,

    #[serde(rename = "score")]
    pub score: u32,

    #[serde(rename = "economy")]
    pub economy: MatchRoundPlayerStatsEconomy,

    #[serde(rename = "was_afk")]
    pub was_afk: bool,

    #[serde(rename = "was_penalized")]
    pub was_penalized: bool,

    #[serde(rename = "stayed_in_spawn")]
    pub stayed_in_spawn: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsAbilityCasts {
    #[serde(rename = "c_casts")]
    pub c: u32,

    #[serde(rename = "q_casts")]
    pub q: u32,

    #[serde(rename = "e_cast")]
    pub e: u32,

    #[serde(rename = "x_cast")]
    pub x: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsDamageEvent {
    #[serde(rename = "receiver_puuid")]
    pub receiver_id: Uuid,

    #[serde(rename = "receiver_display_name")]
    pub receiver_display_name: String,

    #[serde(rename = "receiver_team")]
    pub receiver_team: PlayerTeam,

    #[serde(rename = "damage")]
    pub damage: u32,

    #[serde(rename = "headshots")]
    pub head_shots: u32,

    #[serde(rename = "bodyshots")]
    pub body_shots: u32,

    #[serde(rename = "legshots")]
    pub leg_shots: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsEconomy {
    #[serde(rename = "loadout_value")]
    pub load_out_value: u32,

    #[serde(rename = "weapon")]
    pub weapon: MatchRoundPlayerStatsEconomyWeapon,

    #[serde(rename = "armor")]
    pub armor: MatchRoundPlayerStatsEconomyArmor,

    #[serde(rename = "remaining")]
    pub remaining: u32,

    #[serde(rename = "spent")]
    pub spent: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsEconomyArmor {
    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "assets")]
    pub assets: MatchRoundPlayerStatsEconomyArmorAssets,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsEconomyArmorAssets {
    #[serde(rename = "display_icon")]
    pub display_icon_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsEconomyWeapon {
    #[serde(rename = "id")]
    pub id: Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "assets")]
    pub assets: MatchRoundPlayerStatsEconomyWeaponAssets,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MatchRoundPlayerStatsEconomyWeaponAssets {
    #[serde(rename = "display_icon")]
    pub display_icon_url: Url,

    #[serde(rename = "killfeed_icon")]
    pub kill_feed_icon_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Observer {
    #[serde(rename = "puuid")]
    pub id: Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "tag")]
    pub tag: String,

    #[serde(rename = "platform")]
    pub platform: Platform,

    #[serde(rename = "session_playtime", serialize_with = "crate::types_serde::serialize_playtime", deserialize_with = "crate::types_serde::deserialize_playtime")]
    pub playtime: Duration,

    #[serde(rename = "team")]
    pub team: PlayerTeam,

    #[serde(rename = "level")]
    pub level: u32,

    #[serde(rename = "player_card")]
    pub card_id: Uuid,

    #[serde(rename = "player_title")]
    pub title_id: Uuid,

    #[serde(rename = "party_id")]
    pub party_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OfferTier {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "dev_name")]
    pub dev_name: String,

    #[serde(rename = "icon")]
    pub icon_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Platform {
    #[serde(rename = "type")]
    pub type_: PlatformType,

    #[serde(rename = "os")]
    pub os: PlatformOs,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlatformOs {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "version")]
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "puuid")]
    pub id: Uuid,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "tag")]
    pub tag: String,

    #[serde(rename = "team")]
    pub team: PlayerTeam,

    #[serde(rename = "level")]
    pub level: u32,

    #[serde(rename = "character")]
    pub character: Character,

    #[serde(rename = "currenttier")]
    pub current_tier: u32,

    #[serde(rename = "currenttier_patched")]
    pub current_tier_patched: String,

    #[serde(rename = "player_card")]
    pub card_id: Uuid,

    #[serde(rename = "player_title")]
    pub title_id: Uuid,

    #[serde(rename = "party_id")]
    pub party_id: Uuid,

    #[serde(rename = "session_playtime", serialize_with = "crate::types_serde::serialize_playtime", deserialize_with = "crate::types_serde::deserialize_playtime")]
    pub playtime: Duration,

    #[serde(rename = "assets")]
    pub assets: PlayerAssets,

    #[serde(rename = "behaviour")]
    pub behaviour: PlayerBehavior,

    #[serde(rename = "platform")]
    pub platform: Platform,

    #[serde(rename = "ability_casts")]
    pub ability_casts: PlayerAbilityCasts,

    #[serde(rename = "stats")]
    pub stats: PlayerStats,

    #[serde(rename = "economy")]
    pub economy: PlayerEconomy,

    #[serde(rename = "damage_made")]
    pub damage_made: u32,

    #[serde(rename = "damage_received")]
    pub damage_received: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerAbilityCasts {
    #[serde(rename = "c_cast")]
    pub c: u32,

    #[serde(rename = "q_cast")]
    pub q: u32,

    #[serde(rename = "e_cast")]
    pub e: u32,

    #[serde(rename = "x_cast")]
    pub x: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerAssets {
    #[serde(rename = "card")]
    pub card: PlayerAssetsCard,

    #[serde(rename = "agent")]
    pub agent: PlayerAssetsAgent,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerAssetsAgent {
    #[serde(rename = "small")]
    pub small_url: Url,

    #[serde(rename = "full")]
    pub full_url: Url,

    #[serde(rename = "bust")]
    pub bust_url: Url,

    #[serde(rename = "killfeed")]
    pub kill_feed_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerAssetsCard {
    #[serde(rename = "small")]
    pub small_url: Url,

    #[serde(rename = "large")]
    pub large_url: Url,

    #[serde(rename = "wide")]
    pub wide_url: Url,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerBehavior {
    #[serde(rename = "afk_rounds")]
    pub afk_rounds: u32,

    #[serde(rename = "friendly_fire")]
    pub friendly_fire: PlayerBehaviorFriendlyFire,

    #[serde(rename = "rounds_in_spawn")]
    pub rounds_in_spawn: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerBehaviorFriendlyFire {
    #[serde(rename = "incoming")]
    pub incoming: u32,

    #[serde(rename = "outgoing")]
    pub outgoing: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerEconomy {
    #[serde(rename = "spent")]
    pub spent: PlayerEconomyCredits,

    #[serde(rename = "loadout_value")]
    pub load_out_value: PlayerEconomyCredits,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerEconomyCredits {
    #[serde(rename = "overall")]
    pub overall: u32,

    #[serde(rename = "average")]
    pub average: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerStats {
    #[serde(rename = "score")]
    pub score: u32,

    #[serde(rename = "kills")]
    pub kills: u32,

    #[serde(rename = "deaths")]
    pub deaths: u32,

    #[serde(rename = "assists")]
    pub assists: u32,

    #[serde(rename = "bodyshots")]
    pub body_shots: u32,

    #[serde(rename = "headshots")]
    pub head_shots: u32,

    #[serde(rename = "legshots")]
    pub leg_shots: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "archive_at")]
    pub archived: Option<DateTime<FixedOffset>>,

    #[serde(rename = "incident_severity")]
    pub incident_severity: Option<String>,

    #[serde(rename = "maintenance_status")]
    pub maintenance_status: String,

    #[serde(rename = "titles")]
    pub titles: Vec<Localized<String>>,

    #[serde(rename = "platforms")]
    pub platforms: Vec<String>,

    #[serde(rename = "updates")]
    pub updates: Vec<StatusUpdate>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusUpdate {
    #[serde(rename = "id")]
    pub id: u32,

    #[serde(rename = "publish")]
    pub publish: bool,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "created_at")]
    pub created: DateTime<FixedOffset>,

    #[serde(rename = "updated_at")]
    pub updated: Option<DateTime<FixedOffset>>,

    #[serde(rename = "publish_locations")]
    pub publish_locations: Vec<String>,

    #[serde(rename = "translations")]
    pub translations: Vec<Localized<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Team {
    #[serde(rename = "has_won")]
    pub has_won: bool,

    #[serde(rename = "rounds_won")]
    pub rounds_won: u32,

    #[serde(rename = "rounds_lost")]
    pub rounds_lost: u32,

    #[serde(rename = "roaster")]
    pub roaster: TeamRoaster,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamRoaster {
    #[serde(rename = "members")]
    pub members: [Uuid; 5],

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "tag")]
    pub tag: String,

    #[serde(rename = "customization")]
    pub customization: TeamRoasterCustomization,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeamRoasterCustomization {
    #[serde(rename = "icon")]
    pub icon: Uuid,

    #[serde(rename = "image")]
    pub image_url: Url,

    #[serde(rename = "primary_color", serialize_with = "crate::types_serde::serialize_color", deserialize_with = "crate::types_serde::deserialize_color")]
    pub color_primary: u32,

    #[serde(rename = "secondary_color", serialize_with = "crate::types_serde::serialize_color", deserialize_with = "crate::types_serde::deserialize_color")]
    pub color_secondary: u32,

    #[serde(rename = "tertiary_color", serialize_with = "crate::types_serde::serialize_color", deserialize_with = "crate::types_serde::deserialize_color")]
    pub color_tertiary: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V1AccountData {
    #[serde(rename = "puuid")]
    pub id: Uuid,

    #[serde(rename = "region")]
    pub region: Region,

    #[serde(rename = "account_level")]
    pub account_level: u32,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "tag")]
    pub tag: Option<String>,

    #[serde(rename = "card")]
    pub card: Card,

    #[serde(rename = "last_update_raw", serialize_with = "crate::types_serde::serialize_date_time_utc", deserialize_with = "crate::types_serde::deserialize_date_time_utc")]
    pub last_update: DateTime<Utc>,

    #[serde(rename = "last_update")]
    pub last_update_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V1StatusData {
    #[serde(rename = "maintenances")]
    pub maintenances: Vec<Status>,

    #[serde(rename = "incidents")]
    pub incidents: Vec<Status>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V1VersionData {
    #[serde(rename = "region")]
    pub region: Region,

    #[serde(rename = "branch")]
    pub branch: String,

    #[serde(rename = "build_date", serialize_with = "crate::types_serde::serialize_naive_date", deserialize_with = "crate::types_serde::deserialize_naive_date")]
    pub build_date: NaiveDate,

    #[serde(rename = "build_ver")]
    pub build_version: String,

    #[serde(rename = "last_checked")]
    pub last_checked: DateTime<FixedOffset>,

    #[serde(rename = "version")]
    pub version: u32,

    #[serde(rename = "version_for_api")]
    pub version_for_api: String,
}

pub type V1WebsiteData = Vec<Website>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V2StoreOffer {
    #[serde(rename = "offer_id")]
    pub offer_id: Uuid,

    #[serde(rename = "cost")]
    pub cost: u32,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "icon")]
    pub icon_url: Option<Url>,

    #[serde(rename = "type")]
    pub type_: OfferType,

    #[serde(rename = "skin_id")]
    pub skin_id: Option<Uuid>,

    #[serde(rename = "content_tier")]
    pub offer_tier: Option<OfferTier>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct V2StoreOffersData {
    #[serde(rename = "offers")]
    pub offers: Vec<V2StoreOffer>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Website {
    #[serde(rename = "banner_url")]
    pub banner_url: Url,

    #[serde(rename = "category")]
    pub category: String,

    #[serde(rename = "date")]
    pub date: DateTime<FixedOffset>,

    #[serde(rename = "external_link")]
    pub external_link: Option<Url>,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: Url,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// API STRUCTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValorantApiError {
    #[serde(rename = "code")]
    pub code: u32,

    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "details")]
    pub details: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValorantApiResponse<T> {
    #[serde(rename = "status")]
    pub status: u16,

    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "tag")]
    pub tag: Option<String>,

    #[serde(rename = "errors")]
    pub errors: Option<Vec<ValorantApiError>>,

    #[serde(rename = "results")]
    pub results: Option<ValorantApiResults>,

    #[serde(rename = "data")]
    pub data: Option<T>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ValorantApiResults {
    #[serde(rename = "total")]
    pub total: u32,

    #[serde(rename = "returned")]
    pub returned: u32,

    #[serde(rename = "before")]
    pub before: u32,

    #[serde(rename = "after")]
    pub after: u32,
}
