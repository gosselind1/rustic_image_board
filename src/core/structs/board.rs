use std::collections::HashSet;
use std::string::String;

pub(crate) struct Board {
    name: String,
    description: String,
    active: Vec<u64>,     // active threads, as parent post IDs
    archive: Vec<u64>,    // archive threads, as parent post IDs
    sticky: HashSet<u64>, // should contain a subset of threads
    count: u64,           // running total of all posts
}

impl Board {
    pub fn new(name: String, description: String) -> Board {
        let board = Board {
            name,
            description,
            active: Vec::new(),
            archive: Vec::new(),
            sticky: HashSet::new(),
            count: 0,
        };

        return board;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_description(&self) -> &String {
        return &self.description;
    }

    pub fn get_active(&self) -> &Vec<u64> {
        return &self.active;
    }

    pub fn get_archive(&self) -> &Vec<u64> {
        return &self.archive;
    }

    pub fn get_sticky(&self) -> &HashSet<u64> {
        return &self.sticky;
    }

    pub fn get_count(&self) -> u64 {
        return self.count;
    }

    pub fn modify_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn modify_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn add_active(&mut self, parent: u64) {
        self.active.push(parent);
    }

    pub fn add_archive(&mut self, parent: u64) {
        self.archive.push(parent);
    }

    pub fn add_sticky(&mut self, parent: u64) -> bool {
        return self.sticky.insert(parent);
    }

    pub fn increment_count(&mut self) -> u64 {
        self.count += 1;
        return self.count;
    }

    pub fn remove_active(&mut self, parent: u64) -> bool {
        return Board::remove_element(&mut self.active, parent);
    }

    pub fn remove_archive(&mut self, parent: u64) -> bool {
        return Board::remove_element(&mut self.archive, parent);
    }

    fn remove_element(vec: &mut Vec<u64>, parent: u64) -> bool {
        for index in 0..vec.len() {
            if *vec.get(index).unwrap() == parent {
                vec.remove(index);
                return true;
            }
        }
        return false;
    }

    pub fn remove_sticky(&mut self, parent: u64) -> bool {
        return self.sticky.remove(&parent);
    }

    pub fn pop_active(&mut self) -> Option<u64> {
        return self.active.pop();
    }

    pub fn pop_archive(&mut self) -> Option<u64> {
        return self.archive.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "Test Board";
    const DESC: &str = "This is a test board.";

    fn struct_init() -> Board {
        let a_board = Board::new(NAME.to_string(), DESC.to_string());
        return a_board;
    }

    #[test]
    fn test_init() {
        struct_init();
    }

    #[test]
    fn test_attributes_direct() {
        let a = struct_init();

        assert_eq!(a.name, NAME);
        assert_eq!(a.description, DESC);
        assert_eq!(a.active.len(), 0);
        assert_eq!(a.archive.len(), 0);
        assert_eq!(a.sticky.len(), 0);
        assert_eq!(a.count, 0);
    }

    #[test]
    fn test_get_name() {
        let a = struct_init();
        assert_eq!(a.get_name(), NAME);
    }

    #[test]
    fn test_get_description() {
        let a = struct_init();
        assert_eq!(a.get_description(), DESC);
    }

    #[test]
    fn test_get_active() {
        let a = struct_init();
        assert_eq!(a.get_active().len(), 0);
        assert_eq!(a.get_active().is_empty(), true);
    }

    #[test]
    fn test_get_archive() {
        let a = struct_init();
        assert_eq!(a.get_archive().len(), 0);
        assert_eq!(a.get_archive().is_empty(), true);
    }

    #[test]
    fn test_get_sticky() {
        let a = struct_init();
        assert_eq!(a.get_sticky().len(), 0);
        assert_eq!(a.get_sticky().is_empty(), true);
    }

    #[test]
    fn test_get_count() {
        let a = struct_init();
        assert_eq!(a.count, 0);
    }

    #[test]
    fn test_modify_name() {
        let mut a = struct_init();
        let new_name = &"Not a test board anymore";

        a.modify_name(new_name.to_string());
        assert_eq!(a.get_name(), new_name)
    }

    #[test]
    fn test_modify_description() {
        let mut a = struct_init();
        let new_desc = &"Cooked brainrot central on god fr fr skibidi gyatt ohio rizz";

        a.modify_description(new_desc.to_string());
        assert_eq!(a.get_description(), new_desc);
    }

    #[test]
    fn test_add_active() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_active(i as u64);
        }

        assert_eq!(a.get_active().len(), size);
        for i in 0..size {
            assert_eq!(*a.get_active().get(i).unwrap(), i as u64);
        }
    }

    #[test]
    fn test_add_archive() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_archive(i as u64);
        }

        assert_eq!(a.get_archive().len(), size);
        for i in 0..size {
            assert_eq!(*a.get_archive().get(i).unwrap(), i as u64);
        }
    }

    #[test]
    fn test_add_sticky() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_sticky(i as u64);
        }

        assert_eq!(a.get_sticky().len(), size);
        for i in 0..size {
            assert!(a.get_sticky().contains(&(i as u64)));
        }
    }

    #[test]
    fn test_remove_active() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_active(i as u64);
        }
        assert!(a.remove_active((size - 1) as u64));
        a.add_active((size - 1) as u64);

        assert!(!a.remove_active(size as u64));
        for i in 0..size {
            assert!(a.remove_active(i as u64));
            assert_eq!(a.get_active().len(), size - i - 1);
        }
        assert!(!a.remove_active(0));
    }

    #[test]
    fn test_remove_archive() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_archive(i as u64);
        }
        assert!(a.remove_archive((size - 1) as u64));
        a.add_archive((size - 1) as u64);

        assert!(!a.remove_archive(size as u64));
        for i in 0..size {
            assert!(a.remove_archive(i as u64));
            assert_eq!(a.get_archive().len(), size - i - 1);
        }
        assert!(!a.remove_archive(0));
    }

    #[test]
    fn test_remove_sticky() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_sticky(i as u64);
        }
        assert!(a.remove_sticky((size - 1) as u64));
        a.add_sticky((size - 1) as u64);

        assert!(!a.remove_sticky(size as u64));
        for i in 0..size {
            assert!(a.remove_sticky(i as u64));
            assert_eq!(a.get_sticky().len(), size - i - 1);
        }
        assert!(!a.remove_sticky(0));
    }

    #[test]
    fn test_pop_active() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_active(i as u64);
        }

        for i in 0..size {
            let thread = a.pop_active();
            assert!(thread.is_some());
            assert_eq!(thread.unwrap(), (size - i - 1) as u64);
            assert_eq!(a.get_active().len(), size - i - 1);
            assert!(!a.remove_active((size - i - 1) as u64));
        }
    }

    #[test]
    fn test_pop_archive() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 0..size {
            a.add_archive(i as u64);
        }

        for i in 0..size {
            let thread = a.pop_archive();
            assert!(thread.is_some());
            assert_eq!(thread.unwrap(), (size - i - 1) as u64);
            assert_eq!(a.get_archive().len(), size - i - 1);
            assert!(!a.remove_archive((size - i - 1) as u64));
        }
    }

    #[test]
    fn increment_count() {
        let mut a = struct_init();
        let size: usize = 127;

        for i in 1..size {
            assert_eq!(a.increment_count(), i as u64);
            assert_eq!(a.get_count(), i as u64);
        }
    }
}
