use std::env;
use std::path::PathBuf;

use typed_arena::Arena;

use annotate::{self, AnnotateError};
use config::{AnnotateConfig, StripConfig};
use environment::Environment;
use project::{Project, ProjectPath, ProjectError};
use remote::server::RunServerError;
use remote::client::{Remote, ConnectError};
use remote;
use report::Reporter;
use strip::{self, StripError};

pub fn check(reporter: &mut Reporter) -> bool {
    let project_path = match ProjectPath::find(env::current_dir().expect("env::current_dir")) {
        Some(path) => path,
        None => {
            reporter.error(&format!("Couldn't find TypedRuby.toml"), &[]);
            return false;
        }
    };

    match Remote::connect(&project_path.socket_path()) {
        Ok(mut remote) => {
            match remote.check(reporter) {
                Ok(result) => result,
                Err(e) => {
                    reporter.error(&format!("Error communicating with TypedRuby server: {:?}", e), &[]);
                    false
                }
            }
        }
        Err(ConnectError::Io(e)) => {
            reporter.error(&format!("Could not connect to TypedRuby server: {:?}", e), &[]);
            false
        }
        Err(ConnectError::VersionMismatch(version)) => {
            reporter.error(&format!("TypedRuby server is running version {}, expected {}", version, remote::protocol::VERSION), &[]);
            false
        }
        Err(ConnectError::Protocol(e)) => {
            reporter.error(&format!("Error communicating with TypedRuby server: {:?}", e), &[]);
            false
        }
        Err(ConnectError::NoServer) => {
            let project = match Project::new(reporter, project_path) {
                Ok(project) => project,
                Err(ProjectError::Toml(e)) => {
                    // TODO use Loc stuff to pinpoint error in TypedRuby.toml
                    reporter.error(&format!("Couldn't parse TypedRuby.toml: {:?}", e), &[]);
                    return false;
                }
                Err(e) => {
                    reporter.error(&format!("Couldn't load project: {:?}", e), &[]);
                    return false;
                }
            };

            let arena = Arena::new();
            Environment::new(&arena, &project, reporter).run()
        }
    }
}

pub fn annotate(reporter: &mut Reporter, config: AnnotateConfig, file: PathBuf) -> bool {
    match annotate::apply_annotations(&file, config) {
        Ok(()) => true,
        Err(err) => {
            match err {
                AnnotateError::Syntax(diagnostics) => {
                    for diagnostic in diagnostics {
                        reporter.parser_diagnostic(&diagnostic);
                    }
                }
                AnnotateError::Json(err) => {
                    reporter.error(&format!("Could not parse line of annotations file: {}", err), &[]);
                }
                AnnotateError::Io(err) => {
                    reporter.error(&format!("Could not open {}: {}", file.display(), err), &[]);
                }
            }

            false
        }
    }
}

pub fn strip(reporter: &mut Reporter, config: StripConfig, files: Vec<PathBuf>) -> bool {
    let mut success = true;

    for file in files {
        match strip::strip_file(file.clone(), &config) {
            Ok(()) => {},
            Err(err) => {
                success = false;

                match err {
                    StripError::Syntax(diagnostics) => {
                        for diagnostic in diagnostics {
                            reporter.parser_diagnostic(&diagnostic);
                        }
                    }
                    StripError::Io(err) => {
                        reporter.error(&format!("Could not open {}: {}", file.display(), err), &[]);
                    }
                }
            }
        }
    }

    success
}

pub fn server(reporter: &mut Reporter) -> bool {
    match remote::server::run(reporter) {
        Ok(()) => true,
        Err(RunServerError::AlreadyRunning(path)) => {
            reporter.error(&format!("A TypedRuby server is already running on {}", path.display()), &[]);
            false
        }
        Err(e) => {
            reporter.error(&format!("Could not start server: {:?}", e), &[]);
            false
        }
    }
}
