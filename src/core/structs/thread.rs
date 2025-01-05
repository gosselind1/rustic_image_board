use std::string::String;
use std::vec::Vec;

pub(crate) struct Thread {
    name: String,
    children: Vec<u64>, // thread IDs
    locked: bool,
    deleted: bool, // could be considered redundant with post's deleted, but this will simplify things
}

impl Thread {
    pub fn new(name: String, parent: u64) -> Thread {
        let new_thread = Thread {
            name,
            children: Vec::from([parent]),
            locked: false,
            deleted: false,
        };

        return new_thread;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_parent(&self) -> u64 {
        return *self.children.get(0).unwrap();
    }

    pub fn get_children(&self) -> &Vec<u64> {
        return &self.children;
    }

    pub fn get_locked(&self) -> bool {
        return self.locked;
    }

    pub fn get_deleted(&self) -> bool {
        return self.deleted;
    }

    pub fn modify_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn add_child(&mut self, child: u64) {
        self.children.push(child); // This allows a child post to be older than the thread.
                                   // More so, this function technically represents a
                                   // potential target for DoS attacks because our vectors
                                   // are unbounded, re-allocation will eventually occur,
                                   // giving a nice lovely target surface for re-allocation
                                   // brute-forcing. Of course, this *hopefully* won't be
                                   // an issue with a proper rate limiter in higher levels of
                                   // the software.
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }

    pub fn delete(&mut self) {
        self.deleted = true;
    }

    pub fn undelete(&mut self) {
        self.deleted = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "test post please ignore";
    const PARENT: u64 = 42069;

    fn struct_init() -> Thread {
        let a_thread = Thread::new(NAME.to_string(), PARENT);
        return a_thread;
    }

    #[test]
    fn test_init() {
        struct_init();
    }

    #[test]
    fn test_attributes_direct() {
        let a = struct_init();

        assert_eq!(a.name, NAME);
        assert_eq!(a.children.len(), 1);
        assert_eq!(*a.children.get(0).unwrap(), PARENT);
        assert!(!a.locked);
        assert!(!a.deleted);
    }

    #[test]
    fn test_get_name() {
        let a = struct_init();
        assert_eq!(a.get_name(), NAME)
    }

    #[test]
    fn test_get_parent() {
        let a = struct_init();
        assert_eq!(a.get_parent(), PARENT)
    }

    #[test]
    fn test_get_children() {
        let a = struct_init();
        assert_eq!(a.get_children().len(), 1);
        assert_eq!(*a.get_children().get(0).unwrap(), PARENT);
    }

    #[test]
    fn test_get_locked() {
        let a = struct_init();
        assert!(!a.get_locked());
    }

    #[test]
    fn test_get_deleted() {
        let a = struct_init();
        assert!(!a.get_deleted());
    }

    #[test]
    fn test_modify_name() {
        let mut a = struct_init();
        let new_name = &"Not ignored real post";

        a.modify_name(new_name.to_string());
        assert_eq!(a.get_name(), new_name);
    }

    #[test]
    fn test_add_child() {
        let mut a = struct_init();
        let size: usize = 42;
        assert_eq!(a.get_children().len(), 1);
        assert_eq!(*a.get_children().get(0).unwrap(), PARENT);

        for i in 1..size {
            a.add_child(PARENT + i as u64);
            assert_eq!(a.get_children().len(), i + 1)
        }

        for i in 0..size {
            assert_eq!(*a.get_children().get(i).unwrap(), PARENT + i as u64);
        }
    }

    #[test]
    fn test_lock() {
        let mut a = struct_init();
        a.lock();
        assert!(a.get_locked());
    }

    #[test]
    fn test_unlock() {
        let mut a = struct_init();
        a.locked = true;
        a.unlock();
        assert!(!a.get_locked());
    }

    #[test]
    fn test_delete() {
        let mut a = struct_init();
        a.delete();
        assert!(a.get_deleted());
    }

    #[test]
    fn test_undelete() {
        let mut a = struct_init();
        a.deleted = true;
        a.undelete();
        assert!(!a.get_deleted());
    }
}
