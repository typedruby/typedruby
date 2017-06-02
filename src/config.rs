use std::path::PathBuf;
use std::vec::Vec;

pub struct Config {
    pub require_paths: Vec<PathBuf>,
    pub warning: bool,
    pub autoload_paths: Vec<PathBuf>,
    pub inflect_acronyms: Vec<String>,
    pub ignore_errors_in: Vec<PathBuf>,
}

impl Config {
    // TODO read from a config file or something
    pub fn new() -> Config {
        Config {
            require_paths: Vec::new(),
            warning: false,
            autoload_paths: Vec::new(),
            inflect_acronyms: Vec::new(),
            ignore_errors_in: Vec::new(),
        }
    }
}
