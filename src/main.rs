#[macro_use]
extern crate clap;
extern crate immutable_map;
extern crate itertools;
extern crate regex;
extern crate typed_arena;
extern crate termcolor;

use std::path::PathBuf;
use std::fs;
use std::process;
use clap::{App, Arg};
use typed_arena::Arena;
use termcolor::{ColorChoice, StandardStream};

mod ast;
mod abstract_type;
mod config;
mod define;
mod deferred_cell;
mod environment;
mod errors;
mod inflect;
mod object;
mod slice_util;
mod top_level;
mod typecheck;
mod util;

use environment::Environment;
use errors::ErrorReporter;
use config::Config;

fn config() -> (Config, Vec<PathBuf>) {
    let mut config = Config::new();
    let mut files = Vec::new();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("load-path")
            .multiple(true)
            .number_of_values(1)
            .short("I")
            .value_name("directory")
            .help("Adds a directory to the load path")
            .takes_value(true))
        .arg(Arg::with_name("autoload-path")
            .multiple(true)
            .number_of_values(1)
            .short("R")
            .long("rails-autoload")
            .value_name("directory")
            .help("Turns on Rails-style autoloading and adds a directory to the autoload path"))
        .arg(Arg::with_name("inflect-acronym")
            .multiple(true)
            .number_of_values(1)
            .long("inflect-acronym")
            .value_name("word")
            .help("Registers the passed word as an acronym for inflection in Rails-style autoloading"))
        .arg(Arg::with_name("ignore-errors-in")
            .multiple(true)
            .number_of_values(1)
            .long("ignore-errors-in")
            .value_name("directory")
            .help("Ignores warnings/errors under a path prefix"))
        .arg(Arg::with_name("warning")
            .short("w")
            .help("Turns on additional warnings, like Ruby's -w"))
        .arg(Arg::with_name("source")
            .index(1)
            .multiple(true)
            .required(true)
            .help("Source files to type check"))
        .get_matches();

    if let Some(load_paths) = matches.values_of("load-path") {
        config.require_paths.extend(load_paths.map(PathBuf::from));
    }

    if let Some(autoload_paths) = matches.values_of("autoload-path") {
        config.autoload_paths.extend(autoload_paths.map(PathBuf::from));
    }

    if let Some(acronyms) = matches.values_of("inflect-acronym") {
        config.inflect_acronyms.extend(acronyms.map(String::from));
    }

    if let Some(ignore_errors_in) = matches.values_of("ignore-errors-in") {
        config.ignore_errors_in.extend(ignore_errors_in
            .map(PathBuf::from)
            .map(fs::canonicalize)
            .filter_map(Result::ok));
    }

    config.warning = matches.is_present("warning");

    if let Some(files_iter) = matches.values_of("source") {
        files.extend(files_iter.map(PathBuf::from));
    }

    (config, files)
}

fn main() {
    let (config, files) = config();
    let errors = ErrorReporter::new(StandardStream::stderr(ColorChoice::Auto));
    let arena = Arena::new();
    let env = Environment::new(&arena, Box::new(errors), config);

    let success = files.iter().all(|file|
        match env.require(&file) {
            Ok(()) => true,
            Err(e) => {
                env.error_sink.borrow_mut()
                    .error(&format!("{}: {}", file.display(), e), &[]);
                false
            }
        });

    env.define();

    env.typecheck();

    let errors = env.error_sink.borrow();

    let success = success &&
        errors.error_count() == 0 &&
        errors.warning_count() == 0;

    process::exit(match success {
        true => 0,
        false => 1,
    });
}
