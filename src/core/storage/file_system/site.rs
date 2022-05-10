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

pub fn initialize(root: &Path){
    // 1. check for config file in current directory, ie: ./
    // 2. create a basic configuration file is none exists
    // 3. read configuration file
    // 4. validate that storage is setup according to configuration file
    // 5. Fill in any missing items
    // 6. initialize sub-items (so specific boards in this case)
}