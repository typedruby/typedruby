extern crate ruby_parser;

use std::rc::Rc;

pub use self::ruby_parser::SourceFile;
pub use self::ruby_parser::ast::{Id, Node, Loc};

pub struct SourceLoc {
    pub file: Rc<SourceFile>,
    pub loc: Loc,
}
