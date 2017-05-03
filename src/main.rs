extern crate clap;
extern crate typed_arena;
extern crate immutable_map;

use std::io;
use std::path::PathBuf;
use clap::{App, Arg};
use typed_arena::Arena;

mod ast;
mod config;
mod environment;
mod errors;
mod object;
mod top_level;
mod typecheck;

use environment::Environment;
use errors::ErrorReporter;
use config::Config;

fn config() -> (Config, Vec<PathBuf>) {
    let mut config = Config::new();
    let mut files = Vec::new();

    let matches = App::new("typedruby")
        .arg(Arg::with_name("load-path")
            .multiple(true)
            .number_of_values(1)
            .short("I")
            .value_name("directory")
            .help("Adds a directory to the load path")
            .takes_value(true))
        .arg(Arg::with_name("source")
            .index(1)
            .multiple(true)
            .required(true)
            .help("Source files to type check"))
        .get_matches();

    if let Some(load_paths) = matches.values_of("load-path") {
        config.require_paths.extend(load_paths.map(PathBuf::from));
    }

    if let Some(files_iter) = matches.values_of("source") {
        files.extend(files_iter.map(PathBuf::from));
    }

    (config, files)
}

fn main() {
    let (config, files) = config();
    let errors = ErrorReporter::new(io::stderr());
    let arena = Arena::new();
    let env = Environment::new(&arena, Box::new(errors), config);

    for file in files {
        match env.load_file(&file) {
            Ok(()) => (),
            Err(e) => println!("{}: {:?}", file.display(), e),
        }
    }

    env.typecheck();
}
