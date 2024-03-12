use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use unicase::UniCase;

use crate::types::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// ENUMS
////////////////////////////////////////////////////////////////////////////////////////////////////

const AFFINITY_EUROPE: UniCase<&str> = UniCase::unicode("eu");
const AFFINITY_NORTH_AMERICA: UniCase<&str> = UniCase::unicode("na");
const AFFINITY_LATIN_AMERICA: UniCase<&str> = UniCase::unicode("latam");
const AFFINITY_BRAZIL: UniCase<&str> = UniCase::unicode("br");
const AFFINITY_ASIA_PACIFIC: UniCase<&str> = UniCase::unicode("ap");
const AFFINITY_KOREA: UniCase<&str> = UniCase::unicode("kr");

impl Affinity {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Affinity::Europe => AFFINITY_EUROPE.as_ref(),
            Affinity::NorthAmerica => AFFINITY_NORTH_AMERICA.as_ref(),
            Affinity::LatinAmerica => AFFINITY_LATIN_AMERICA.as_ref(),
            Affinity::Brazil => AFFINITY_BRAZIL.as_ref(),
            Affinity::AsiaPacific => AFFINITY_ASIA_PACIFIC.as_ref(),
            Affinity::Korea => AFFINITY_KOREA.as_ref(),
        }
    }
}

impl fmt::Display for Affinity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Affinity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let a = UniCase::unicode(s);
        if a == AFFINITY_EUROPE {
            Ok(Affinity::Europe)
        } else if a == AFFINITY_NORTH_AMERICA {
            Ok(Affinity::NorthAmerica)
        } else if a == AFFINITY_LATIN_AMERICA {
            Ok(Affinity::LatinAmerica)
        } else if a == AFFINITY_BRAZIL {
            Ok(Affinity::Brazil)
        } else if a == AFFINITY_ASIA_PACIFIC {
            Ok(Affinity::AsiaPacific)
        } else if a == AFFINITY_KOREA {
            Ok(Affinity::Korea)
        } else {
            Err(format!("not an affinity: {s}"))
        }
    }
}

const CHARACTER_BRIMSTONE: UniCase<&str> = UniCase::unicode("Brimstone");
const CHARACTER_VIPER: UniCase<&str> = UniCase::unicode("Viper");
const CHARACTER_OMEN: UniCase<&str> = UniCase::unicode("Omen");
const CHARACTER_KILLJOY: UniCase<&str> = UniCase::unicode("Killjoy");
const CHARACTER_CYPHER: UniCase<&str> = UniCase::unicode("Cypher");
const CHARACTER_SOVA: UniCase<&str> = UniCase::unicode("Sova");
const CHARACTER_SAGE: UniCase<&str> = UniCase::unicode("Sage");
const CHARACTER_PHOENIX: UniCase<&str> = UniCase::unicode("Phoenix");
const CHARACTER_JETT: UniCase<&str> = UniCase::unicode("Jett");
const CHARACTER_REYNA: UniCase<&str> = UniCase::unicode("Reyna");
const CHARACTER_RAZE: UniCase<&str> = UniCase::unicode("Raze");
const CHARACTER_BREACH: UniCase<&str> = UniCase::unicode("Breach");
const CHARACTER_SKYE: UniCase<&str> = UniCase::unicode("Skye");
const CHARACTER_YORU: UniCase<&str> = UniCase::unicode("Yoru");
const CHARACTER_ASTRA: UniCase<&str> = UniCase::unicode("Astra");
const CHARACTER_KAYO: UniCase<&str> = UniCase::unicode("KAY/O");
const CHARACTER_CHAMBER: UniCase<&str> = UniCase::unicode("Chamber");
const CHARACTER_NEON: UniCase<&str> = UniCase::unicode("Neon");
const CHARACTER_FADE: UniCase<&str> = UniCase::unicode("Fade");
const CHARACTER_HARBOR: UniCase<&str> = UniCase::unicode("Harbor");
const CHARACTER_GEKKO: UniCase<&str> = UniCase::unicode("Gekko");
const CHARACTER_DEADLOCK: UniCase<&str> = UniCase::unicode("Deadlock");
const CHARACTER_ISO: UniCase<&str> = UniCase::unicode("Iso");

impl Character {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Character::Brimstone => CHARACTER_BRIMSTONE.as_ref(),
            Character::Viper => CHARACTER_VIPER.as_ref(),
            Character::Omen => CHARACTER_OMEN.as_ref(),
            Character::Killjoy => CHARACTER_KILLJOY.as_ref(),
            Character::Cypher => CHARACTER_CYPHER.as_ref(),
            Character::Sova => CHARACTER_SOVA.as_ref(),
            Character::Sage => CHARACTER_SAGE.as_ref(),
            Character::Phoenix => CHARACTER_PHOENIX.as_ref(),
            Character::Jett => CHARACTER_JETT.as_ref(),
            Character::Reyna => CHARACTER_REYNA.as_ref(),
            Character::Raze => CHARACTER_RAZE.as_ref(),
            Character::Breach => CHARACTER_BREACH.as_ref(),
            Character::Skye => CHARACTER_SKYE.as_ref(),
            Character::Yoru => CHARACTER_YORU.as_ref(),
            Character::Astra => CHARACTER_ASTRA.as_ref(),
            Character::KAYO => CHARACTER_KAYO.as_ref(),
            Character::Chamber => CHARACTER_CHAMBER.as_ref(),
            Character::Neon => CHARACTER_NEON.as_ref(),
            Character::Fade => CHARACTER_FADE.as_ref(),
            Character::Harbor => CHARACTER_HARBOR.as_ref(),
            Character::Gekko => CHARACTER_GEKKO.as_ref(),
            Character::Deadlock => CHARACTER_DEADLOCK.as_ref(),
            Character::Iso => CHARACTER_ISO.as_ref(),
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Character {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = UniCase::unicode(s);
        if c == CHARACTER_BRIMSTONE {
            Ok(Character::Brimstone)
        } else if c == CHARACTER_VIPER {
            Ok(Character::Viper)
        } else if c == CHARACTER_OMEN {
            Ok(Character::Omen)
        } else if c == CHARACTER_KILLJOY {
            Ok(Character::Killjoy)
        } else if c == CHARACTER_CYPHER {
            Ok(Character::Cypher)
        } else if c == CHARACTER_SOVA {
            Ok(Character::Sova)
        } else if c == CHARACTER_SAGE {
            Ok(Character::Sage)
        } else if c == CHARACTER_PHOENIX {
            Ok(Character::Phoenix)
        } else if c == CHARACTER_JETT {
            Ok(Character::Jett)
        } else if c == CHARACTER_REYNA {
            Ok(Character::Reyna)
        } else if c == CHARACTER_RAZE {
            Ok(Character::Raze)
        } else if c == CHARACTER_BREACH {
            Ok(Character::Breach)
        } else if c == CHARACTER_SKYE {
            Ok(Character::Skye)
        } else if c == CHARACTER_YORU {
            Ok(Character::Yoru)
        } else if c == CHARACTER_ASTRA {
            Ok(Character::Astra)
        } else if c == CHARACTER_KAYO {
            Ok(Character::KAYO)
        } else if c == CHARACTER_CHAMBER {
            Ok(Character::Chamber)
        } else if c == CHARACTER_NEON {
            Ok(Character::Neon)
        } else if c == CHARACTER_FADE {
            Ok(Character::Fade)
        } else if c == CHARACTER_HARBOR {
            Ok(Character::Harbor)
        } else if c == CHARACTER_GEKKO {
            Ok(Character::Gekko)
        } else if c == CHARACTER_DEADLOCK {
            Ok(Character::Deadlock)
        } else if c == CHARACTER_ISO {
            Ok(Character::Iso)
        } else {
            Err(format!("not a character: {s}"))
        }
    }
}

const MAP_ASCENT: UniCase<&str> = UniCase::unicode("Ascent");
const MAP_BIND: UniCase<&str> = UniCase::unicode("Bind");
const MAP_BREEZE: UniCase<&str> = UniCase::unicode("Breeze");
const MAP_DISTRICT: UniCase<&str> = UniCase::unicode("District");
const MAP_FRACTURE: UniCase<&str> = UniCase::unicode("Fracture");
const MAP_HAVEN: UniCase<&str> = UniCase::unicode("Haven");
const MAP_ICEBOX: UniCase<&str> = UniCase::unicode("Icebox");
const MAP_KASBAH: UniCase<&str> = UniCase::unicode("Kasbah");
const MAP_LOTUS: UniCase<&str> = UniCase::unicode("Lotus");
const MAP_PEARL: UniCase<&str> = UniCase::unicode("Pearl");
const MAP_PIAZZA: UniCase<&str> = UniCase::unicode("Piazza");
const MAP_SPLIT: UniCase<&str> = UniCase::unicode("Split");
const MAP_SUNSET: UniCase<&str> = UniCase::unicode("Sunset");

impl Map {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Map::Ascent => MAP_ASCENT.as_ref(),
            Map::Bind => MAP_BIND.as_ref(),
            Map::Breeze => MAP_BREEZE.as_ref(),
            Map::District => MAP_DISTRICT.as_ref(),
            Map::Fracture => MAP_FRACTURE.as_ref(),
            Map::Haven => MAP_HAVEN.as_ref(),
            Map::Icebox => MAP_ICEBOX.as_ref(),
            Map::Kasbah => MAP_KASBAH.as_ref(),
            Map::Lotus => MAP_LOTUS.as_ref(),
            Map::Pearl => MAP_PEARL.as_ref(),
            Map::Piazza => MAP_PIAZZA.as_ref(),
            Map::Split => MAP_SPLIT.as_ref(),
            Map::Sunset => MAP_SUNSET.as_ref(),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = UniCase::unicode(s);
        if m == MAP_ASCENT {
            Ok(Map::Ascent)
        } else if m == MAP_BIND {
            Ok(Map::Bind)
        } else if m == MAP_BREEZE {
            Ok(Map::Breeze)
        } else if m == MAP_DISTRICT {
            Ok(Map::District)
        } else if m == MAP_FRACTURE {
            Ok(Map::Fracture)
        } else if m == MAP_HAVEN {
            Ok(Map::Haven)
        } else if m == MAP_ICEBOX {
            Ok(Map::Icebox)
        } else if m == MAP_KASBAH {
            Ok(Map::Kasbah)
        } else if m == MAP_LOTUS {
            Ok(Map::Lotus)
        } else if m == MAP_PEARL {
            Ok(Map::Pearl)
        } else if m == MAP_PIAZZA {
            Ok(Map::Piazza)
        } else if m == MAP_SPLIT {
            Ok(Map::Split)
        } else if m == MAP_SUNSET {
            Ok(Map::Sunset)
        } else {
            Err(format!("not a map: {s}"))
        }
    }
}

const MODE_COMPETITIVE: UniCase<&str> = UniCase::unicode("Competitive");
const MODE_COMPETITIVE_ID: UniCase<&str> = UniCase::unicode("competitive");
const MODE_COMPETITIVE_API: UniCase<&str> = UniCase::unicode("competitive");
const MODE_PREMIER: UniCase<&str> = UniCase::unicode("Premier");
const MODE_PREMIER_ID: UniCase<&str> = UniCase::unicode("premier");
const MODE_PREMIER_API: UniCase<&str> = UniCase::unicode("premier");
const MODE_UNRATED: UniCase<&str> = UniCase::unicode("Unrated");
const MODE_UNRATED_ID: UniCase<&str> = UniCase::unicode("unrated");
const MODE_UNRATED_API: UniCase<&str> = UniCase::unicode("unrated");
const MODE_DEATHMATCH: UniCase<&str> = UniCase::unicode("Deathmatch");
const MODE_DEATHMATCH_ID: UniCase<&str> = UniCase::unicode("deathmatch");
const MODE_DEATHMATCH_API: UniCase<&str> = UniCase::unicode("deathmatch");
const MODE_TEAM_DEATHMATCH: UniCase<&str> = UniCase::unicode("Team Deathmatch");
const MODE_TEAM_DEATHMATCH_ID: UniCase<&str> = UniCase::unicode("hurm");
const MODE_TEAM_DEATHMATCH_API: UniCase<&str> = UniCase::unicode("teamdeathmatch");
const MODE_SWIFTPLAY: UniCase<&str> = UniCase::unicode("Swiftplay");
const MODE_SWIFTPLAY_ID: UniCase<&str> = UniCase::unicode("swiftplay");
const MODE_SWIFTPLAY_API: UniCase<&str> = UniCase::unicode("swiftplay");
const MODE_SPIKE_RUSH: UniCase<&str> = UniCase::unicode("Spike Rush");
const MODE_SPIKE_RUSH_ID: UniCase<&str> = UniCase::unicode("spikerush");
const MODE_SPIKE_RUSH_API: UniCase<&str> = UniCase::unicode("spikerush");
const MODE_NEW_MAP: UniCase<&str> = UniCase::unicode("New Map");
const MODE_NEW_MAP_ID: UniCase<&str> = UniCase::unicode("newmap");
const MODE_NEW_MAP_API: UniCase<&str> = UniCase::unicode("newmap");
const MODE_SNOWBALL_FIGHT: UniCase<&str> = UniCase::unicode("Snowball Fight");
const MODE_SNOWBALL_FIGHT_ID: UniCase<&str> = UniCase::unicode("snowball");
const MODE_SNOWBALL_FIGHT_API: UniCase<&str> = UniCase::unicode("snowballfight");
const MODE_CUSTOM_GAME: UniCase<&str> = UniCase::unicode("Custom Game");
const MODE_CUSTOM_GAME_ID: UniCase<&str> = UniCase::unicode("custom");
const MODE_CUSTOM_GAME_API: UniCase<&str> = UniCase::unicode("custom");

impl Mode {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Mode::Competitive => MODE_COMPETITIVE.as_ref(),
            Mode::Premier => MODE_PREMIER.as_ref(),
            Mode::Unrated => MODE_UNRATED.as_ref(),
            Mode::Deathmatch => MODE_DEATHMATCH.as_ref(),
            Mode::TeamDeathmatch => MODE_TEAM_DEATHMATCH.as_ref(),
            Mode::Swiftplay => MODE_SWIFTPLAY.as_ref(),
            Mode::SpikeRush => MODE_SPIKE_RUSH.as_ref(),
            Mode::NewMap => MODE_NEW_MAP.as_ref(),
            Mode::SnowballFight => MODE_SNOWBALL_FIGHT.as_ref(),
            Mode::CustomGame => MODE_CUSTOM_GAME.as_ref(),
        }
    }

    pub fn api(&self) -> &'static str {
        match *self {
            Mode::Competitive => MODE_COMPETITIVE_API.as_ref(),
            Mode::Premier => MODE_PREMIER_API.as_ref(),
            Mode::Unrated => MODE_UNRATED_API.as_ref(),
            Mode::Deathmatch => MODE_DEATHMATCH_API.as_ref(),
            Mode::TeamDeathmatch => MODE_TEAM_DEATHMATCH_API.as_ref(),
            Mode::Swiftplay => MODE_SWIFTPLAY_API.as_ref(),
            Mode::SpikeRush => MODE_SPIKE_RUSH_API.as_ref(),
            Mode::NewMap => MODE_NEW_MAP_API.as_ref(),
            Mode::SnowballFight => MODE_SNOWBALL_FIGHT_API.as_ref(),
            Mode::CustomGame => MODE_CUSTOM_GAME_API.as_ref(),
        }
    }

    pub fn id(&self) -> &'static str {
        match *self {
            Mode::Competitive => MODE_COMPETITIVE_ID.as_ref(),
            Mode::Premier => MODE_PREMIER_ID.as_ref(),
            Mode::Unrated => MODE_UNRATED_ID.as_ref(),
            Mode::Deathmatch => MODE_DEATHMATCH_ID.as_ref(),
            Mode::TeamDeathmatch => MODE_TEAM_DEATHMATCH_ID.as_ref(),
            Mode::Swiftplay => MODE_SWIFTPLAY_ID.as_ref(),
            Mode::SpikeRush => MODE_SPIKE_RUSH_ID.as_ref(),
            Mode::NewMap => MODE_NEW_MAP_ID.as_ref(),
            Mode::SnowballFight => MODE_SNOWBALL_FIGHT_ID.as_ref(),
            Mode::CustomGame => MODE_CUSTOM_GAME_ID.as_ref(),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m = UniCase::unicode(s);
        if m == MODE_COMPETITIVE {
            Ok(Mode::Competitive)
        } else if m == MODE_COMPETITIVE_API {
            Ok(Mode::Competitive)
        } else if m == MODE_COMPETITIVE_ID {
            Ok(Mode::Competitive)
        } else if m == MODE_PREMIER {
            Ok(Mode::Premier)
        } else if m == MODE_PREMIER_API {
            Ok(Mode::Premier)
        } else if m == MODE_PREMIER_ID {
            Ok(Mode::Premier)
        } else if m == MODE_UNRATED {
            Ok(Mode::Unrated)
        } else if m == MODE_UNRATED_API {
            Ok(Mode::Unrated)
        } else if m == MODE_UNRATED_ID {
            Ok(Mode::Unrated)
        } else if m == MODE_DEATHMATCH {
            Ok(Mode::Deathmatch)
        } else if m == MODE_DEATHMATCH_API {
            Ok(Mode::Deathmatch)
        } else if m == MODE_DEATHMATCH_ID {
            Ok(Mode::Deathmatch)
        } else if m == MODE_TEAM_DEATHMATCH {
            Ok(Mode::TeamDeathmatch)
        } else if m == MODE_TEAM_DEATHMATCH_API {
            Ok(Mode::TeamDeathmatch)
        } else if m == MODE_TEAM_DEATHMATCH_ID {
            Ok(Mode::TeamDeathmatch)
        } else if m == MODE_SWIFTPLAY {
            Ok(Mode::Swiftplay)
        } else if m == MODE_SWIFTPLAY_API {
            Ok(Mode::Swiftplay)
        } else if m == MODE_SWIFTPLAY_ID {
            Ok(Mode::Swiftplay)
        } else if m == MODE_SPIKE_RUSH {
            Ok(Mode::SpikeRush)
        } else if m == MODE_SPIKE_RUSH_API {
            Ok(Mode::SpikeRush)
        } else if m == MODE_SPIKE_RUSH_ID {
            Ok(Mode::SpikeRush)
        } else if m == MODE_NEW_MAP {
            Ok(Mode::NewMap)
        } else if m == MODE_NEW_MAP_API {
            Ok(Mode::NewMap)
        } else if m == MODE_NEW_MAP_ID {
            Ok(Mode::NewMap)
        } else if m == MODE_SNOWBALL_FIGHT {
            Ok(Mode::SnowballFight)
        } else if m == MODE_SNOWBALL_FIGHT_API {
            Ok(Mode::SnowballFight)
        } else if m == MODE_SNOWBALL_FIGHT_ID {
            Ok(Mode::SnowballFight)
        } else if m == MODE_CUSTOM_GAME {
            Ok(Mode::CustomGame)
        } else if m == MODE_CUSTOM_GAME_API {
            Ok(Mode::CustomGame)
        } else if m == MODE_CUSTOM_GAME_ID {
            Ok(Mode::CustomGame)
        } else {
            Err(format!("not a mode: {s}"))
        }
    }
}

const OFFER_TYPE_BUDDY: UniCase<&str> = UniCase::unicode("buddy");
const OFFER_TYPE_PLAYER_CARD: UniCase<&str> = UniCase::unicode("player_card");
const OFFER_TYPE_PLAYER_TITLE: UniCase<&str> = UniCase::unicode("player_title");
const OFFER_TYPE_SKIN_CHROMA: UniCase<&str> = UniCase::unicode("skin_chroma");
const OFFER_TYPE_SKIN_LEVEL: UniCase<&str> = UniCase::unicode("skin_level");
const OFFER_TYPE_SPRAY: UniCase<&str> = UniCase::unicode("spray");

impl OfferType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            OfferType::Buddy => OFFER_TYPE_BUDDY.as_ref(),
            OfferType::PlayerCard => OFFER_TYPE_PLAYER_CARD.as_ref(),
            OfferType::PlayerTitle => OFFER_TYPE_PLAYER_TITLE.as_ref(),
            OfferType::SkinChroma => OFFER_TYPE_SKIN_CHROMA.as_ref(),
            OfferType::SkinLevel => OFFER_TYPE_SKIN_LEVEL.as_ref(),
            OfferType::Spray => OFFER_TYPE_SPRAY.as_ref(),
        }
    }
}

impl fmt::Display for OfferType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for OfferType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let o = UniCase::unicode(s);
        if o == OFFER_TYPE_BUDDY {
            Ok(OfferType::Buddy)
        } else if o == OFFER_TYPE_PLAYER_CARD {
            Ok(OfferType::PlayerCard)
        } else if o == OFFER_TYPE_PLAYER_TITLE {
            Ok(OfferType::PlayerTitle)
        } else if o == OFFER_TYPE_SKIN_CHROMA {
            Ok(OfferType::SkinChroma)
        } else if o == OFFER_TYPE_SKIN_LEVEL {
            Ok(OfferType::SkinLevel)
        } else if o == OFFER_TYPE_SPRAY {
            Ok(OfferType::Spray)
        } else {
            Err(format!("not a offer type: {s}"))
        }
    }
}

const PLANT_SITE_A: UniCase<&str> = UniCase::unicode("A");
const PLANT_SITE_B: UniCase<&str> = UniCase::unicode("B");
const PLANT_SITE_C: UniCase<&str> = UniCase::unicode("C");

impl PlantSite {
    pub fn to_str(&self) -> &'static str {
        match *self {
            PlantSite::A => PLANT_SITE_A.as_ref(),
            PlantSite::B => PLANT_SITE_B.as_ref(),
            PlantSite::C => PLANT_SITE_C.as_ref(),
        }
    }
}

impl fmt::Display for PlantSite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for PlantSite {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = UniCase::unicode(s);
        if p == PLANT_SITE_A {
            Ok(PlantSite::A)
        } else if p == PLANT_SITE_B {
            Ok(PlantSite::B)
        } else if p == PLANT_SITE_C {
            Ok(PlantSite::C)
        } else {
            Err(format!("not a plant site: {s}"))
        }
    }
}

const PLAYER_TEAM_NEUTRAL: UniCase<&str> = UniCase::unicode("Neutral");
const PLAYER_TEAM_RED: UniCase<&str> = UniCase::unicode("Red");
const PLAYER_TEAM_BLUE: UniCase<&str> = UniCase::unicode("Blue");

impl PlayerTeam {
    pub fn to_str(&self) -> &'static str {
        match *self {
            PlayerTeam::Neutral => PLAYER_TEAM_NEUTRAL.as_ref(),
            PlayerTeam::Red => PLAYER_TEAM_RED.as_ref(),
            PlayerTeam::Blue => PLAYER_TEAM_BLUE.as_ref(),
        }
    }
}

impl fmt::Display for PlayerTeam {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for PlayerTeam {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = UniCase::unicode(s);
        if p == PLAYER_TEAM_NEUTRAL {
            Ok(PlayerTeam::Neutral)
        } else if p == PLAYER_TEAM_RED {
            Ok(PlayerTeam::Red)
        } else if p == PLAYER_TEAM_BLUE {
            Ok(PlayerTeam::Blue)
        } else {
            Err(format!("not a player team: {s}"))
        }
    }
}

const PLATFORM_TYPE_PC: UniCase<&str> = UniCase::unicode("PC");
const PLATFORM_TYPE_CONSOLE: UniCase<&str> = UniCase::unicode("Console");

impl PlatformType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            PlatformType::PC => PLATFORM_TYPE_PC.as_ref(),
            PlatformType::Console => PLATFORM_TYPE_CONSOLE.as_ref(),
        }
    }
}

impl fmt::Display for PlatformType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for PlatformType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = UniCase::unicode(s);
        if p == PLATFORM_TYPE_PC {
            Ok(PlatformType::PC)
        } else if p == PLATFORM_TYPE_CONSOLE {
            Ok(PlatformType::Console)
        } else {
            Err(format!("not a platform: {s}"))
        }
    }
}

const REGION_EUROPE: UniCase<&str> = UniCase::unicode("eu");
const REGION_NORTH_AMERICA: UniCase<&str> = UniCase::unicode("na");
const REGION_ASIA_PACIFIC: UniCase<&str> = UniCase::unicode("ap");
const REGION_KOREA: UniCase<&str> = UniCase::unicode("kr");

impl Region {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Region::Europe => REGION_EUROPE.as_ref(),
            Region::NorthAmerica => REGION_NORTH_AMERICA.as_ref(),
            Region::AsiaPacific => REGION_ASIA_PACIFIC.as_ref(),
            Region::Korea => REGION_KOREA.as_ref(),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for Region {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = UniCase::unicode(s);
        if r == REGION_EUROPE {
            Ok(Region::Europe)
        } else if r == REGION_NORTH_AMERICA {
            Ok(Region::NorthAmerica)
        } else if r == REGION_ASIA_PACIFIC {
            Ok(Region::AsiaPacific)
        } else if r == REGION_KOREA {
            Ok(Region::Korea)
        } else {
            Err(format!("not a region: {s}"))
        }
    }
}

const ROUND_END_TYPE_ELIMINATED: UniCase<&str> = UniCase::unicode("Eliminated");
const ROUND_END_TYPE_BOMB_DETONATED: UniCase<&str> = UniCase::unicode("Bomb detonated");
const ROUND_END_TYPE_BOMB_DEFUSED: UniCase<&str> = UniCase::unicode("Bomb defused");
const ROUND_END_TYPE_ROUND_TIMER_EXPIRED: UniCase<&str> = UniCase::unicode("Round timer expired");

impl RoundEndType {
    pub fn to_str(&self) -> &'static str {
        match *self {
            RoundEndType::Eliminated => ROUND_END_TYPE_ELIMINATED.as_ref(),
            RoundEndType::BombDetonated => ROUND_END_TYPE_BOMB_DETONATED.as_ref(),
            RoundEndType::BombDefused => ROUND_END_TYPE_BOMB_DEFUSED.as_ref(),
            RoundEndType::RoundTimerExpired => ROUND_END_TYPE_ROUND_TIMER_EXPIRED.as_ref(),
        }
    }
}

impl fmt::Display for RoundEndType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

impl FromStr for RoundEndType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = UniCase::unicode(s);
        if r == ROUND_END_TYPE_ELIMINATED {
            Ok(RoundEndType::Eliminated)
        } else if r == ROUND_END_TYPE_BOMB_DETONATED {
            Ok(RoundEndType::BombDetonated)
        } else if r == ROUND_END_TYPE_BOMB_DEFUSED {
            Ok(RoundEndType::BombDefused)
        } else if r == ROUND_END_TYPE_ROUND_TIMER_EXPIRED {
            Ok(RoundEndType::RoundTimerExpired)
        } else {
            Err(format!("not a round end type: {s}"))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MODEL STRUCTS
////////////////////////////////////////////////////////////////////////////////////////////////////

impl PartialEq for MapLocation {
    fn eq(&self, other: &Self) -> bool {
        let double_equal = if !self.view_radians.is_nan() && !other.view_radians.is_nan() {
            self.view_radians == other.view_radians
        } else if self.view_radians.is_nan() && other.view_radians.is_nan() {
            true
        } else {
            false
        };

        self.id == other.id &&
            self.display_name == other.display_name &&
            self.team == other.team &&
            self.coordinates == other.coordinates &&
            double_equal
    }
}

impl Eq for MapLocation {}

impl Hash for MapLocation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.display_name.hash(state);
        self.team.hash(state);
        self.coordinates.hash(state);
        if !self.view_radians.is_nan() {
            self.view_radians.to_bits().hash(state);
        } else {
            f64::MAX.to_bits().hash(state);
        }
    }
}
