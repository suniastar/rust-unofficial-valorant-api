use std::str::FromStr;

use rust_unofficial_valorant_api::types::Tier;

#[test]
fn to_str() {
    assert_eq!("Unrated", Tier::Unrated.to_str());
    assert_eq!("Unknown 1", Tier::Unknown1.to_str());
    assert_eq!("Unknown 2", Tier::Unknown2.to_str());
    assert_eq!("Iron 1", Tier::Iron1.to_str());
    assert_eq!("Iron 2", Tier::Iron2.to_str());
    assert_eq!("Iron 3", Tier::Iron3.to_str());
    assert_eq!("Bronze 1", Tier::Bronze1.to_str());
    assert_eq!("Bronze 2", Tier::Bronze2.to_str());
    assert_eq!("Bronze 3", Tier::Bronze3.to_str());
    assert_eq!("Silver 1", Tier::Silver1.to_str());
    assert_eq!("Silver 2", Tier::Silver2.to_str());
    assert_eq!("Silver 3", Tier::Silver3.to_str());
    assert_eq!("Gold 1", Tier::Gold1.to_str());
    assert_eq!("Gold 2", Tier::Gold2.to_str());
    assert_eq!("Gold 3", Tier::Gold3.to_str());
    assert_eq!("Platinum 1", Tier::Platinum1.to_str());
    assert_eq!("Platinum 2", Tier::Platinum2.to_str());
    assert_eq!("Platinum 3", Tier::Platinum3.to_str());
    assert_eq!("Diamond 1", Tier::Diamond1.to_str());
    assert_eq!("Diamond 2", Tier::Diamond2.to_str());
    assert_eq!("Diamond 3", Tier::Diamond3.to_str());
    assert_eq!("Ascendant 1", Tier::Ascendant1.to_str());
    assert_eq!("Ascendant 2", Tier::Ascendant2.to_str());
    assert_eq!("Ascendant 3", Tier::Ascendant3.to_str());
    assert_eq!("Immortal 1", Tier::Immortal1.to_str());
    assert_eq!("Immortal 2", Tier::Immortal2.to_str());
    assert_eq!("Immortal 3", Tier::Immortal3.to_str());
    assert_eq!("Radiant", Tier::Radiant.to_str());
}

#[test]
fn to_u32() {
    assert_eq!(0, Tier::Unrated as u32);
    assert_eq!(1, Tier::Unknown1 as u32);
    assert_eq!(2, Tier::Unknown2 as u32);
    assert_eq!(3, Tier::Iron1 as u32);
    assert_eq!(4, Tier::Iron2 as u32);
    assert_eq!(5, Tier::Iron3 as u32);
    assert_eq!(6, Tier::Bronze1 as u32);
    assert_eq!(7, Tier::Bronze2 as u32);
    assert_eq!(8, Tier::Bronze3 as u32);
    assert_eq!(9, Tier::Silver1 as u32);
    assert_eq!(10, Tier::Silver2 as u32);
    assert_eq!(11, Tier::Silver3 as u32);
    assert_eq!(12, Tier::Gold1 as u32);
    assert_eq!(13, Tier::Gold2 as u32);
    assert_eq!(14, Tier::Gold3 as u32);
    assert_eq!(15, Tier::Platinum1 as u32);
    assert_eq!(16, Tier::Platinum2 as u32);
    assert_eq!(17, Tier::Platinum3 as u32);
    assert_eq!(18, Tier::Diamond1 as u32);
    assert_eq!(19, Tier::Diamond2 as u32);
    assert_eq!(20, Tier::Diamond3 as u32);
    assert_eq!(21, Tier::Ascendant1 as u32);
    assert_eq!(22, Tier::Ascendant2 as u32);
    assert_eq!(23, Tier::Ascendant3 as u32);
    assert_eq!(24, Tier::Immortal1 as u32);
    assert_eq!(25, Tier::Immortal2 as u32);
    assert_eq!(26, Tier::Immortal3 as u32);
    assert_eq!(27, Tier::Radiant as u32);
}

#[test]
fn from_str() {
    assert_eq!(Tier::Unrated, Tier::from_str("Unrated").unwrap());
    assert_eq!(Tier::Unknown1, Tier::from_str("Unknown 1").unwrap());
    assert_eq!(Tier::Unknown2, Tier::from_str("Unknown 2").unwrap());
    assert_eq!(Tier::Iron1, Tier::from_str("Iron 1").unwrap());
    assert_eq!(Tier::Iron2, Tier::from_str("Iron 2").unwrap());
    assert_eq!(Tier::Iron3, Tier::from_str("Iron 3").unwrap());
    assert_eq!(Tier::Bronze1, Tier::from_str("Bronze 1").unwrap());
    assert_eq!(Tier::Bronze2, Tier::from_str("Bronze 2").unwrap());
    assert_eq!(Tier::Bronze3, Tier::from_str("Bronze 3").unwrap());
    assert_eq!(Tier::Silver1, Tier::from_str("Silver 1").unwrap());
    assert_eq!(Tier::Silver2, Tier::from_str("Silver 2").unwrap());
    assert_eq!(Tier::Silver3, Tier::from_str("Silver 3").unwrap());
    assert_eq!(Tier::Gold1, Tier::from_str("Gold 1").unwrap());
    assert_eq!(Tier::Gold2, Tier::from_str("Gold 2").unwrap());
    assert_eq!(Tier::Gold3, Tier::from_str("Gold 3").unwrap());
    assert_eq!(Tier::Platinum1, Tier::from_str("Platinum 1").unwrap());
    assert_eq!(Tier::Platinum2, Tier::from_str("Platinum 2").unwrap());
    assert_eq!(Tier::Platinum3, Tier::from_str("Platinum 3").unwrap());
    assert_eq!(Tier::Diamond1, Tier::from_str("Diamond 1").unwrap());
    assert_eq!(Tier::Diamond2, Tier::from_str("Diamond 2").unwrap());
    assert_eq!(Tier::Diamond3, Tier::from_str("Diamond 3").unwrap());
    assert_eq!(Tier::Ascendant1, Tier::from_str("Ascendant 1").unwrap());
    assert_eq!(Tier::Ascendant2, Tier::from_str("Ascendant 2").unwrap());
    assert_eq!(Tier::Ascendant3, Tier::from_str("Ascendant 3").unwrap());
    assert_eq!(Tier::Immortal1, Tier::from_str("Immortal 1").unwrap());
    assert_eq!(Tier::Immortal2, Tier::from_str("Immortal 2").unwrap());
    assert_eq!(Tier::Immortal3, Tier::from_str("Immortal 3").unwrap());
    assert_eq!(Tier::Radiant, Tier::from_str("Radiant").unwrap());

    assert_eq!(Tier::Unrated, Tier::from_str("UNRATED").unwrap());
    assert_eq!(Tier::Unknown1, Tier::from_str("Unknown 1").unwrap());
    assert_eq!(Tier::Unknown2, Tier::from_str("unknown 2").unwrap());
    assert_eq!(Tier::Iron1, Tier::from_str("Iron 1").unwrap());
    assert_eq!(Tier::Iron2, Tier::from_str("iron 2").unwrap());
    assert_eq!(Tier::Iron3, Tier::from_str("IRON 3").unwrap());
    assert_eq!(Tier::Bronze1, Tier::from_str("Bronze 1").unwrap());
    assert_eq!(Tier::Bronze2, Tier::from_str("Bronze 2").unwrap());
    assert_eq!(Tier::Bronze3, Tier::from_str("BRONZE 3").unwrap());
    assert_eq!(Tier::Silver1, Tier::from_str("Silver 1").unwrap());
    assert_eq!(Tier::Silver2, Tier::from_str("silver 2").unwrap());
    assert_eq!(Tier::Silver3, Tier::from_str("SILVER 3").unwrap());
    assert_eq!(Tier::Gold1, Tier::from_str("Gold 1").unwrap());
    assert_eq!(Tier::Gold2, Tier::from_str("gold 2").unwrap());
    assert_eq!(Tier::Gold3, Tier::from_str("GOLD 3").unwrap());
    assert_eq!(Tier::Platinum1, Tier::from_str("Platinum 1").unwrap());
    assert_eq!(Tier::Platinum2, Tier::from_str("platinum 2").unwrap());
    assert_eq!(Tier::Platinum3, Tier::from_str("PLATINUM 3").unwrap());
    assert_eq!(Tier::Diamond1, Tier::from_str("Diamond 1").unwrap());
    assert_eq!(Tier::Diamond2, Tier::from_str("diamond 2").unwrap());
    assert_eq!(Tier::Diamond3, Tier::from_str("DIAMOND 3").unwrap());
    assert_eq!(Tier::Ascendant1, Tier::from_str("Ascendant 1").unwrap());
    assert_eq!(Tier::Ascendant2, Tier::from_str("ascendant 2").unwrap());
    assert_eq!(Tier::Ascendant3, Tier::from_str("ASCENDANT 3").unwrap());
    assert_eq!(Tier::Immortal1, Tier::from_str("Immortal 1").unwrap());
    assert_eq!(Tier::Immortal2, Tier::from_str("immortal 2").unwrap());
    assert_eq!(Tier::Immortal3, Tier::from_str("IMMORTAL 3").unwrap());
    assert_eq!(Tier::Radiant, Tier::from_str("RaDiAnT").unwrap());

    assert!(Tier::from_str("not exist").is_err());
    assert!(Tier::from_str("").is_err());
    assert!(Tier::from_str("    ").is_err());
}

#[test]
fn from_u8() {
    assert_eq!(Tier::Unrated, Tier::try_from(0_u8).unwrap());
    assert_eq!(Tier::Unknown1, Tier::try_from(1_u8).unwrap());
    assert_eq!(Tier::Unknown2, Tier::try_from(2_u8).unwrap());
    assert_eq!(Tier::Iron1, Tier::try_from(3_u8).unwrap());
    assert_eq!(Tier::Iron2, Tier::try_from(4_u8).unwrap());
    assert_eq!(Tier::Iron3, Tier::try_from(5_u8).unwrap());
    assert_eq!(Tier::Bronze1, Tier::try_from(6_u8).unwrap());
    assert_eq!(Tier::Bronze2, Tier::try_from(7_u8).unwrap());
    assert_eq!(Tier::Bronze3, Tier::try_from(8_u8).unwrap());
    assert_eq!(Tier::Silver1, Tier::try_from(9_u8).unwrap());
    assert_eq!(Tier::Silver2, Tier::try_from(10_u8).unwrap());
    assert_eq!(Tier::Silver3, Tier::try_from(11_u8).unwrap());
    assert_eq!(Tier::Gold1, Tier::try_from(12_u8).unwrap());
    assert_eq!(Tier::Gold2, Tier::try_from(13_u8).unwrap());
    assert_eq!(Tier::Gold3, Tier::try_from(14_u8).unwrap());
    assert_eq!(Tier::Platinum1, Tier::try_from(15_u8).unwrap());
    assert_eq!(Tier::Platinum2, Tier::try_from(16_u8).unwrap());
    assert_eq!(Tier::Platinum3, Tier::try_from(17_u8).unwrap());
    assert_eq!(Tier::Diamond1, Tier::try_from(18_u8).unwrap());
    assert_eq!(Tier::Diamond2, Tier::try_from(19_u8).unwrap());
    assert_eq!(Tier::Diamond3, Tier::try_from(20_u8).unwrap());
    assert_eq!(Tier::Ascendant1, Tier::try_from(21_u8).unwrap());
    assert_eq!(Tier::Ascendant2, Tier::try_from(22_u8).unwrap());
    assert_eq!(Tier::Ascendant3, Tier::try_from(23_u8).unwrap());
    assert_eq!(Tier::Immortal1, Tier::try_from(24_u8).unwrap());
    assert_eq!(Tier::Immortal2, Tier::try_from(25_u8).unwrap());
    assert_eq!(Tier::Immortal3, Tier::try_from(26_u8).unwrap());
    assert_eq!(Tier::Radiant, Tier::try_from(27_u8).unwrap());

    assert_eq!(Tier::Unrated, Tier::try_from(0_u128).unwrap());
    assert_eq!(Tier::Unknown1, Tier::try_from(1_u128).unwrap());
    assert_eq!(Tier::Unknown2, Tier::try_from(2_u128).unwrap());
    assert_eq!(Tier::Iron1, Tier::try_from(3_u128).unwrap());
    assert_eq!(Tier::Iron2, Tier::try_from(4_u128).unwrap());
    assert_eq!(Tier::Iron3, Tier::try_from(5_u128).unwrap());
    assert_eq!(Tier::Bronze1, Tier::try_from(6_u128).unwrap());
    assert_eq!(Tier::Bronze2, Tier::try_from(7_u128).unwrap());
    assert_eq!(Tier::Bronze3, Tier::try_from(8_u128).unwrap());
    assert_eq!(Tier::Silver1, Tier::try_from(9_u128).unwrap());
    assert_eq!(Tier::Silver2, Tier::try_from(10_u128).unwrap());
    assert_eq!(Tier::Silver3, Tier::try_from(11_u128).unwrap());
    assert_eq!(Tier::Gold1, Tier::try_from(12_u128).unwrap());
    assert_eq!(Tier::Gold2, Tier::try_from(13_u128).unwrap());
    assert_eq!(Tier::Gold3, Tier::try_from(14_u128).unwrap());
    assert_eq!(Tier::Platinum1, Tier::try_from(15_u128).unwrap());
    assert_eq!(Tier::Platinum2, Tier::try_from(16_u128).unwrap());
    assert_eq!(Tier::Platinum3, Tier::try_from(17_u128).unwrap());
    assert_eq!(Tier::Diamond1, Tier::try_from(18_u128).unwrap());
    assert_eq!(Tier::Diamond2, Tier::try_from(19_u128).unwrap());
    assert_eq!(Tier::Diamond3, Tier::try_from(20_u128).unwrap());
    assert_eq!(Tier::Ascendant1, Tier::try_from(21_u128).unwrap());
    assert_eq!(Tier::Ascendant2, Tier::try_from(22_u128).unwrap());
    assert_eq!(Tier::Ascendant3, Tier::try_from(23_u128).unwrap());
    assert_eq!(Tier::Immortal1, Tier::try_from(24_u128).unwrap());
    assert_eq!(Tier::Immortal2, Tier::try_from(25_u128).unwrap());
    assert_eq!(Tier::Immortal3, Tier::try_from(26_u128).unwrap());
    assert_eq!(Tier::Radiant, Tier::try_from(27_u128).unwrap());

    assert!(Tier::try_from(-1_i8).is_err());
    assert!(Tier::try_from(64_u8).is_err());
    assert!(Tier::try_from(u128::MAX).is_err());
}

#[test]
fn serialize() {
    assert_eq!("0", serde_json::to_string(&Tier::Unrated).unwrap());
    assert_eq!("1", serde_json::to_string(&Tier::Unknown1).unwrap());
    assert_eq!("2", serde_json::to_string(&Tier::Unknown2).unwrap());
    assert_eq!("3", serde_json::to_string(&Tier::Iron1).unwrap());
    assert_eq!("4", serde_json::to_string(&Tier::Iron2).unwrap());
    assert_eq!("5", serde_json::to_string(&Tier::Iron3).unwrap());
    assert_eq!("6", serde_json::to_string(&Tier::Bronze1).unwrap());
    assert_eq!("7", serde_json::to_string(&Tier::Bronze2).unwrap());
    assert_eq!("8", serde_json::to_string(&Tier::Bronze3).unwrap());
    assert_eq!("9", serde_json::to_string(&Tier::Silver1).unwrap());
    assert_eq!("10", serde_json::to_string(&Tier::Silver2).unwrap());
    assert_eq!("11", serde_json::to_string(&Tier::Silver3).unwrap());
    assert_eq!("12", serde_json::to_string(&Tier::Gold1).unwrap());
    assert_eq!("13", serde_json::to_string(&Tier::Gold2).unwrap());
    assert_eq!("14", serde_json::to_string(&Tier::Gold3).unwrap());
    assert_eq!("15", serde_json::to_string(&Tier::Platinum1).unwrap());
    assert_eq!("16", serde_json::to_string(&Tier::Platinum2).unwrap());
    assert_eq!("17", serde_json::to_string(&Tier::Platinum3).unwrap());
    assert_eq!("18", serde_json::to_string(&Tier::Diamond1).unwrap());
    assert_eq!("19", serde_json::to_string(&Tier::Diamond2).unwrap());
    assert_eq!("20", serde_json::to_string(&Tier::Diamond3).unwrap());
    assert_eq!("21", serde_json::to_string(&Tier::Ascendant1).unwrap());
    assert_eq!("22", serde_json::to_string(&Tier::Ascendant2).unwrap());
    assert_eq!("23", serde_json::to_string(&Tier::Ascendant3).unwrap());
    assert_eq!("24", serde_json::to_string(&Tier::Immortal1).unwrap());
    assert_eq!("25", serde_json::to_string(&Tier::Immortal2).unwrap());
    assert_eq!("26", serde_json::to_string(&Tier::Immortal3).unwrap());
    assert_eq!("27", serde_json::to_string(&Tier::Radiant).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Tier::Unrated, serde_json::from_str("0").unwrap());
    assert_eq!(Tier::Unknown1, serde_json::from_str("1").unwrap());
    assert_eq!(Tier::Unknown2, serde_json::from_str("2").unwrap());
    assert_eq!(Tier::Iron1, serde_json::from_str("3").unwrap());
    assert_eq!(Tier::Iron2, serde_json::from_str("4").unwrap());
    assert_eq!(Tier::Iron3, serde_json::from_str("5").unwrap());
    assert_eq!(Tier::Bronze1, serde_json::from_str("6").unwrap());
    assert_eq!(Tier::Bronze2, serde_json::from_str("7").unwrap());
    assert_eq!(Tier::Bronze3, serde_json::from_str("8").unwrap());
    assert_eq!(Tier::Silver1, serde_json::from_str("9").unwrap());
    assert_eq!(Tier::Silver2, serde_json::from_str("10").unwrap());
    assert_eq!(Tier::Silver3, serde_json::from_str("11").unwrap());
    assert_eq!(Tier::Gold1, serde_json::from_str("12").unwrap());
    assert_eq!(Tier::Gold2, serde_json::from_str("13").unwrap());
    assert_eq!(Tier::Gold3, serde_json::from_str("14").unwrap());
    assert_eq!(Tier::Platinum1, serde_json::from_str("15").unwrap());
    assert_eq!(Tier::Platinum2, serde_json::from_str("16").unwrap());
    assert_eq!(Tier::Platinum3, serde_json::from_str("17").unwrap());
    assert_eq!(Tier::Diamond1, serde_json::from_str("18").unwrap());
    assert_eq!(Tier::Diamond2, serde_json::from_str("19").unwrap());
    assert_eq!(Tier::Diamond3, serde_json::from_str("20").unwrap());
    assert_eq!(Tier::Ascendant1, serde_json::from_str("21").unwrap());
    assert_eq!(Tier::Ascendant2, serde_json::from_str("22").unwrap());
    assert_eq!(Tier::Ascendant3, serde_json::from_str("23").unwrap());
    assert_eq!(Tier::Immortal1, serde_json::from_str("24").unwrap());
    assert_eq!(Tier::Immortal2, serde_json::from_str("25").unwrap());
    assert_eq!(Tier::Immortal3, serde_json::from_str("26").unwrap());
    assert_eq!(Tier::Radiant, serde_json::from_str("27").unwrap());

    assert!(serde_json::from_str::<Tier>("-1").is_err());
    assert!(serde_json::from_str::<Tier>("64").is_err());
    assert!(serde_json::from_str::<Tier>("340282366920938463463374607431768211455").is_err());
}
