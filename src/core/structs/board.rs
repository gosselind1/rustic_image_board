use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::string::String;
use std::collections::VecDeque;
use std::fmt::Formatter;

pub(crate) struct Board {
    name: String,
    description: String,
    posts: HashSet<u64>,   // should this be in memory?
    active: VecDeque<u64>,  // active threads, as parent post IDs
    archive: VecDeque<u64>, // archive threads, as parent post IDs
    sticky: HashSet<u64>,  // should contain a subset of threads
    active_size: usize,  // # of allowed active+sticky threads
    archive_size: usize, // # of allowed non active threads
    count: u64  // running total of all ever existing posts
}

impl Board {

    pub fn new(name: String, description: String, active_size: usize, archive_size: usize) -> Board {

        let board = Board {
            name,
            description,
            posts: HashSet::new(),  // we might want to specify a capacity eventually
            active: VecDeque::with_capacity(active_size),
            archive: VecDeque::with_capacity(archive_size),
            sticky: HashSet::new(),
            active_size,
            archive_size,
            count: 0
        };

        return board;
    }

    pub fn get_name(self) -> String {
        return self.name.to_owned();
    }

    pub fn get_description(self) -> String {
        return self.description.to_owned();
    }

    pub fn get_posts(self) -> HashSet<u64> {
        return self.posts.to_owned();
    }

    pub fn get_post_len(self) -> usize {
        return self.posts.len();
    }

    pub fn get_active(self) -> VecDeque<u64> {
        return self.active.to_owned();
    }

    pub fn get_active_len(self) -> usize {
        return self.active.len();
    }

    pub fn get_archive(self) -> VecDeque<u64> {
        return self.archive.to_owned();
    }

    pub fn get_sticky(self) -> HashSet<u64> {
        return self.sticky.to_owned();
    }

    pub fn get_sticky_len(self) -> usize {
        return self.sticky.len();
    }

    pub fn get_archive_len(self) -> usize {
        return self.archive.len();
    }

    pub fn get_active_size(self) -> usize {
        return self.active_size;
    }

    pub fn get_archive_size(self) -> usize {
        return self.archive_size;
    }

    pub fn get_count(self) -> u64 {
        return self.count;
    }

    pub fn modify_name(mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn modify_description(mut self, new_description: String) {
        self.description = new_description;
    }

    pub fn add_post(mut self) -> u64 {
        // A more sophisticated distributed implementation would fetch/check count from storage
        // for such an implementation, this function should include a post # input param
        // which is bounds checked, and then checked if it already exists within the structure.
        // The panic should be substituted for a exception in this case
        self.count += 1;
        let check: bool = self.posts.insert(self.count);

        if check {
            return self.count;
        } else { // SHOULD NEVER HAPPEN
            panic!("Board already has post {}", self.count);
        }
    }

    pub fn contains_post(mut self, post: &u64) -> bool {
        return self.posts.contains(post);
    }

    pub fn remove_post(mut self, post: &u64) -> Result<(), PostNotFoundError> {
        let exist: bool = self.posts.remove(post);

        if exist {
            return Ok(());
        } else {
            return Err(PostNotFoundError(post.to_string()));
        }
    }

    fn queue_thread(thread_list: &mut VecDeque<u64>, uid: String, thread_id: u64) -> Result<(), ThreadListFullError> {
        let size: usize = thread_list.capacity();
        let len: usize = thread_list.len();
;
        if len > size { // THIS SHOULD NEVER HAPPEN
            panic!("Thread count exceeded set size.");
        } else if  len == size {
            return Err(ThreadListFullError(uid));
        } else {
            return  Ok(thread_list.push_back(thread_id));
        }
    }

    fn dequeue_thread(thread_list: &mut VecDeque<u64>, uid: String) -> Result<u64, ThreadListEmptyError> {
        let thread: Option<u64> = thread_list.pop_front();

        if thread.is_some() { // I think this could technically be better written, but idk how
            return Ok(thread.unwrap());
        } else {
            return Err(ThreadListEmptyError(uid));
        }
    }

    fn contains_thread(thread_list: &VecDeque<u64>, post: &u64) -> bool {
        return thread_list.contains(post);
    }

    fn bump_thread(thread_list: &mut VecDeque<u64>, uid: String, thread_id: &u64) -> Result<(), ThreadNotFoundError> {
        if thread_list.contains(thread_id) {
            thread_list.retain(|&candidate| candidate != *thread_id);
            thread_list.push_back(*thread_id);
            return Ok(())
        } else {
            return Err(ThreadNotFoundError(uid + thread_id.to_string().as_str()));
        }
    }

    pub fn queue_active(&mut self, thread_id: u64) -> Result<(), ThreadListFullError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::queue_thread(thread_list, self.name.to_owned() + " active", thread_id);
    }

    pub fn dequeue_active(&mut self) -> Result<u64, ThreadListEmptyError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::dequeue_thread(thread_list, self.name.to_owned() + " active");
    }

    pub fn bump_active(&mut self, thread_id: &u64) -> Result<(), ThreadNotFoundError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::bump_thread(thread_list, self.name.to_owned() + " active", thread_id);
    }

    pub fn queue_archive(&mut self, thread_id: u64) -> Result<(), ThreadListFullError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::queue_thread(thread_list, self.name.to_owned() + " archive", thread_id);
    }

    pub fn dequeue_archive(&mut self) -> Result<u64, ThreadListEmptyError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::dequeue_thread(thread_list, self.name.to_owned() + " archive");
    }

    pub fn bump_archive(&mut self, thread_id: &u64) -> Result<(), ThreadNotFoundError> {
        let thread_list: &mut VecDeque<u64> = self.active.borrow_mut();
        return Board::bump_thread(thread_list, self.name.to_owned() + " archive", thread_id);
    }

    pub fn add_sticky(&mut self, thread_id: u64) {
        self.sticky.insert(thread_id);
    }

    pub fn contains_sticky(&self, thread_id: &u64) -> bool {
        return self.sticky.contains(thread_id);
    }

    pub fn remove_sticky(&mut self, thread_id: &u64) -> Result<(), ThreadNotFoundError> {
        let contains: bool = self.sticky.remove(thread_id);

        if contains {
            return Ok(());
        } else {
            return Err(ThreadNotFoundError(thread_id.to_string()));
        }
    }
}

#[derive(Debug)]
pub struct ThreadListFullError(String);

impl std::error::Error for ThreadListFullError {}

impl std::fmt::Display for ThreadListFullError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: board is full!")
    }
}

#[derive(Debug)]
pub struct ThreadListEmptyError(String);

impl std::error::Error for ThreadListEmptyError {}

impl std::fmt::Display for ThreadListEmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: board is empty!")
    }
}

#[derive(Debug)]
pub struct ThreadNotFoundError(String);

impl std::error::Error for ThreadNotFoundError {}

impl std::fmt::Display for ThreadNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: thread not found!")
    }
}

#[derive(Debug)]
pub struct PostNotFoundError(String);

impl std::error::Error for PostNotFoundError {}

impl std::fmt::Display for PostNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: post not found!")
    }
}
