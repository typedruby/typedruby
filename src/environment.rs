use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};
use std::fs;

use typed_arena::Arena;

use ast::{parse, SourceFile, Node, Id, Level};
use config::Config;
use errors::{ErrorSink, Detail};
use inflect::Inflector;
use object::{ObjectGraph, RubyObject, MethodEntry, Scope};
use top_level;
use typecheck;

enum LoadState {
    Loading,
    Loaded,
}

pub struct Environment<'object> {
    pub object: ObjectGraph<'object>,
    pub error_sink: RefCell<Box<ErrorSink>>,
    pub config: Config,
    loaded_features: RefCell<HashMap<PathBuf, LoadState>>,
    method_queue: RefCell<VecDeque<Rc<MethodEntry<'object>>>>,
    inflector: Inflector,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/core.rb");

static REQUIRE_EXTS: &'static [&'static str] = &["", ".rb"];

impl<'object> Environment<'object> {
    pub fn new(arena: &'object Arena<RubyObject<'object>>, error_sink: Box<ErrorSink>, config: Config) -> Environment<'object> {
        let inflector = Inflector::new(&config.inflect_acronyms);

        let env = Environment {
            error_sink: RefCell::new(error_sink),
            object: ObjectGraph::new(&arena),
            config: config,
            loaded_features: RefCell::new(HashMap::new()),
            method_queue: RefCell::new(VecDeque::new()),
            inflector: inflector,
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

    pub fn should_emit_errors(&self, path: &Path) -> bool {
        !self.config.ignore_errors_in.iter().any(|prefix| path.starts_with(prefix))
    }

    fn load_source_file(&self, source_file: SourceFile) {
        let source_file = Rc::new(source_file);
        let ast = parse(source_file.clone());

        for diag in ast.diagnostics {
            match diag.level {
                Level::Note => {},
                Level::Warning => {
                    if self.config.warning {
                        self.error_sink.borrow_mut().warning(&format!("{}", diag), &[
                            Detail::Loc("here", &diag.loc),
                        ]);
                    }
                },
                Level::Error |
                Level::Fatal => {
                    self.error_sink.borrow_mut().error(&format!("{}", diag), &[
                        Detail::Loc("here", &diag.loc),
                    ]);
                }
            }
        }

        if let Some(ref node) = ast.node {
            top_level::evaluate(self, node.clone());
        }
    }

    pub fn require(&self, path: &Path) -> io::Result<()> {
        let path = fs::canonicalize(path)?;

        {
            let loaded_features_ref = self.loaded_features.borrow();

            match loaded_features_ref.get(&path) {
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
            loaded_features_ref.insert(path.clone(), LoadState::Loading);
        }

        let result = self.load_file(&path);

        let mut loaded_features_ref = self.loaded_features.borrow_mut();

        match result {
            Ok(()) => {
                loaded_features_ref.insert(path, LoadState::Loaded);
                Ok(())
            },
            Err(err) => {
                loaded_features_ref.remove(&path);
                Err(err)
            },
        }
    }

    fn search_paths(file: &str, paths: &[PathBuf]) -> Option<PathBuf> {
        for path in paths {
            for ext in REQUIRE_EXTS {
                let resolved = path.join(file.to_owned() + ext);

                if resolved.is_file() {
                    return Some(resolved)
                }
            }
        }

        None
    }

    pub fn search_require_path(&self, file: &str) -> Option<PathBuf> {
        Self::search_paths(file, &self.config.require_paths)
    }

    pub fn search_relative_path(&self, file: &str, from: &SourceFile) -> Option<PathBuf> {
        from.filename().parent().and_then(|parent|
            REQUIRE_EXTS.iter().map(|ext|
                parent.join(file.to_owned() + ext)
            ).find(|path|
                path.is_file()
            )
        )
    }

    pub fn autoload(&self, module: &'object RubyObject<'object>, name: &str) -> Option<&'object RubyObject<'object>> {
        if self.config.autoload_paths.is_empty() {
            return None;
        }

        let constant_path = self.object.constant_path(module, name);

        let path = self.inflector.underscore(&constant_path);

        let path_rb = path.clone() + ".rb";

        // search for ruby files first:
        for autoload_path in &self.config.autoload_paths {
            let resolved = autoload_path.join(&path_rb);

            if resolved.is_file() {
                // TODO do something with the potential IO error:
                let _ = self.require(&resolved);

                return self.object.get_const(module, name).map(|ce| ce.value);
            }
        }

        // search for directories and autodefine modules:
        for autoload_path in &self.config.autoload_paths {
            let resolved = autoload_path.join(&path);

            if resolved.is_dir() {
                return Some(self.object.define_module(None, module, name));
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

    pub fn resolve_cpath<'node>(&self, node: &'node Node, scope: Rc<Scope<'object>>) -> Result<&'object RubyObject<'object>, (&'node Node, &'static str)> {
        match *node {
            Node::Cbase(_) =>
                Ok(Scope::root(&scope).module),

            Node::Const(_, Some(ref base), Id(_, ref name)) => {
                match self.resolve_cpath(base, scope) {
                    Ok(&RubyObject::Object { .. }) => Err((base, "Not a class or module")),
                    Ok(&RubyObject::IClass { .. }) => panic!(),
                    Ok(base_ref) =>
                        self.object.get_const(base_ref, name)
                            .map(|ce| Ok(ce.value)).unwrap_or_else(||
                                self.autoload(base_ref, name).ok_or(
                                    (node, "No such constant"))),
                    error => error,
                }
            },

            Node::Const(_, None, Id(_, ref name)) => {
                for scope in Scope::ancestors(&scope) {
                    if let Some(ce) = self.object.get_const(scope.module, name) {
                        return Ok(ce.value);
                    }
                }

                for scope in Scope::ancestors(&scope) {
                    if let Some(obj) = self.autoload(scope.module, name) {
                        return Ok(obj);
                    }
                }

                Err((node, "No such constant"))
            }

            _ =>
                Err((node, "Not a static constant path")),
        }
    }
}
