use std::str::FromStr;

use rust_unofficial_valorant_api::types::Character;

#[test]
fn to_str() {
    assert_eq!("Brimstone", Character::Brimstone.to_str());
    assert_eq!("Viper", Character::Viper.to_str());
    assert_eq!("Omen", Character::Omen.to_str());
    assert_eq!("Killjoy", Character::Killjoy.to_str());
    assert_eq!("Cypher", Character::Cypher.to_str());
    assert_eq!("Sova", Character::Sova.to_str());
    assert_eq!("Sage", Character::Sage.to_str());
    assert_eq!("Phoenix", Character::Phoenix.to_str());
    assert_eq!("Jett", Character::Jett.to_str());
    assert_eq!("Reyna", Character::Reyna.to_str());
    assert_eq!("Raze", Character::Raze.to_str());
    assert_eq!("Breach", Character::Breach.to_str());
    assert_eq!("Skye", Character::Skye.to_str());
    assert_eq!("Yoru", Character::Yoru.to_str());
    assert_eq!("Astra", Character::Astra.to_str());
    assert_eq!("KAY/O", Character::KAYO.to_str());
    assert_eq!("Chamber", Character::Chamber.to_str());
    assert_eq!("Neon", Character::Neon.to_str());
    assert_eq!("Fade", Character::Fade.to_str());
    assert_eq!("Harbor", Character::Harbor.to_str());
    assert_eq!("Gekko", Character::Gekko.to_str());
    assert_eq!("Deadlock", Character::Deadlock.to_str());
    assert_eq!("Iso", Character::Iso.to_str());
}

#[test]
fn from_str() {
    assert_eq!(Character::Brimstone, Character::from_str("Brimstone").unwrap());
    assert_eq!(Character::Viper, Character::from_str("Viper").unwrap());
    assert_eq!(Character::Omen, Character::from_str("Omen").unwrap());
    assert_eq!(Character::Killjoy, Character::from_str("Killjoy").unwrap());
    assert_eq!(Character::Cypher, Character::from_str("Cypher").unwrap());
    assert_eq!(Character::Sova, Character::from_str("Sova").unwrap());
    assert_eq!(Character::Sage, Character::from_str("Sage").unwrap());
    assert_eq!(Character::Phoenix, Character::from_str("Phoenix").unwrap());
    assert_eq!(Character::Jett, Character::from_str("Jett").unwrap());
    assert_eq!(Character::Reyna, Character::from_str("Reyna").unwrap());
    assert_eq!(Character::Raze, Character::from_str("Raze").unwrap());
    assert_eq!(Character::Breach, Character::from_str("Breach").unwrap());
    assert_eq!(Character::Skye, Character::from_str("Skye").unwrap());
    assert_eq!(Character::Yoru, Character::from_str("Yoru").unwrap());
    assert_eq!(Character::Astra, Character::from_str("Astra").unwrap());
    assert_eq!(Character::KAYO, Character::from_str("KAY/O").unwrap());
    assert_eq!(Character::Chamber, Character::from_str("Chamber").unwrap());
    assert_eq!(Character::Neon, Character::from_str("Neon").unwrap());
    assert_eq!(Character::Fade, Character::from_str("Fade").unwrap());
    assert_eq!(Character::Harbor, Character::from_str("Harbor").unwrap());
    assert_eq!(Character::Gekko, Character::from_str("Gekko").unwrap());
    assert_eq!(Character::Deadlock, Character::from_str("Deadlock").unwrap());
    assert_eq!(Character::Iso, Character::from_str("Iso").unwrap());

    assert_eq!(Character::Brimstone, Character::from_str("brimstone").unwrap());
    assert_eq!(Character::Viper, Character::from_str("viper").unwrap());
    assert_eq!(Character::Omen, Character::from_str("OMEN").unwrap());
    assert_eq!(Character::Killjoy, Character::from_str("KILLJOY").unwrap());
    assert_eq!(Character::Cypher, Character::from_str("cypher").unwrap());
    assert_eq!(Character::Sova, Character::from_str("sova").unwrap());
    assert_eq!(Character::Sage, Character::from_str("SAGE").unwrap());
    assert_eq!(Character::Phoenix, Character::from_str("PHOENIX").unwrap());
    assert_eq!(Character::Jett, Character::from_str("jett").unwrap());
    assert_eq!(Character::Reyna, Character::from_str("reyna").unwrap());
    assert_eq!(Character::Raze, Character::from_str("RAZE").unwrap());
    assert_eq!(Character::Breach, Character::from_str("BREACH").unwrap());
    assert_eq!(Character::Skye, Character::from_str("skye").unwrap());
    assert_eq!(Character::Yoru, Character::from_str("yoru").unwrap());
    assert_eq!(Character::Astra, Character::from_str("ASTRA").unwrap());
    assert_eq!(Character::KAYO, Character::from_str("kay/o").unwrap());
    assert_eq!(Character::Chamber, Character::from_str("CHAMBER").unwrap());
    assert_eq!(Character::Neon, Character::from_str("neon").unwrap());
    assert_eq!(Character::Fade, Character::from_str("FADE").unwrap());
    assert_eq!(Character::Harbor, Character::from_str("harbor").unwrap());
    assert_eq!(Character::Gekko, Character::from_str("GEKKO").unwrap());
    assert_eq!(Character::Deadlock, Character::from_str("deadlock").unwrap());
    assert_eq!(Character::Iso, Character::from_str("ISO").unwrap());

    assert!(Character::from_str("not exist").is_err());
    assert!(Character::from_str("").is_err());
    assert!(Character::from_str("    ").is_err());
}

#[test]
fn serialize() {
    assert_eq!("\"Brimstone\"", serde_json::to_string(&Character::Brimstone).unwrap());
    assert_eq!("\"Viper\"", serde_json::to_string(&Character::Viper).unwrap());
    assert_eq!("\"Omen\"", serde_json::to_string(&Character::Omen).unwrap());
    assert_eq!("\"Killjoy\"", serde_json::to_string(&Character::Killjoy).unwrap());
    assert_eq!("\"Cypher\"", serde_json::to_string(&Character::Cypher).unwrap());
    assert_eq!("\"Sova\"", serde_json::to_string(&Character::Sova).unwrap());
    assert_eq!("\"Sage\"", serde_json::to_string(&Character::Sage).unwrap());
    assert_eq!("\"Phoenix\"", serde_json::to_string(&Character::Phoenix).unwrap());
    assert_eq!("\"Jett\"", serde_json::to_string(&Character::Jett).unwrap());
    assert_eq!("\"Reyna\"", serde_json::to_string(&Character::Reyna).unwrap());
    assert_eq!("\"Raze\"", serde_json::to_string(&Character::Raze).unwrap());
    assert_eq!("\"Breach\"", serde_json::to_string(&Character::Breach).unwrap());
    assert_eq!("\"Skye\"", serde_json::to_string(&Character::Skye).unwrap());
    assert_eq!("\"Yoru\"", serde_json::to_string(&Character::Yoru).unwrap());
    assert_eq!("\"Astra\"", serde_json::to_string(&Character::Astra).unwrap());
    assert_eq!("\"KAY/O\"", serde_json::to_string(&Character::KAYO).unwrap());
    assert_eq!("\"Chamber\"", serde_json::to_string(&Character::Chamber).unwrap());
    assert_eq!("\"Neon\"", serde_json::to_string(&Character::Neon).unwrap());
    assert_eq!("\"Fade\"", serde_json::to_string(&Character::Fade).unwrap());
    assert_eq!("\"Harbor\"", serde_json::to_string(&Character::Harbor).unwrap());
    assert_eq!("\"Gekko\"", serde_json::to_string(&Character::Gekko).unwrap());
    assert_eq!("\"Deadlock\"", serde_json::to_string(&Character::Deadlock).unwrap());
    assert_eq!("\"Iso\"", serde_json::to_string(&Character::Iso).unwrap());
}

#[test]
fn deserialize() {
    assert_eq!(Character::Brimstone, serde_json::from_str("\"Brimstone\"").unwrap());
    assert_eq!(Character::Viper, serde_json::from_str("\"Viper\"").unwrap());
    assert_eq!(Character::Omen, serde_json::from_str("\"Omen\"").unwrap());
    assert_eq!(Character::Killjoy, serde_json::from_str("\"Killjoy\"").unwrap());
    assert_eq!(Character::Cypher, serde_json::from_str("\"Cypher\"").unwrap());
    assert_eq!(Character::Sova, serde_json::from_str("\"Sova\"").unwrap());
    assert_eq!(Character::Sage, serde_json::from_str("\"Sage\"").unwrap());
    assert_eq!(Character::Phoenix, serde_json::from_str("\"Phoenix\"").unwrap());
    assert_eq!(Character::Jett, serde_json::from_str("\"Jett\"").unwrap());
    assert_eq!(Character::Reyna, serde_json::from_str("\"Reyna\"").unwrap());
    assert_eq!(Character::Raze, serde_json::from_str("\"Raze\"").unwrap());
    assert_eq!(Character::Breach, serde_json::from_str("\"Breach\"").unwrap());
    assert_eq!(Character::Skye, serde_json::from_str("\"Skye\"").unwrap());
    assert_eq!(Character::Yoru, serde_json::from_str("\"Yoru\"").unwrap());
    assert_eq!(Character::Astra, serde_json::from_str("\"Astra\"").unwrap());
    assert_eq!(Character::KAYO, serde_json::from_str("\"KAY/O\"").unwrap());
    assert_eq!(Character::Chamber, serde_json::from_str("\"Chamber\"").unwrap());
    assert_eq!(Character::Neon, serde_json::from_str("\"Neon\"").unwrap());
    assert_eq!(Character::Fade, serde_json::from_str("\"Fade\"").unwrap());
    assert_eq!(Character::Harbor, serde_json::from_str("\"Harbor\"").unwrap());
    assert_eq!(Character::Gekko, serde_json::from_str("\"Gekko\"").unwrap());
    assert_eq!(Character::Deadlock, serde_json::from_str("\"Deadlock\"").unwrap());
    assert_eq!(Character::Iso, serde_json::from_str("\"Iso\"").unwrap());

    assert_eq!(Character::Brimstone, serde_json::from_str("\"brimstone\"").unwrap());
    assert_eq!(Character::Viper, serde_json::from_str("\"viper\"").unwrap());
    assert_eq!(Character::Omen, serde_json::from_str("\"OMEN\"").unwrap());
    assert_eq!(Character::Killjoy, serde_json::from_str("\"KILLJOY\"").unwrap());
    assert_eq!(Character::Cypher, serde_json::from_str("\"cypher\"").unwrap());
    assert_eq!(Character::Sova, serde_json::from_str("\"sova\"").unwrap());
    assert_eq!(Character::Sage, serde_json::from_str("\"SAGE\"").unwrap());
    assert_eq!(Character::Phoenix, serde_json::from_str("\"PHOENIX\"").unwrap());
    assert_eq!(Character::Jett, serde_json::from_str("\"jett\"").unwrap());
    assert_eq!(Character::Reyna, serde_json::from_str("\"reyna\"").unwrap());
    assert_eq!(Character::Raze, serde_json::from_str("\"RAZE\"").unwrap());
    assert_eq!(Character::Breach, serde_json::from_str("\"BREACH\"").unwrap());
    assert_eq!(Character::Skye, serde_json::from_str("\"skye\"").unwrap());
    assert_eq!(Character::Yoru, serde_json::from_str("\"yoru\"").unwrap());
    assert_eq!(Character::Astra, serde_json::from_str("\"ASTRA\"").unwrap());
    assert_eq!(Character::KAYO, serde_json::from_str("\"kay/o\"").unwrap());
    assert_eq!(Character::Chamber, serde_json::from_str("\"CHAMBER\"").unwrap());
    assert_eq!(Character::Neon, serde_json::from_str("\"neon\"").unwrap());
    assert_eq!(Character::Fade, serde_json::from_str("\"FADE\"").unwrap());
    assert_eq!(Character::Harbor, serde_json::from_str("\"harbor\"").unwrap());
    assert_eq!(Character::Gekko, serde_json::from_str("\"GEKKO\"").unwrap());
    assert_eq!(Character::Deadlock, serde_json::from_str("\"deadlock\"").unwrap());
    assert_eq!(Character::Iso, serde_json::from_str("\"ISO\"").unwrap());

    assert!(serde_json::from_str::<Character>("\"not exist\"").is_err());
    assert!(serde_json::from_str::<Character>("\"\"").is_err());
    assert!(serde_json::from_str::<Character>("\"    \"").is_err());
}
