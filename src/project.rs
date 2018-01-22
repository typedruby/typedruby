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

fn file_needs_refresh(path: &Path, cached_mtime: SystemTime) -> bool {
    let current_mtime = fs::metadata(path)
        .and_then(|meta| meta.modified());

    match current_mtime {
        Ok(current_mtime) => current_mtime > cached_mtime,
        Err(_) => {
            // if we can't fetch the mtime of the file then always refresh:
            true
        }
    }
}

struct Refreshable<T> {
    inner: T,
    files: Vec<(PathBuf, Option<SystemTime>)>,
}

impl<T> Refreshable<T> {
    pub fn new(inner: T, files: &[PathBuf]) -> Self {
        let files = files.iter()
            .map(|path|
                (path.to_owned(),
                    fs::metadata(path)
                        .and_then(|meta| meta.modified())
                        .ok()))
            .collect();

        Refreshable { inner, files }
    }

    pub fn needs_refresh(&self) -> bool {
        self.files.iter().any(|&(ref path, mtime)|
            match mtime {
                Some(mtime) => file_needs_refresh(path, mtime),
                None => true,
            })
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn map<F, U>(self, f: F) -> Refreshable<U>
        where F: FnOnce(T) -> U
    {
        Refreshable {
            inner: f(self.inner),
            files: self.files,
        }
    }
}

pub struct Project {
    pub cache: LoadCache,
    pub path: ProjectPath,
    pub config: ProjectConfig,
    check_config: Refreshable<CheckConfig>,
    codegen: Refreshable<()>,
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

enum BundlerStrategy<'a> {
    Default,
    Custom(&'a Command),
}

fn interpret_bundler_config<'a>(config: &'a BundlerConfig) -> Option<BundlerStrategy<'a>> {
    match *config {
        BundlerConfig { enabled: Some(false), .. } |
        BundlerConfig { enabled: None, exec: None, .. } =>
            None,
        BundlerConfig { enabled: Some(true), exec: None, .. } =>
            Some(BundlerStrategy::Default),
        BundlerConfig { exec: Some(ref cmd), .. } =>
            Some(BundlerStrategy::Custom(cmd)),
    }
}

fn load_bundler_load_paths(reporter: &mut Reporter, project_root: &Path, config: &BundlerConfig) -> Result<Refreshable<Vec<PathBuf>>, ProjectError> {
    let (mut command, refresh) = match interpret_bundler_config(config) {
        None => {
            return Ok(Refreshable::new(Vec::new(), &[]));
        }
        Some(BundlerStrategy::Default) => {
            let mut command = prepare_command(project_root, "ruby");
            command.args(&["-r", "bundler/setup", "-e", "puts $LOAD_PATH"]);

            let mut refresh = paths_from_strings(&config.refresh)?;
            refresh.push(project_root.join("Gemfile.lock"));

            (command, refresh)
        }
        Some(BundlerStrategy::Custom(cmd)) => {
            (prepare_config_command(project_root, cmd), paths_from_strings(&config.refresh)?)
        }
    };

    reporter.info("Loading Gem environment...");

    let output = command.output().map_err(ProjectError::Io)?;

    if output.status.success() {
        let paths = output.stdout
            .split(|c| *c == ('\r' as u8) || *c == ('\n' as u8))
            .filter_map(|line| str::from_utf8(line).ok())
            .map(PathBuf::from)
            .collect();

        Ok(Refreshable::new(paths, &refresh))
    } else {
        reporter.error("Command exited with non-zero status", &[]);
        Err(ProjectError::Bundler(CommandError::NonZeroStatus))
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

fn setup_require_paths(reporter: &mut Reporter, project_root: &Path, config: &ProjectConfig)
    -> Result<Refreshable<Vec<PathBuf>>, ProjectError>
{
    let mut require_paths = Vec::new();

    require_paths.extend(
        paths_from_strings(&config.typedruby.load_path)?);

    if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
        require_paths.push(PathBuf::from(lib_path));
    } else {
        reporter.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
    }

    Ok(load_bundler_load_paths(reporter, project_root, &config.bundler)?
        .map(|paths| {
            require_paths.extend(paths);
            require_paths
        }))
}

fn setup_check_config(reporter: &mut Reporter, project_root: &Path, config: &ProjectConfig)
    -> Result<Refreshable<CheckConfig>, ProjectError>
{
    let require_paths = setup_require_paths(reporter, project_root, config)?;
    let autoload_paths = paths_from_strings(&config.typedruby.autoload_path)?;
    let ignore_errors_in = paths_from_strings(&config.typedruby.ignore_errors)?;
    let files = paths_from_strings(&config.typedruby.source)?;
    let inflect_acronyms = config.inflect.acronyms.as_slice().to_vec();

    Ok(require_paths.map(|require_paths| CheckConfig {
        warning: false,
        require_paths,
        autoload_paths,
        ignore_errors_in,
        inflect_acronyms,
        files,
    }))
}

fn codegen(reporter: &mut Reporter, project_root: &Path, config: &ProjectConfig) -> Result<Refreshable<()>, ProjectError> {
    let cmd = match config.codegen.exec {
        Some(ref cmd) => cmd,
        None => { return Ok(Refreshable::new((), &[])) }
    };

    reporter.info("Generating code...");

    match prepare_config_command(project_root, cmd).status() {
        Ok(stat) if stat.success() => Ok(Refreshable::new((), &paths_from_strings(&config.codegen.refresh)?)),
        Ok(_) => {
            reporter.error("Command exited with non-zero status", &[]);
            Err(ProjectError::Codegen(CommandError::NonZeroStatus))
        }
        Err(e) => {
            reporter.error("Could not invoke codegen command", &[]);
            Err(ProjectError::Codegen(CommandError::Io(e)))
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

        let check_config = setup_check_config(reporter, path.root(), &config)?;

        let codegen = codegen(reporter, path.root(), &config)?;

        let project = Project {
            check_config,
            path,
            config,
            config_mtime,
            codegen,
            cache: LoadCache::new(),
        };

        Ok(project)
    }

    pub fn needs_reload(&self) -> bool {
        file_needs_refresh(&self.path.config_path(), self.config_mtime)
    }

    pub fn refresh(&mut self, reporter: &mut Reporter) -> Result<(), ProjectError> {
        if self.check_config.needs_refresh() {
            self.check_config = setup_check_config(reporter, self.path.root(), &self.config)?;
        }

        if self.codegen.needs_refresh() {
            self.codegen = codegen(reporter, self.path.root(), &self.config)?;
        }

        Ok(())
    }

    pub fn check_config(&self) -> &CheckConfig {
        self.check_config.inner()
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

    let check_config = setup_check_config(reporter, &root, &config)
        .expect("init_check_config");

    Project {
        check_config,
        path: ProjectPath { root },
        config,
        config_mtime: SystemTime::now(),
        codegen: Refreshable::new((), &[]),
        cache: LoadCache::new(),
    }
}
