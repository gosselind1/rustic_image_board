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
        self.count += 1;
    }

    pub fn add_archive(&mut self, parent: u64) {
        self.archive.push(parent);
        self.count += 1;
    }

    pub fn add_sticky(&mut self, parent: u64) -> bool {
        return self.sticky.insert(parent);
    }

    pub fn remove_active(&mut self, parent: u64) -> bool {
        let removed = Board::remove_element(&mut self.active, parent);
        if removed {
            self.count -= 1;
        }
        return removed;
    }

    pub fn remove_archive(&mut self, parent: u64) -> bool {
        let removed = Board::remove_element(&mut self.archive, parent);
        if removed {
            self.count -= 1;
        }
        return removed;
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

    pub fn pop_active(&mut self) -> Option<u64> {
        return self.active.pop();
    }

    pub fn pop_archive(&mut self) -> Option<u64> {
        return self.archive.pop();
    }

    pub fn remove_sticky(&mut self, parent: u64) -> bool {
        return self.sticky.remove(&parent);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
