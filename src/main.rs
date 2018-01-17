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

use std::path::PathBuf;
use std::fs;
use std::process;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use termcolor::{ColorChoice, StandardStream};

mod abstract_type;
mod annotate;
mod ast;
mod command;
mod config;
mod debug;
mod define;
mod environment;
mod errors;
mod inflect;
mod load;
mod object;
mod remote;
mod slice_util;
mod strip;
mod top_level;
mod typecheck;
mod util;

use config::{AnnotateConfig, CheckConfig, StripConfig};
use errors::ErrorReporter;

enum Command {
    Check(CheckConfig, Vec<PathBuf>),
    Annotate(AnnotateConfig, PathBuf),
    Strip(StripConfig, Vec<PathBuf>),
    Server,
}

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

fn parse_cmdline() -> Command {
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

fn main() {
    let mut errors = ErrorReporter::new(StandardStream::stderr(ColorChoice::Auto));

    let success = match parse_cmdline() {
        Command::Check(config, files) => command::check(errors, config, files),
        Command::Annotate(config, file) => command::annotate(errors, config, file),
        Command::Strip(config, files) => command::strip(errors, config, files),
        Command::Server => command::server(&mut errors),
    };

    process::exit(match success {
        true => 0,
        false => 1,
    });
}
