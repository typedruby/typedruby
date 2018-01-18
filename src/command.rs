use std::env;
use std::path::PathBuf;

use termcolor::StandardStream;
use typed_arena::Arena;

use annotate::{self, AnnotateError};
use config::{AnnotateConfig, StripConfig};
use environment::Environment;
use errors::{ErrorReporter, ErrorSink};
use project::{Project, ProjectError};
use remote::server::RunServerError;
use remote::client::{Remote, ConnectError};
use remote;
use strip::{self, StripError};

pub fn check(mut errors: ErrorReporter<StandardStream>) -> bool {
    let project = match Project::find(&mut errors, &env::current_dir().expect("env::current_dir")) {
        Ok(project) => project,
        Err(ProjectError::Toml(e)) => {
            // TODO use Loc stuff to pinpoint error in TypedRuby.toml
            errors.error(&format!("Couldn't parse TypedRuby.toml: {:?}", e), &[]);
            return false;
        }
        Err(e) => {
            errors.error(&format!("Couldn't load project: {:?}", e), &[]);
            return false;
        }
    };

    match Remote::connect(&project.socket_path()) {
        Ok(mut remote) => {
            match remote.check(&mut errors) {
                Ok(result) => result,
                Err(e) => {
                    errors.error(&format!("Error communicating with TypedRuby server: {:?}", e), &[]);
                    false
                }
            }
        }
        Err(ConnectError::Io(e)) => {
            errors.error(&format!("Could not connect to TypedRuby server: {:?}", e), &[]);
            false
        }
        Err(ConnectError::VersionMismatch(version)) => {
            errors.error(&format!("TypedRuby server is running version {}, expected {}", version, remote::protocol::VERSION), &[]);
            false
        }
        Err(ConnectError::Protocol(e)) => {
            errors.error(&format!("Error communicating with TypedRuby server: {:?}", e), &[]);
            false
        }
        Err(ConnectError::NoServer) => {
            let arena = Arena::new();

            let env = Environment::new(&arena, &project, &mut errors);

            env.load_files(project.check_config.files.iter());
            env.define();
            env.typecheck();

            let errors = env.error_sink.borrow();

            errors.error_count() == 0 && errors.warning_count() == 0
        }
    }
}

pub fn annotate(mut errors: ErrorReporter<StandardStream>, config: AnnotateConfig, file: PathBuf) -> bool {
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

pub fn strip(mut errors: ErrorReporter<StandardStream>, config: StripConfig, files: Vec<PathBuf>) -> bool {
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

pub fn server(errors: &mut ErrorSink) -> bool {
    match remote::server::run(errors) {
        Ok(()) => true,
        Err(RunServerError::AlreadyRunning(path)) => {
            errors.error(&format!("A TypedRuby server is already running on {}", path.display()), &[]);
            false
        }
        Err(e) => {
            errors.error(&format!("Could not start server: {:?}", e), &[]);
            false
        }
    }
}
