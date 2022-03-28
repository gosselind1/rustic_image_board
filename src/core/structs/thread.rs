use std::string::String;
use std::vec::Vec;

pub(crate) struct Thread{
    name: String,
    parent: u64,  // thread ID #
    children: std::vec::Vec<u64>,  // thread IDs
    locked: bool,
    deleted: bool, // could be considered redundant with post's deleted, but this will simplify things
}

impl Thread {
    pub fn new(name: String, parent: u64) -> Thread {
        let new_thread = Thread {
            name,
            parent,
            children: Vec::new(),
            locked: false,
            deleted: false
        };

        return new_thread;
    }

    pub fn get_name(self) -> String {
        return self.name;
    }

    pub fn get_parent(self: &Self) -> u64 {
        return self.parent;
    }

    pub fn get_children(self) -> Vec<u64> {
        return self.children;
    }

    pub fn get_lock(self) -> bool {
        return self.locked;
    }

    pub fn get_deleted(self) -> bool {
        return self.deleted;
    }

    pub fn modify_name(mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn add_child(mut self, child: u64) {
        self.children.push(child);
    }

    pub fn lock(mut self) {
        self.locked = true;
    }

    pub fn unlock(mut self) {
        self.locked = false;
    }

    pub fn delete(mut self) {
        self.deleted = true;
    }

    pub fn undelete(mut self) {
        self.deleted = false;
    }
}