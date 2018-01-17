use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use toml;

use config::ProjectConfig;
use load::LoadCache;

const CONFIG_FILE: &'static str = "TypedRuby.toml";
const SOCKET_FILE: &'static str = ".typedruby.sock";

pub struct Project {
    pub cache: LoadCache,
    root: PathBuf,
    pub config: ProjectConfig,
}

#[derive(Debug)]
pub enum ProjectError {
    Io(io::Error),
    Toml(toml::de::Error),
}

fn find_typedruby_toml(initial_dir: &Path) -> PathBuf {
    let mut dir = initial_dir.to_owned();
    dir.push(CONFIG_FILE);

    loop {
        let config = dir.with_file_name(CONFIG_FILE);

        if config.exists() {
            return config;
        }

        if !dir.pop() {
            break;
        }
    }

    // if we couldn't find a directory with a TypedRuby.toml, just use the
    // initial dir that was passed in:
    initial_dir.to_owned()
}

fn read_typedruby_toml(path: &Path) -> Result<ProjectConfig, ProjectError> {
    let mut buff = String::new();

    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buff))
        .map_err(ProjectError::Io)?;

    toml::from_str(&buff).map_err(ProjectError::Toml)
}

impl Project {
    pub fn find(dir: &Path) -> Result<Project, ProjectError> {
        let config_path = find_typedruby_toml(dir);

        let config = read_typedruby_toml(&config_path)?;

        let root = {
            let mut root = config_path;
            root.pop();
            root
        };

        Ok(Project {
            root,
            config,
            cache: LoadCache::new(),
        })
    }

    pub fn socket_path(&self) -> PathBuf {
        self.root.with_file_name(SOCKET_FILE)
    }
}

