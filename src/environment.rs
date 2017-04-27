use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use typed_arena::Arena;

use ast::{SourceFile};
use errors::ErrorSink;
use object::{ObjectGraph, RubyObject};
use top_level;
use config::Config;

enum LoadState {
    Loading,
    Loaded,
}

pub struct Environment<'object> {
    arena: &'object Arena<RubyObject<'object>>,
    pub object: ObjectGraph<'object>,
    pub error_sink: RefCell<Box<ErrorSink>>,
    pub config: Config,
    loaded_features: RefCell<HashMap<PathBuf, LoadState>>,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/stdlib.rb");

impl<'object> Environment<'object> {
    pub fn new(arena: &'object Arena<RubyObject<'object>>, error_sink: Box<ErrorSink>, config: Config) -> Environment<'object> {
        let mut env = Environment {
            arena: arena,
            error_sink: RefCell::new(error_sink),
            object: ObjectGraph::new(&arena),
            config: config,
            loaded_features: RefCell::new(HashMap::new()),
        };

        let source_file = SourceFile::new("(builtin stdlib)".to_owned(), STDLIB_DEFINITIONS.to_owned());

        top_level::evaluate(&env, Rc::new(source_file));

        env
    }

    pub fn load_file<'env>(&'env self, path: &Path) -> io::Result<()> {
        let source_file = match SourceFile::open(path.to_str().unwrap().to_owned()) {
            Ok(sf) => sf,
            Err(err) => return Err(err),
        };

        top_level::evaluate(self, Rc::new(source_file));

        Ok(())
    }

    pub fn require<'env>(&'env self, path: &Path) -> io::Result<()> {
        {
            let loaded_features_ref = self.loaded_features.borrow();

            match loaded_features_ref.get(path) {
                None => {},
                Some(&LoadState::Loading) => {
                    // circular require
                    panic!("circular require")
                },
                Some(&LoadState::Loaded) => {
                    return Ok(());
                },
            }
        }

        {
            let mut loaded_features_ref = self.loaded_features.borrow_mut();
            loaded_features_ref.insert(path.to_owned(), LoadState::Loading);
        }

        let result = self.load_file(path);

        let mut loaded_features_ref = self.loaded_features.borrow_mut();

        match self.load_file(path) {
            Ok(()) => {
                loaded_features_ref.insert(path.to_owned(), LoadState::Loaded);
                Ok(())
            },
            Err(err) => {
                loaded_features_ref.remove(path);
                Err(err)
            },
        }
    }

    pub fn search_require_path(&self, file: &str) -> Option<PathBuf> {
        for path in &self.config.require_paths {
            for ext in &["", ".rb"] {
                let resolved = path.join(file.to_owned() + ext);

                if resolved.is_file() {
                    return Some(resolved)
                }
            }
        }

        None
    }
}
