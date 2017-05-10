use std::path::PathBuf;
use std::vec::Vec;

pub struct Config {
    pub require_paths: Vec<PathBuf>,
    pub warning: bool,
}

impl Config {
    // TODO read from a config file or something
    pub fn new() -> Config {
        Config {
            require_paths: Vec::new(),
            warning: false,
        }
    }
}
