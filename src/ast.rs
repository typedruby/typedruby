extern crate ruby_parser;

pub use self::ruby_parser::{parse, Ast, SourceFile, Id, Node, Loc, Diagnostic, Level};

use std::rc::Rc;

pub trait IntoNode<'a> {
    fn into_node(self) -> Option<&'a Node>;
}

impl<'a> IntoNode<'a> for &'a Rc<Node> {
    fn into_node(self) -> Option<&'a Node> {
        Some(self.as_ref())
    }
}

impl<'a> IntoNode<'a> for &'a Option<Rc<Node>> {
    fn into_node(self) -> Option<&'a Node> {
        self.as_ref().map(Rc::as_ref)
    }
}
