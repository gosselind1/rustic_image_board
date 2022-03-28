use std::time::{Instant};
use std::string::String;
use bytes::Bytes;

pub(crate) struct Post {
    owner: String,
    text: String,
    attachment: Bytes,
    created: Instant,   // unix milli
    modified: Instant,  // unix milli
    deleted: bool,
    parent: u64    // thread ID #
}

impl Post {
        pub fn new(owner: String, text: String, attachment: Bytes, parent: u64) -> Post {
        let created: Instant = Instant::now();
        let modified: Instant = created;

        let new_post: Post = Post {
            owner,
            text,
            attachment,
            created,
            modified,
            deleted: false,
            parent
        };

        return new_post;
    }

    pub fn get_owner(self) -> String {
        return self.owner;
    }

    pub fn get_text(self) -> String {
        return self.text;
    }

    pub fn get_attachment(self) -> Bytes {
        return self.attachment;
    }

    pub fn get_created(self) -> Instant {
        return self.created;
    }

    pub fn get_modified(self) -> Instant {
        return self.modified;
    }

    pub fn get_deleted(self) -> bool {
        return self.deleted;
    }

    pub fn get_parent(self) -> u64 {
        return self.parent;
    }

    pub fn modify_owner(mut self, new_owner: String) {
        self.modification();
        self.owner = new_owner;
    }

    pub fn modify_text(mut self, new_text: String) {
        self.modification();
        self.text = new_text;
    }

    pub fn remove_attachment(mut self) {
        self.modification();
        self.attachment = Bytes::new();
    }

    pub fn delete(mut self) {
        self.modification();
        self.deleted = true;
    }

    pub fn undelete(mut self) {
        self.modification();
        self.deleted = false;
    }

    fn modification(&mut self) {
        let modified: Instant = Instant::now();
        self.modified = modified;
    }
}