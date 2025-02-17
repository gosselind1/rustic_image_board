use file_system::*;  // should be generic enough to be swapped w/o issue.

pub fn connect(config) {
    ...;
}

pub fn read_board(config, board) {
    ...;
}

pub fn read_thread(config, board, thread) {
    ...;
}

pub fn read_post(config, board, post) {
    ...;
}

pub fn disconnect(config) {
    // must guarantee a flush!
    ...;
}

pub fn write_board(config, board) {
    ...;
}

pub fn write_thread(config, board, thread) {
    ...;
}

pub fn write_post(config, board, post) {
    ...;
}
