use ast::*;
use ffi::Driver;
use std::rc::Rc;

#[derive(Copy,Clone)]
pub enum ParserMode {
    Program,
    Prototype,
}

pub struct ParserOptions<'a> {
    pub emit_file_vars_as_literals: bool,
    pub emit_lambda: bool,
    pub emit_procarg0: bool,
    pub declare_env: &'a [&'a str],
    pub mode: ParserMode,
}

impl<'a> ParserOptions<'a> {
    fn defaults() -> Self {
        ParserOptions {
            emit_file_vars_as_literals: false,
            emit_lambda: true,
            emit_procarg0: true,
            declare_env: &[],
            mode: ParserMode::Program,
        }
    }
}

pub fn parse(source_file: Rc<SourceFile>) -> Ast {
    parse_with_opts(SourceRef::Entire { file: source_file }, ParserOptions::defaults())
}

pub fn parse_with_opts(source_file: SourceRef, opts: ParserOptions) -> Ast {
    let mut driver = Driver::new(opts, source_file);
    let node = driver.parse();

    Ast {
        node: node,
        diagnostics: driver.diagnostics(),
    }
}
