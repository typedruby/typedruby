use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};
use std::str;
use std::time::SystemTime;

use glob;
use toml;

use config::{ProjectConfig, CheckConfig, Strings, Command, BundlerConfig};
use load::LoadCache;
use report::Reporter;

const CONFIG_FILE: &'static str = "TypedRuby.toml";
const SOCKET_FILE: &'static str = ".typedruby.sock";

#[derive(Clone)]
pub struct ProjectPath {
    root: PathBuf,
}

impl ProjectPath {
    pub fn find(dir: PathBuf) -> Option<ProjectPath> {
        let mut dir = dir;

        loop {
            if dir.join(CONFIG_FILE).exists() {
                return Some(ProjectPath { root: dir });
            }

            if !dir.pop() {
                break;
            }
        }

        None
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config_path(&self) -> PathBuf {
        self.root.join(CONFIG_FILE)
    }

    pub fn socket_path(&self) -> PathBuf {
        self.root.join(SOCKET_FILE)
    }
}

pub struct Project {
    pub cache: LoadCache,
    pub path: ProjectPath,
    pub config: ProjectConfig,
    pub check_config: CheckConfig,
    config_mtime: SystemTime,
}

#[derive(Debug)]
pub enum ProjectError {
    Io(io::Error),
    Toml(toml::de::Error),
    GlobPattern(glob::PatternError),
    Glob(glob::GlobError),
    Bundler(CommandError),
    Codegen(CommandError),
}

#[derive(Debug)]
pub enum CommandError {
    NonZeroStatus,
    Io(io::Error),
}

fn read_typedruby_toml(path: &Path) -> Result<ProjectConfig, ProjectError> {
    let mut buff = String::new();

    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buff))
        .map_err(ProjectError::Io)?;

    toml::from_str(&buff).map_err(ProjectError::Toml)
}

fn prepare_command(project_root: &Path, bin: &str) -> process::Command {
    let mut process = process::Command::new(bin);
    process.current_dir(project_root);
    process.stderr(Stdio::inherit());
    process
}

fn prepare_config_command(project_root: &Path, command: &Command) -> process::Command {
    let mut process;

    match *command {
        Command::Shell { ref cmd } => {
            // TODO implement windows support
            process = prepare_command(project_root, "sh");
            process.args(&["-e", "-c", cmd]);
        }
        Command::Argv { ref bin, ref args } => {
            process = prepare_command(project_root, bin);
            process.args(args);
        }
    }

    process
}

fn load_bundler_load_paths(reporter: &mut Reporter, project_root: &Path, config: &BundlerConfig) -> Result<Vec<PathBuf>, CommandError> {
    let mut command = match *config {
        BundlerConfig { enabled: Some(false), .. } |
        BundlerConfig { enabled: None, exec: None } => {
            return Ok(Vec::new());
        }
        BundlerConfig { enabled: Some(true), exec: None } => {
            let mut command = prepare_command(project_root, "ruby");
            command.args(&["-r", "bundler/setup", "-e", "puts $LOAD_PATH"]);
            command
        }
        BundlerConfig { exec: Some(ref cmd), .. } => {
            prepare_config_command(project_root, cmd)
        }
    };

    reporter.info("Loading Gem environment...");

    let output = command.output().map_err(CommandError::Io)?;

    if output.status.success() {
        Ok(output.stdout
            .split(|c| *c == ('\r' as u8) || *c == ('\n' as u8))
            .filter_map(|line| str::from_utf8(line).ok())
            .map(PathBuf::from)
            .collect())
    } else {
        reporter.error("Command exited with non-zero status", &[]);
        Err(CommandError::NonZeroStatus)
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

fn init_check_config(reporter: &mut Reporter, project_root: &Path, config: &ProjectConfig) -> Result<CheckConfig, ProjectError> {
    let mut require_paths = Vec::new();

    require_paths.extend(
        paths_from_strings(&config.typedruby.load_path)?);

    if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
        require_paths.push(PathBuf::from(lib_path));
    } else {
        reporter.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
    }

    require_paths.extend(
        load_bundler_load_paths(reporter, project_root, &config.bundler)
            .map_err(ProjectError::Bundler)?);

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

fn codegen(reporter: &mut Reporter, project_root: &Path, config: &ProjectConfig) -> Result<(), CommandError> {
    let cmd = match config.codegen.exec {
        Some(ref cmd) => cmd,
        None => { return Ok(()) }
    };

    reporter.info("Generating code...");

    match prepare_config_command(project_root, cmd).status() {
        Ok(stat) if stat.success() => Ok(()),
        Ok(_) => {
            reporter.error("Command exited with non-zero status", &[]);
            Err(CommandError::NonZeroStatus)
        }
        Err(e) => {
            reporter.error("Could not invoke codegen command", &[]);
            Err(CommandError::Io(e))
        }
    }
}

impl Project {
    pub fn new(reporter: &mut Reporter, path: ProjectPath) -> Result<Project, ProjectError> {
        let config_path = path.config_path();

        let config = read_typedruby_toml(&config_path)?;

        let config_mtime = fs::metadata(&config_path)
            .map_err(ProjectError::Io)?
            .modified()
            .map_err(ProjectError::Io)?;

        // XXX - GLOBAL STATE!
        //
        // the glob crate is always relative to the process's current directory
        // and does not allow us to pass a base path to use instead.
        // until the glob crate is fixed we need to set the process's current
        // directory to the current project.
        // this prevents us from supporting multiple projects in the same process
        //
        env::set_current_dir(path.root()).map_err(ProjectError::Io)?;

        let check_config = init_check_config(reporter, path.root(), &config)?;

        let project = Project {
            check_config,
            path,
            config,
            config_mtime,
            cache: LoadCache::new(),
        };

        codegen(reporter, project.path.root(), &project.config)
            .map_err(ProjectError::Codegen)?;

        Ok(project)
    }

    pub fn needs_reload(&self) -> bool {
        let mtime = fs::metadata(self.path.config_path())
            .and_then(|meta| meta.modified());

        match mtime {
            Ok(mtime) => mtime > self.config_mtime,
            Err(_) => {
                // if we couldn't fetch the mtime of the project config file
                // then always force a project refresh
                true
            }
        }
    }

    pub fn refresh(&self) {
        // TODO
    }
}

#[cfg(test)]
pub fn load_fixture(reporter: &mut Reporter, fixture_path: &Path) -> Project {
    let root = fixture_path
        .parent()
        .expect("fixture_path.parent()")
        .to_owned();

    let mut config = ProjectConfig::default();

    config.typedruby.source = Strings::One(
        fixture_path.to_str()
            .expect("fixture_path should be UTF-8")
            .to_owned());

    let check_config = init_check_config(reporter, &root, &config)
        .expect("init_check_config");

    Project {
        check_config,
        path: ProjectPath { root },
        config,
        cache: LoadCache::new(),
    }
}
