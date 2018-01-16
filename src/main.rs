#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate crossbeam;
extern crate glob;
extern crate immutable_map;
extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate termcolor;
extern crate typed_arena;
extern crate vec_map;

use std::env;
use std::path::PathBuf;
use std::fs;
use std::process;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use typed_arena::Arena;
use termcolor::{ColorChoice, StandardStream};

mod abstract_type;
mod annotate;
mod ast;
mod config;
mod debug;
mod define;
mod environment;
mod errors;
mod inflect;
mod object;
mod remote;
mod slice_util;
mod strip;
mod top_level;
mod typecheck;
mod util;

use annotate::AnnotateError;
use strip::StripError;
use environment::Environment;
use errors::{ErrorReporter, ErrorSink};
use config::{Command, AnnotateConfig, CheckConfig, StripConfig};

fn source_files(matches: &ArgMatches) -> Vec<PathBuf> {
    let sources = matches.values_of("source")
        .expect("sources should be required");

    if matches.is_present("glob") {
        sources
            .flat_map(|pattern| glob::glob(pattern).ok())
            .flat_map(|iter| iter.flat_map(Result::ok))
            .collect()
    } else {
        sources
            .map(PathBuf::from)
            .collect()
    }
}

fn command() -> Command {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("annotate")
                .about("Annotate source files in place with provided type annotations")
                .arg(Arg::with_name("print")
                        .help("Print annotated source without modifying source files")
                        .short("p")
                        .long("print"))
                .arg(Arg::with_name("input")
                    .index(1)
                    .multiple(false)
                    .required(true)
                    .help("Annotations file")))
        .subcommand(
            SubCommand::with_name("strip")
                .about("Strip source files of type annotations in place without type checking")
                .arg(Arg::with_name("annotate")
                        .help("Annotate stripped type annotations without modifying source files")
                        .short("a")
                        .long("annotate")
                        .conflicts_with("print"))
                .arg(Arg::with_name("print")
                        .help("Print stripped source without modifying source files")
                        .short("p")
                        .long("print"))
                .arg(Arg::with_name("glob")
                        .help("Treat source filenames as glob patterns")
                        .short("g")
                        .long("glob"))
                .arg(Arg::with_name("source")
                    .index(1)
                    .multiple(true)
                    .required(true)
                    .help("Source files to strip")))
        .subcommand(
            SubCommand::with_name("server")
                .about("Runs the TypedRuby development server for the project in the current directory"))
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
                .arg(Arg::with_name("glob")
                    .help("Treat source filenames as glob patterns")
                    .short("g")
                    .long("glob"))
                .arg(Arg::with_name("source")
                    .index(1)
                    .multiple(true)
                    .required(true)
                    .help("Source files to type check")));

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("check") {
        let mut config = CheckConfig::new();

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

        Command::Check(config, source_files(matches))
    } else if let Some(matches) = matches.subcommand_matches("annotate") {
        let config = AnnotateConfig {
            print: matches.is_present("print"),
        };

        let input = matches.value_of("input")
            .expect("input should be required")
            .into();

        Command::Annotate(config, input)
    } else if let Some(matches) = matches.subcommand_matches("strip") {
        let config = StripConfig {
            annotate: matches.is_present("annotate"),
            print: matches.is_present("print"),
        };

        Command::Strip(config, source_files(matches))
    } else if let Some(_) = matches.subcommand_matches("server") {
        Command::Server
    } else {
        panic!("unreachable - clap should have exited if no subcommand matched");
    }
}

fn check(mut errors: ErrorReporter<StandardStream>, mut config: CheckConfig, files: Vec<PathBuf>) -> bool {
    let socket_path = remote::server::socket_path().expect("server::socket_path");

    if socket_path.exists() {
        match remote::client::check_remote(&socket_path, &mut errors, config, files) {
            Ok(result) => result,
            Err(e) => panic!("client::check_remote: {:?}", e),
        }
    } else {
        let arena = Arena::new();

        if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
            config.require_paths.insert(0, PathBuf::from(lib_path));
        } else {
            errors.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
        }

        let env = Environment::new(&arena, &mut errors, config);

        env.load_files(files.iter());
        env.define();
        env.typecheck();

        let errors = env.error_sink.borrow();

        errors.error_count() == 0 && errors.warning_count() == 0
    }
}

fn annotate(mut errors: ErrorReporter<StandardStream>, config: AnnotateConfig, file: PathBuf) -> bool {
    match annotate::apply_annotations(&file, config) {
        Ok(()) => true,
        Err(err) => {
            match err {
                AnnotateError::Syntax(diagnostics) => {
                    for diagnostic in diagnostics {
                        errors.parser_diagnostic(&diagnostic);
                    }
                }
                AnnotateError::Json(err) => {
                    errors.error(&format!("Could not parse line of annotations file: {}", err), &[]);
                }
                AnnotateError::Io(err) => {
                    errors.error(&format!("Could not open {}: {}", file.display(), err), &[]);
                }
            }

            false
        }
    }
}

fn strip(mut errors: ErrorReporter<StandardStream>, config: StripConfig, files: Vec<PathBuf>) -> bool {
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

fn server(errors: &mut ErrorSink) -> bool {
    match remote::server::run() {
        Ok(()) => {
            true
        }
        Err(e) => {
            errors.error(&format!("Could not run server: {:?}", e), &[]);
            false
        }
    }
}

fn main() {
    let mut errors = ErrorReporter::new(StandardStream::stderr(ColorChoice::Auto));

    let success = match command() {
        Command::Check(config, files) => check(errors, config, files),
        Command::Annotate(config, file) => annotate(errors, config, file),
        Command::Strip(config, files) => strip(errors, config, files),
        Command::Server => server(&mut errors),
    };

    process::exit(match success {
        true => 0,
        false => 1,
    });
}
