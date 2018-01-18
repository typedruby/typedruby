use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};
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
    Bundler(CommandError),
    Codegen(CommandError),
}

#[derive(Debug)]
pub enum CommandError {
    NonZeroStatus,
    Io(io::Error),
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

fn load_bundler_load_paths(project_root: &Path, config: &BundlerConfig) -> Result<Vec<PathBuf>, CommandError> {
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

    let output = command.output().map_err(CommandError::Io)?;

    if output.status.success() {
        Ok(output.stdout
            .split(|c| *c == ('\r' as u8) || *c == ('\n' as u8))
            .filter_map(|line| str::from_utf8(line).ok())
            .map(PathBuf::from)
            .collect())
    } else {
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
        load_bundler_load_paths(project_root, &config.bundler)
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

        let check_config = init_check_config(errors, &root, &config)?;

        let mut project = Project {
            check_config,
            root,
            config,
            cache: LoadCache::new(),
        };

        project.codegen().map_err(ProjectError::Codegen)?;

        Ok(project)
    }

    pub fn codegen(&mut self) -> Result<(), CommandError> {
        if let Some(ref cmd) = self.config.codegen.exec {
            match prepare_config_command(&self.root, cmd).status() {
                Ok(stat) if stat.success() => Ok(()),
                Ok(_) => Err(CommandError::NonZeroStatus),
                Err(e) => Err(CommandError::Io(e)),
            }
        } else {
            Ok(())
        }
    }

    pub fn socket_path(&self) -> PathBuf {
        self.root.with_file_name(SOCKET_FILE)
    }
}

#[cfg(test)]
pub fn load_fixture(errors: &mut ErrorSink, fixture_path: &Path) -> Project {
    let root = fixture_path
        .parent()
        .expect("fixture_path.parent()")
        .to_owned();

    let mut config = ProjectConfig::default();

    config.typedruby.source = Strings::One(
        fixture_path.to_str()
            .expect("fixture_path should be UTF-8")
            .to_owned());

    let check_config = init_check_config(errors, &root, &config)
        .expect("init_check_config");

    Project {
        check_config,
        root,
        config,
        cache: LoadCache::new(),
    }
}
