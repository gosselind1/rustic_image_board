use bytes::Bytes;
use std::string::String;
use std::time::Instant;

pub(crate) struct Post {
    owner: String,
    text: String,
    attachment: Bytes,
    created: Instant,  // unix milli
    modified: Instant, // unix milli
    deleted: bool,
    parent: u64, // thread ID #
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
            parent,
        };

        return new_post;
    }

    pub fn get_owner(&self) -> &String {
        return &self.owner;
    }

    pub fn get_text(&self) -> &String {
        return &self.text;
    }

    pub fn get_attachment(&self) -> &Bytes {
        return &self.attachment;
    }

    pub fn get_created(&self) -> &Instant {
        return &self.created;
    }

    pub fn get_modified(&self) -> &Instant {
        return &self.modified;
    }

    pub fn get_deleted(&self) -> bool {
        return self.deleted;
    }

    pub fn get_parent(&self) -> u64 {
        return self.parent;
    }

    pub fn modify_owner(&mut self, new_owner: String) {
        // Does this functionality make sense?
        self.modification();
        self.owner = new_owner;
    }

    pub fn modify_text(&mut self, new_text: String) {
        self.modification();
        self.text = new_text;
    }

    pub fn remove_attachment(&mut self) {
        self.modification();
        self.attachment = Bytes::new();
    }

    pub fn delete(&mut self) {
        self.modification();
        self.deleted = true;
    }

    pub fn undelete(&mut self) {
        self.modification();
        self.deleted = false;
    }

    fn modification(&mut self) {
        let modified: Instant = Instant::now();
        self.modified = modified;
    }
}

#[cfg(test)]
mod tests {
    use crate::core::structs;

    use super::*;
    const OWNER: &str = "me";
    const TEXT: &str = "Yeeting on em";
    const FILE: Bytes = Bytes::from_static(b"file.png");

    fn struct_init() -> Post {
        let a_post = Post::new(OWNER.to_string(), TEXT.to_string(), FILE, 0);
        return a_post;
    }

    #[test]
    fn test_init() {
        struct_init();
    }

    #[test]
    fn test_attributes_direct() {
        let a = struct_init();
        let start = Instant::now();

        assert_eq!(a.owner, OWNER);
        assert_eq!(a.text, TEXT);
        assert_eq!(a.attachment, FILE);
        let delta = Instant::now()
            .duration_since(start)
            .abs_diff(a.created.elapsed()); // I shouldn't need abs here???
        assert!(delta.as_micros() < 1000);
        assert_eq!(a.created, a.modified);
        assert!(!a.deleted);
        assert_eq!(a.parent, 0);
    }

    #[test]
    fn test_modification() {
        let a = &mut struct_init();
        a.modification();
        assert!(a.get_created().lt(a.get_modified()))
    }

    #[test]
    fn test_get_owner() {
        let a = &struct_init();
        assert_eq!(a.owner, *a.get_owner());
    }

    #[test]
    fn test_get_text() {
        let a = &struct_init();
        assert_eq!(a.text, *a.get_text());
    }

    #[test]
    fn test_get_attachment() {
        let a = &struct_init();
        assert_eq!(a.attachment, *a.get_attachment());
    }

    #[test]
    fn test_get_created() {
        let a = &struct_init();
        assert_eq!(a.created, *a.get_created());
    }

    #[test]
    fn test_get_modified() {
        let a = &struct_init();
        assert_eq!(a.modified, *a.get_modified());
    }

    #[test]
    fn test_get_deleted() {
        let a = &struct_init();
        assert_eq!(a.deleted, a.get_deleted());
    }

    #[test]
    fn test_get_parent() {
        let a = &struct_init();
        assert_eq!(a.parent, a.get_parent());
    }

    #[test]
    fn test_modify_owner() {
        let a = &mut struct_init();
        let new_owner = &"you";

        a.modify_owner(new_owner.to_string());
        assert_eq!(a.get_owner(), new_owner);
        assert!(a.get_created().lt(a.get_modified()))
    }

    #[test]
    fn test_modify_text() {
        let a = &mut struct_init();
        let new_text = &"yooted below us";

        a.modify_text(new_text.to_string());
        assert_eq!(a.get_text(), new_text);
        assert!(a.get_created().lt(a.get_modified()))
    }

    #[test]
    fn test_remove_attachment() {
        let a = &mut struct_init();

        a.remove_attachment();
        assert_eq!(*a.get_attachment(), Bytes::new());
        assert!(a.get_created().lt(a.get_modified()));
    }

    #[test]
    fn test_delete() {
        let a = &mut struct_init();

        a.delete();
        assert!(a.get_deleted());
        assert!(a.get_created().lt(a.get_modified()));
    }

    #[test]
    fn test_undelete() {
        let a = &mut struct_init();
        a.deleted = true;

        a.undelete();
        assert!(!a.get_deleted());
        assert!(a.get_created().lt(a.get_modified()));
    }
}
