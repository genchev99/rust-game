extern crate serde_json;
use std::fs::File;

pub fn load_json(path: String) {
    return ::serde_json::from_reader(File::open(path)?)?;
}
