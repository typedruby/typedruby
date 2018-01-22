use std::path::PathBuf;
use std::vec::Vec;

use ref_slice::ref_slice;
use serde::{Deserialize, Deserializer};
use serde::de::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckConfig {
    pub require_paths: Vec<PathBuf>,
    pub warning: bool,
    pub autoload_paths: Vec<PathBuf>,
    pub inflect_acronyms: Vec<String>,
    pub ignore_errors_in: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}

pub struct AnnotateConfig {
    pub print: bool,
}

pub struct StripConfig {
    pub annotate: bool,
    pub print: bool,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Strings {
    None,
    One(String),
    Many(Vec<String>),
}

impl Default for Strings {
    fn default() -> Self {
        Strings::None
    }
}

impl Strings {
    pub fn as_slice(&self) -> &[String] {
        match *self {
            Strings::None => &[],
            Strings::One(ref string) => ref_slice(string),
            Strings::Many(ref strings) => strings,
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Shell { cmd: String },
    Argv { bin: String, args: Vec<String> },
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RawCommand {
            Shell(String),
            Argv(Vec<String>),
        }

        RawCommand::deserialize(deserializer)
            .and_then(|strings| match strings {
                RawCommand::Shell(cmd) => Ok(Command::Shell { cmd }),
                RawCommand::Argv(argv) => {
                    match argv.split_first() {
                        Some((bin, args)) => {
                            Ok(Command::Argv {
                                bin: bin.to_owned(),
                                args: args.to_vec(),
                            })
                        }
                        None => {
                            Err(D::Error::custom("command argument vector may not be empty"))
                        }
                    }
                }
            })
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct TypedRubyConfig {
    #[serde(default)] pub source: Strings,
    #[serde(default)] pub load_path: Strings,
    #[serde(default)] pub autoload_path: Strings,
    #[serde(default)] pub ignore_errors: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct BundlerConfig {
    #[serde(default)] pub enabled: Option<bool>,
    #[serde(default)] pub exec: Option<Command>,
    #[serde(default)] pub refresh: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct InflectConfig {
    #[serde(default)] pub acronyms: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct CodegenConfig {
    #[serde(default)] pub exec: Option<Command>,
    #[serde(default)] pub refresh: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct ProjectConfig {
    #[serde(default)] pub typedruby: TypedRubyConfig,
    #[serde(default)] pub bundler: BundlerConfig,
    #[serde(default)] pub inflect: InflectConfig,
    #[serde(default)] pub codegen: CodegenConfig,
}
