use std::fs;

const RESOURCES_DIR: &'static str = "resources";

pub fn read_resource(file: &'static str) -> String {
    let path = format!("{0}/{RESOURCES_DIR}/{file}", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(path).unwrap()
}
