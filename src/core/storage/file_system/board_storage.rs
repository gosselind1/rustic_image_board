use super::common;
use super::structs;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Error;
use std::io::{Read, Write};
use std::path::Path;

const CONFIG_FILE: &str = "config.txt";

pub(crate) fn initialize(board_path: &Path) -> Result<(), E> {
    // using an input path from site_storage.initialize()
    // 1. check for directory
    // 2. create it if it doesn't exist
    // 3. check for config file
    // 4. create if doesn't exist
    // 5. read config file
    // 6. validate config file
    let config_path: &Path = &*board_path.join(CONFIG_FILE);

    if !board_path.exists() {
        create_dir(board_path)?;
    }
    if !config_path.exists() {
        generate_config(config_path)?;
    }
    //

    return Ok(());
}

pub(self) fn generate_config(config_path: &Path) -> Result<()> {
    // based on architecture notes, we should have the following values in the config file:
    // 1. Board description (board name is handled by folder name)
    // 2. Total number of active threads
    // 3. Total number of archived threads
    let mut config_file: File = File::create(&config_path)?;

    config_file.write(b"description: This is the default board description.\r\n")?;
    config_file.write(b"active_count: 16\r\n")?;
    config_file.write(b"archive_count: 8\r\n")?;
    config_file.sync_all()?;

    return Ok(());
}

pub(self) fn read_config(config_path: &Path) -> Result<HashMap<&str, &str>, E> {
    let data: HashMap<&str, &str> = common::read_config(config_path)?;
    // A better scheme might filter out unused keys, and or raise errors?
    // Should this be structure agnostic at this point, or is it acceptable to return the raw struct?
    // I'll revisit this next time I work on this...
    return Ok(data);
}
