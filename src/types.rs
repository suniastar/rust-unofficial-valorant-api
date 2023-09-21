#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Affinity {
    Europe,
    NorthAmerica,
    LatinAmerica,
    Brazil,
    AsiaPacific,
    Korea,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
pub enum Platform {
    PC,
    Console,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    Europe,
    NorthAmerica,
    AsiaPacific,
    Korea,
}
