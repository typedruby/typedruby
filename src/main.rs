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
use clap::{App, AppSettings, Arg, SubCommand};
use typed_arena::Arena;
use termcolor::{ColorChoice, StandardStream};

mod abstract_type;
mod ast;
mod config;
mod debug;
mod define;
mod environment;
mod errors;
mod inflect;
mod object;
mod slice_util;
mod strip;
mod top_level;
mod typecheck;
mod util;

use strip::StripError;
use environment::Environment;
use errors::{ErrorReporter, ErrorSink};
use config::{Command, CheckConfig, StripConfig};

fn command() -> Command {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("strip")
                .about("Strip source files of type annotations in place without type checking")
                .arg(Arg::with_name("annotate")
                        .help("Annotate stripped type annotations without modifying source files")
                        .short("a")
                        .long("annotate"))
                .arg(Arg::with_name("print")
                        .help("Print stripped source without modifying source files")
                        .short("p")
                        .long("print"))
                .arg(Arg::with_name("source")
                    .index(1)
                    .multiple(true)
                    .required(true)
                    .help("Source files to strip")))
        .subcommand(
            SubCommand::with_name("check")
                .about("Type check Ruby source files")
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
                    .help("Source files to type check")));

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("check") {
        let mut config = CheckConfig::new();
        let mut files = Vec::new();

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

        config.strip = matches.is_present("strip");

        if let Some(files_iter) = matches.values_of("source") {
            files.extend(files_iter.map(PathBuf::from));
        }

        Command::Check(config, files)
    } else if let Some(matches) = matches.subcommand_matches("strip") {
        let config = StripConfig {
            annotate: matches.is_present("annotate"),
            print: matches.is_present("print"),
        };

        let files = matches.values_of("source")
            .expect("source is required")
            .map(PathBuf::from)
            .collect();

        Command::Strip(config, files)
    } else {
        panic!("unreachable - clap should have exited if no subcommand matched");
    }
}

fn check(errors: Box<ErrorSink>, config: CheckConfig, files: Vec<PathBuf>) -> bool {
    let arena = Arena::new();
    let env = Environment::new(&arena, errors, config);

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

    success &&
        errors.error_count() == 0 &&
        errors.warning_count() == 0
}

fn strip(mut errors: Box<ErrorSink>, config: StripConfig, files: Vec<PathBuf>) -> bool {
    let mut success = true;

    for file in files {
        match strip::strip_file(file.clone(), &config) {
            Ok(()) => {},
            Err(err) => {
                success = false;

                match err {
                    StripError::Syntax(diagnostics) => {
                        for diagnostic in diagnostics {
                            errors.parser_diagnostic(&diagnostic);
                        }
                    }
                    StripError::Io(err) => {
                        errors.error(&format!("Could not open {}: {}", file.display(), err), &[]);
                    }
                }
            }
        }
    }

    success
}

fn main() {
    let errors = Box::new(ErrorReporter::new(StandardStream::stderr(ColorChoice::Auto)));

    let success = match command() {
        Command::Check(config, files) => check(errors, config, files),
        Command::Strip(config, files) => strip(errors, config, files),
    };

    process::exit(match success {
        true => 0,
        false => 1,
    });
}
