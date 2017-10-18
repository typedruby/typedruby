use std::path::PathBuf;
use std::vec::Vec;

pub struct CheckConfig {
    pub require_paths: Vec<PathBuf>,
    pub warning: bool,
    pub autoload_paths: Vec<PathBuf>,
    pub inflect_acronyms: Vec<String>,
    pub ignore_errors_in: Vec<PathBuf>,
    pub strip: bool,
}

impl CheckConfig {
    // TODO read from a config file or something
    pub fn new() -> CheckConfig {
        CheckConfig {
            require_paths: Vec::new(),
            warning: false,
            autoload_paths: Vec::new(),
            inflect_acronyms: Vec::new(),
            ignore_errors_in: Vec::new(),
            strip: false,
        }
    }
}

pub struct StripConfig {
    pub annotate: bool,
    pub print: bool,
}

pub enum Command {
    Check(CheckConfig, Vec<PathBuf>),
    Strip(StripConfig, Vec<PathBuf>),
}
