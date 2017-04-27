use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use typed_arena::Arena;

use ast::{SourceFile};
use errors::ErrorSink;
use object::{ObjectGraph, RubyObject};
use top_level;
use config::Config;

pub struct Environment<'object> {
    arena: &'object Arena<RubyObject<'object>>,
    pub object: ObjectGraph<'object>,
    pub error_sink: RefCell<Box<ErrorSink>>,
    pub config: Config,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/stdlib.rb");

impl<'object> Environment<'object> {
    pub fn new(arena: &'object Arena<RubyObject<'object>>, error_sink: Box<ErrorSink>, config: Config) -> Environment<'object> {
        let mut env = Environment {
            arena: arena,
            error_sink: RefCell::new(error_sink),
            object: ObjectGraph::new(&arena),
            config: config,
        };

        let source_file = SourceFile::new("(builtin stdlib)".to_owned(), STDLIB_DEFINITIONS.to_owned());

        top_level::evaluate(&env, Rc::new(source_file));

        env
    }

    pub fn load_file<'env>(&'env self, filename: String) -> io::Result<()> {
        let source_file = match SourceFile::open(filename) {
            Ok(sf) => sf,
            Err(err) => return Err(err),
        };

        top_level::evaluate(self, Rc::new(source_file));

        Ok(())
    }
}
