use object::ObjectGraph;
use ast::SourceFile;
use top_level;
use std::io;

pub struct Environment {
    pub object: ObjectGraph,
}

static STDLIB_DEFINITIONS: &'static str = include_str!("../definitions/stdlib.rb");

impl Environment {
    pub fn new() -> Environment {
        let env = Environment {
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
