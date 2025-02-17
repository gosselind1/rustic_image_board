// Config should contain auth / connection details.
// ie, it should be the configuration required to make a connection to the storage system.
// this can also act as a state manager; eg in the case of an active db connection
use std::path::Path;

pub(crate) struct Config {
    root: &'static Path,
}

impl Config {
    pub fn new(root: String) {
        let root_path = Path::new(&root);
        let config = Config { root: &root_path };
    }

    pub fn connect(&self) -> Result<(), std::io::Error> {
        self.root;
    }
}
