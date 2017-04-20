use object::ObjectGraph;
use ruby_parser::SourceFile;
use top_level;

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
}
