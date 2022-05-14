use std::collections::HashMap;
// This file should contain the ``site'' data. This should be more or less the root of the website's
// data. The main goal here should be taking an input folder, and then validating RW access +
// determining if the basic file structure exists.
// In the case of our current config, we want a setup where we read a generic configuration file,
// which we generate if it doesn't exist.
// The configuration file should define the boards, from which the post and threads folders reside in
// that is to say:
// |
// |-boards
// |-|-board_a
// |-|-threads
// |-|-|-thread_1.data
// |-|-posts
// |-|-|-post_1.text
// |-|-|-post_1.attachment
use std::path::Path;
use std::fs::{File, create_dir};
use std::io::{Read, Write};
use super::board_storage;
use super::common;

const ROOT_DIR: &str = "./";
const CONFIG_FILE: &str = "config.txt";
const BOARD_DIR: &str = "boards/";

pub fn initialize() -> Result<(), E> {
    // 1. check for config file in current directory, ie: ./
    // 2. create a basic configuration file is none exists
    // 3. read configuration file
    // 4. validate that storage is setup according to configuration file
    // 5. Fill in any missing items
    // 6. initialize sub-items (so specific boards in this case)
    let config_path: &Path = Path::new(&format!("{}{}", ROOT_DIR, CONFIG_FILE));
    let boards_root: &Path = Path::new(&format!("{}{}", ROOT_DIR, BOARD_DIR));
    let board_list: Vec<&str>;
    let mut board_path: &Path;

    if !config_path.exists() {
        generate_config(config_path)?;
    }
    board_list = read_config(config_path)?;

    if !boards_root.exists() {
        create_dir(boards_root)?;
    }

    for board in board_list {
        board_path = &*boards_root.join(board);
        board_storage::initialize(board_path)?;
    }

    return Ok(());
}

pub(self) fn generate_config(file_path: &Path) -> Result<(), E> {
    // The default configuration file should just define the list of boards?
    // Leave the default board configuration to the specific submodule
    let mut config_file: File = File::create(&file_path)?;

    config_file.write(b"boards: \xCE\xB1, test\r\n")?;
    config_file.sync_all()?;

    return Ok(());
}

pub(self) fn read_config(file_path: &Path) -> Result<Vec<&str>, E> {
    let data: HashMap<&str, &str> = common::read_config(file_path)?;

    let mut boards: &str = *data.get("boards")?;
    boards = &*boards.replace(" ", "");

    return Ok(boards.split(",").collect());
}
