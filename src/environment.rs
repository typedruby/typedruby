use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};

use typed_arena::Arena;

use ast::{parse, SourceFile, Node, Id};
use errors::ErrorSink;
use object::{ObjectGraph, RubyObject, MethodEntry};
use top_level;
use config::Config;
use typecheck;

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
    method_queue: RefCell<VecDeque<Rc<MethodEntry<'object>>>>,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/stdlib.rb");

impl<'object> Environment<'object> {
    pub fn new(arena: &'object Arena<RubyObject<'object>>, error_sink: Box<ErrorSink>, config: Config) -> Environment<'object> {
        let env = Environment {
            arena: arena,
            error_sink: RefCell::new(error_sink),
            object: ObjectGraph::new(&arena),
            config: config,
            loaded_features: RefCell::new(HashMap::new()),
            method_queue: RefCell::new(VecDeque::new()),
        };

        let source_file = SourceFile::new(PathBuf::from("(builtin stdlib)"), STDLIB_DEFINITIONS.to_owned());

        env.load_source_file(source_file);

        env
    }

    pub fn load_file(&self, path: &Path) -> io::Result<()> {
        let source_file = match SourceFile::open(path.to_owned()) {
            Ok(sf) => sf,
            Err(err) => return Err(err),
        };

        self.load_source_file(source_file);

        Ok(())
    }

    fn load_source_file(&self, source_file: SourceFile) {
        let ast = parse(Rc::new(source_file));

        if let Some(ref node) = ast.node {
            top_level::evaluate(self, node.clone());
        }
    }

    pub fn require(&self, path: &Path) -> io::Result<()> {
        {
            let loaded_features_ref = self.loaded_features.borrow();

            match loaded_features_ref.get(path) {
                None => {},
                Some(&LoadState::Loading) => {
                    // circular require, pass for now
                    return Ok(());
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

        match result {
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

    pub fn enqueue_method_for_type_check(&self, method: Rc<MethodEntry<'object>>) {
        self.method_queue.borrow_mut().push_back(method);
    }

    pub fn typecheck(&self) {
        while let Some(method) = self.method_queue.borrow_mut().pop_front() {
            typecheck::check(self, method);
        }
    }
}
