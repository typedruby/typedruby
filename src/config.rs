use std::path::PathBuf;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
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

pub struct AnnotateConfig {
    pub print: bool,
}

pub struct StripConfig {
    pub annotate: bool,
    pub print: bool,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Paths {
    None,
    One(PathBuf),
    Many(Vec<PathBuf>),
}

impl Default for Paths {
    fn default() -> Self {
        Paths::None
    }
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

#[derive(Deserialize, Default, Debug)]
pub struct TypedRubyConfig {
    #[serde(default)] pub bundler: bool,
    #[serde(default)] pub glob: bool,
    #[serde(default)] pub files: Paths,
    #[serde(default)] pub load_path: Paths,
    #[serde(default)] pub autoload_path: Paths,
    #[serde(default)] pub ignore_errors: Paths,
}

#[derive(Deserialize, Default, Debug)]
pub struct InflectConfig {
    #[serde(default)] pub acronyms: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct CodegenConfig {
    #[serde(default)] pub exec: Strings,
}

#[derive(Deserialize, Default, Debug)]
pub struct ProjectConfig {
    #[serde(default)] pub typedruby: TypedRubyConfig,
    #[serde(default)] pub inflect: InflectConfig,
    #[serde(default)] pub codegen: CodegenConfig,
}
