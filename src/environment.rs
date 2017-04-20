use std::io;

use ast::SourceFile;
use errors::ErrorSink;
use object::ObjectGraph;
use top_level;

pub struct Environment<'a> {
    pub error_sink: &'a mut ErrorSink,
    pub object: ObjectGraph,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/stdlib.rb");

impl<'a> Environment<'a> {
    pub fn new(error_sink: &'a mut ErrorSink) -> Environment {
        let env = Environment {
            error_sink: error_sink,
            object: ObjectGraph::new(),
        };

        let source_file = SourceFile::new("(builtin stdlib)".to_owned(), STDLIB_DEFINITIONS.to_owned());

        top_level::evaluate(&env, &source_file);

        env
    }

    pub fn load_file(&self, filename: String) -> io::Result<()> {
        let source_file = match SourceFile::open(filename) {
            Ok(sf) => sf,
            Err(err) => return Err(err),
        };

        top_level::evaluate(self, &source_file);

        Ok(())
    }
}
