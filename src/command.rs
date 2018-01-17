use std::env;
use std::path::PathBuf;

use termcolor::StandardStream;
use typed_arena::Arena;

use annotate::{self, AnnotateError};
use config::{AnnotateConfig, CheckConfig, StripConfig};
use environment::Environment;
use errors::{ErrorReporter, ErrorSink};
use load::LoadCache;
use remote::server::RunServerError;
use remote;
use strip::{self, StripError};

pub fn check(mut errors: ErrorReporter<StandardStream>, mut config: CheckConfig, files: Vec<PathBuf>) -> bool {
    if let Some(lib_path) = env::var("TYPEDRUBY_LIB").ok() {
        config.require_paths.insert(0, PathBuf::from(lib_path));
    } else {
        errors.warning("TYPEDRUBY_LIB environment variable not set, will not use builtin standard library definitions", &[]);
    }

    let socket_path = remote::socket_path().expect("server::socket_path");

    if socket_path.exists() {
        match remote::client::check_remote(&socket_path, &mut errors, config, files) {
            Ok(result) => result,
            Err(e) => panic!("client::check_remote: {:?}", e),
        }
    } else {
        let load_cache = LoadCache::new();
        let arena = Arena::new();

        let env = Environment::new(&arena, &load_cache, &mut errors, config);

        env.load_files(files.iter());
        env.define();
        env.typecheck();

        let errors = env.error_sink.borrow();

        errors.error_count() == 0 && errors.warning_count() == 0
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
    match remote::server::run() {
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
