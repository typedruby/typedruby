use std::io;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};
use std::fs;

use typed_arena::Arena;

use ast::{Ast, SourceFile, Node, Id};
use config::CheckConfig;
use define::Definitions;
use errors::ErrorSink;
use inflect::Inflector;
use load::LoadCache;
use object::{ObjectGraph, RubyObject, MethodEntry, Scope, ConstantEntry};
use top_level;
use typecheck;

enum LoadState {
    Loading,
    Loaded,
}

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
enum Phase {
    Load,
    Define,
    TypeCheck,
}

impl Phase {
    pub fn can_load(self) -> bool {
        match self {
            Phase::Load => true,
            Phase::Define => false,
            Phase::TypeCheck => false,
        }
    }
}

struct PhaseCell(Cell<Phase>);

impl PhaseCell {
    pub fn new(phase: Phase) -> Self {
        PhaseCell(Cell::new(phase))
    }

    pub fn get(&self) -> Phase {
        self.0.get()
    }

    pub fn set(&self, phase: Phase) {
        let current = self.0.get();

        match (current, phase) {
            (Phase::Load, Phase::Define) |
            (Phase::Define, Phase::TypeCheck) => {
                self.0.set(phase)
            }
            _ => panic!("invalid phase transition! {:?} -> {:?}", current, phase)
        }
    }
}

pub struct Environment<'object> {
    pub object: ObjectGraph<'object>,
    pub error_sink: RefCell<&'object mut ErrorSink>,
    pub config: CheckConfig,
    pub defs: Definitions<'object>,
    phase: PhaseCell,
    loaded_features: RefCell<HashMap<PathBuf, LoadState>>,
    method_queue: RefCell<VecDeque<Rc<MethodEntry<'object>>>>,
    inflector: Inflector,
    load_cache: &'object LoadCache,
}

impl<'object> Environment<'object> {
    pub fn new(arena: &'object Arena<RubyObject<'object>>, load_cache: &'object LoadCache, error_sink: &'object mut ErrorSink, config: CheckConfig) -> Environment<'object> {
        let inflector = Inflector::new(&config.inflect_acronyms);

        let env = Environment {
            error_sink: RefCell::new(error_sink),
            object: ObjectGraph::new(&arena),
            config: config,
            phase: PhaseCell::new(Phase::Load),
            loaded_features: RefCell::new(HashMap::new()),
            method_queue: RefCell::new(VecDeque::new()),
            inflector: inflector,
            defs: Definitions::new(),
            load_cache: load_cache,
        };

        top_level::evaluate(&env, load_cache.builtin_stdlib());

        env
    }

    pub fn load_file(&self, path: &Path) -> io::Result<()> {
        let ast = self.load_cache.load_ast(path)?;

        self.load_ast(&ast, path);

        Ok(())
    }

    pub fn should_emit_errors(&self, path: &Path) -> bool {
        !self.config.ignore_errors_in.iter().any(|prefix| path.starts_with(prefix))
    }

    fn load_ast(&self, ast: &Ast, path: &Path) {
        if !self.phase.get().can_load() {
            panic!("tried to load file in {:?} phase: {}", self.phase.get(), path.display());
        }

        for diag in &ast.diagnostics {
            self.error_sink.borrow_mut().parser_diagnostic(diag);
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
            Ok(_) => { loaded_features_ref.insert(path, LoadState::Loaded); }
            Err(_) => { loaded_features_ref.remove(&path); }
        }

        result
    }

    fn search_paths<P: AsRef<Path>>(file: &str, paths: &[P]) -> Option<PathBuf> {
        let has_ext = file.rsplit('/').next()
            .map(|ext| ext.contains('.'))
            .unwrap_or(false);

        let exts_for_file = if has_ext {
            static NO_EXTS: &[&str] = &[""];
            NO_EXTS
        } else {
            static REQUIRE_EXTS: &[&str] = &[".typed.rb", ".rb"];
            REQUIRE_EXTS
        };

        for path in paths {
            for ext in exts_for_file {
                let resolved = path.as_ref().join(file.to_owned() + ext);

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

    pub fn search_autoload_path(&self, file: &str) -> Option<PathBuf> {
        Self::search_paths(file, &self.config.autoload_paths)
    }

    pub fn search_relative_path(&self, file: &str, from: &SourceFile) -> Option<PathBuf> {
        from.filename().parent().and_then(|parent| {
            Self::search_paths(file, &[parent])
        })
    }

    pub fn autoload(&self, module: &'object RubyObject<'object>, name: &str) -> Option<Rc<ConstantEntry<'object>>> {
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

                return self.object.get_const(module, name);
            }
        }

        // search for directories and autodefine modules:
        for autoload_path in &self.config.autoload_paths {
            let resolved = autoload_path.join(&path);

            if resolved.is_dir() {
                let module = self.object.define_module(None, module, name, vec![]);
                return Some(Rc::new(ConstantEntry::Module { loc: None, value: module }));
            }
        }

        None
    }

    pub fn load_files<I, P>(&self, files_iter: I)
        where I: Iterator<Item = P>, P: AsRef<Path>
    {
        for file in files_iter {
            let file = file.as_ref();

            match self.require(file) {
                Ok(()) => {}
                Err(e) => {
                    self.error_sink.borrow_mut()
                        .error(&format!("{}: {}", file.display(), e), &[]);
                }
            }
        }

        self.defs.autoload_const_references(self);
    }

    pub fn define(&self) {
        self.phase.set(Phase::Define);

        let methods = self.defs.define(&self);

        let mut method_queue = self.method_queue.borrow_mut();

        for method in methods {
            method_queue.push_back(method);
        }
    }

    pub fn typecheck(&self) {
        self.phase.set(Phase::TypeCheck);

        while let Some(method) = self.method_queue.borrow_mut().pop_front() {
            typecheck::check(self, method);
        }
    }

    pub fn resolve_cbase<'node>(&self, node: &'node Node, scope: Rc<Scope<'object>>)
        -> Result<&'object RubyObject<'object>, (&'node Node, &'static str)>
    {
        match *node {
            Node::Cbase(_) => Ok(Scope::root(&scope).module),
            Node::Const(..) =>
                self.resolve_cpath(node, scope)
                    .and_then(|constant| {
                        match *constant {
                            ConstantEntry::Module { value, .. } => Ok(value),
                            ConstantEntry::Expression { .. } =>
                                Err((node, "Not a static class/module")),
                        }}),
            _ => Err((node, "Not a static constant path")),
        }
    }

    pub fn resolve_cpath<'node>(&self, node: &'node Node, scope: Rc<Scope<'object>>)
        -> Result<Rc<ConstantEntry<'object>>, (&'node Node, &'static str)>
    {
        match *node {
            Node::Const(_, Some(ref base), Id(_, ref name)) => {
                self.resolve_cbase(base, scope)
                    .and_then(|value|
                        self.object.get_const(value, name)
                            .map(Ok)
                            .unwrap_or_else(||
                                self.autoload(value, name).ok_or(
                                    (node, "No such constant"))))
            },

            Node::Const(_, None, Id(_, ref name)) => {
                for scope in Scope::ancestors(&scope) {
                    if let Some(ce) = self.object.get_const(scope.module, name) {
                        return Ok(ce);
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
