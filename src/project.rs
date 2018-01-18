use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process;
use std::str;

use glob;
use toml;

use config::{ProjectConfig, CheckConfig, Strings, Command, BundlerConfig};
use errors::ErrorSink;
use load::LoadCache;

const CONFIG_FILE: &'static str = "TypedRuby.toml";
const SOCKET_FILE: &'static str = ".typedruby.sock";

pub struct Project {
    pub cache: LoadCache,
    pub root: PathBuf,
    pub config: ProjectConfig,
    pub check_config: CheckConfig,
}

#[derive(Debug)]
pub enum ProjectError {
    NoProjectConfig,
    Io(io::Error),
    Toml(toml::de::Error),
    GlobPattern(glob::PatternError),
    Glob(glob::GlobError),
    Bundler { stdout: Vec<u8>, stderr: Vec<u8> },
}

fn find_typedruby_toml(initial_dir: &Path) -> Option<PathBuf> {
    let mut initial_dir = initial_dir.to_owned();
    initial_dir.push(CONFIG_FILE);

    let mut dir = initial_dir.clone();

    loop {
        let config = dir.with_file_name(CONFIG_FILE);

        if config.exists() {
            return Some(config);
        }

        if !dir.pop() {
            break;
        }
    }

    None
}

fn read_typedruby_toml(path: &Path) -> Result<ProjectConfig, ProjectError> {
    let mut buff = String::new();

    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buff))
        .map_err(ProjectError::Io)?;

    toml::from_str(&buff).map_err(ProjectError::Toml)
}

fn load_bundler_load_paths(project_root: &Path, config: &BundlerConfig) -> Result<Vec<PathBuf>, ProjectError> {
    let mut command;

    match *config {
        BundlerConfig { enabled: Some(false), .. } |
        BundlerConfig { enabled: None, exec: None } => {
            return Ok(Vec::new());
        }
        BundlerConfig { enabled: Some(true), exec: None } => {
            command = process::Command::new("ruby");
            command.args(&["-r", "bundler/setup", "-e", "puts $LOAD_PATH"]);
        }
        BundlerConfig { exec: Some(Command::Shell { ref cmd }), .. } => {
            // TODO implement windows support
            command = process::Command::new("sh");
            command.args(&["-e", "-c", cmd]);
        }
        BundlerConfig { exec: Some(Command::Argv { ref bin, ref args }), .. } => {
            command = process::Command::new(bin);
            command.args(args);
        }
    };

    let output = command
        .current_dir(project_root)
        .output()
        .map_err(ProjectError::Io)?;

    if output.status.success() {
        Ok(output.stdout
            .split(|c| *c == ('\r' as u8) || *c == ('\n' as u8))
            .filter_map(|line| str::from_utf8(line).ok())
            .map(PathBuf::from)
            .collect())
    } else {
        Err(ProjectError::Bundler {
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

fn paths_from_strings(strings: &Strings) -> Result<Vec<PathBuf>, ProjectError> {
    let mut paths = Vec::new();

    for pattern in strings.as_slice() {
        for path in glob::glob(pattern).map_err(ProjectError::GlobPattern)? {
            let path = path.map_err(ProjectError::Glob)?;
            paths.push(path);
        }
    }

    Ok(paths)
}

fn init_check_config(errors: &mut ErrorSink, project_root: &Path, config: &ProjectConfig) -> Result<CheckConfig, ProjectError> {
    let mut require_paths = Vec::new();

    require_paths.extend(
        paths_from_strings(&config.typedruby.load_path)?);

    if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
        require_paths.push(PathBuf::from(lib_path));
    } else {
        errors.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
    }

    require_paths.extend(
        load_bundler_load_paths(project_root, &config.bundler)?);

    let autoload_paths = paths_from_strings(&config.typedruby.autoload_path)?;
    let ignore_errors_in = paths_from_strings(&config.typedruby.ignore_errors)?;
    let files = paths_from_strings(&config.typedruby.source)?;

    let inflect_acronyms = config.inflect.acronyms.as_slice().to_vec();

    Ok(CheckConfig {
        warning: false,
        require_paths,
        autoload_paths,
        ignore_errors_in,
        inflect_acronyms,
        files,
    })
}

impl Project {
    pub fn find(errors: &mut ErrorSink, dir: &Path) -> Result<Project, ProjectError> {
        let config_path = find_typedruby_toml(dir).ok_or(ProjectError::NoProjectConfig)?;

        let config = read_typedruby_toml(&config_path)?;

        let root = {
            let mut root = config_path;
            root.pop();
            root
        };

        // XXX - GLOBAL STATE!
        //
        // the glob crate is always relative to the process's current directory
        // and does not allow us to pass a base path to use instead.
        // until the glob crate is fixed we need to set the process's current
        // directory to the current project.
        // this prevents us from supporting multiple projects in the same process
        //
        env::set_current_dir(&root).map_err(ProjectError::Io)?;

        Ok(Project {
            check_config: init_check_config(errors, &root, &config)?,
            root,
            config,
            cache: LoadCache::new(),
        })
    }

    pub fn socket_path(&self) -> PathBuf {
        self.root.with_file_name(SOCKET_FILE)
    }
}

