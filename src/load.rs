use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use std::time::SystemTime;

use ast::{self, Ast, SourceFile, Node};

struct CacheEntry {
    mtime: SystemTime,
    ast: Rc<Ast>,
}

type Cache = HashMap<PathBuf, Rc<CacheEntry>>;

pub struct LoadCache {
    cache: RefCell<Cache>,
    builtin_stdlib: Rc<Node>,
}

fn load_stdlib() -> Rc<Node> {
    const STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/core.rb");

    let source_file = Rc::new(SourceFile::new(PathBuf::from("(builtin stdlib)"), STDLIB_DEFINITIONS.to_owned()));
    let ast = Rc::new(ast::parse(source_file));

    Rc::clone(ast.node.as_ref().expect("builtin stdlib AST to not be empty"))
}

fn load(cache: &mut Cache, path: &Path) -> Result<Rc<CacheEntry>, io::Error> {
    let mtime = fs::metadata(path)?.modified()?;

    if let Some(ent) = cache.get(path) {
        if ent.mtime == mtime {
            return Ok(Rc::clone(ent));
        }
    }

    let source_file = Rc::new(SourceFile::open(path.to_owned())?);
    let ast = Rc::new(ast::parse(source_file));

    cache.insert(path.to_owned(), Rc::new(CacheEntry {
        mtime: mtime,
        ast: ast,
    }));

    Ok(cache.get(path).cloned().expect("just inserted key should exist"))
}

impl LoadCache {
    pub fn new() -> Self {
        LoadCache {
            cache: RefCell::new(HashMap::new()),
            builtin_stdlib: load_stdlib(),
        }
    }

    pub fn load_ast(&self, path: &Path) -> Result<Rc<Ast>, io::Error> {
        let mut cache = self.cache.borrow_mut();

        load(&mut cache, path).map(|ent| Rc::clone(&ent.ast))
    }

    pub fn builtin_stdlib(&self) -> Rc<Node> {
        Rc::clone(&self.builtin_stdlib)
    }
}
