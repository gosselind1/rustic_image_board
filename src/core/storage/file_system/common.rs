// This module contains commonly used functions across the file_system storage scheme
use std::path::Path;
use std::fs::File;
use std::io::{Read};
use std::collections::HashMap;

pub(crate) fn read_config(file_path: &Path) -> Result<HashMap<&str, &str>, E> {
    let mut file_descriptor: File = File::open(file_path)?;
    let mut lines: &String;
    let mut keyed_contents: HashMap<&str, &str> = HashMap::new();

    file_descriptor.read_to_string(&mut lines);

    for line in lines.split("\r\n") {
        // this will crash if a config file is malformed.
        let split = line.split_once(": ").unwrap();
        keyed_contents.insert(split.0, split.1);
    }

    return Ok(keyed_contents);
}
