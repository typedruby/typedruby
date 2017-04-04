use std::vec::Vec;
use std::cmp::{min, max};

#[derive(Debug)]
pub struct Range {
    pub begin_pos: usize,
    pub end_pos: usize,
}

impl Range {
    pub fn join(&self, other: &Range) -> Range {
        Range {
            begin_pos: min(self.begin_pos, other.begin_pos),
            end_pos: max(self.end_pos, other.end_pos),
        }
    }
}

pub trait Loc {
    fn expr(&self) -> &Range;
}

#[derive(Debug)]
pub struct ExprLoc {
    pub expr_: Range,
}

impl Loc for ExprLoc {
    fn expr(&self) -> &Range {
        &self.expr_
    }
}

#[derive(Debug)]
pub struct SendLoc {
    pub expr_: Range,
    pub selector: Range,
}

impl Loc for SendLoc {
    fn expr(&self) -> &Range {
        &self.expr_
    }
}

#[derive(Debug)]
pub struct ConstLoc {
    pub expr_: Range,
    pub colon: Option<Range>,
    pub name: Range,
}

impl Loc for ConstLoc {
    fn expr(&self) -> &Range {
        &self.expr_
    }
}

#[derive(Debug)]
pub enum Node {
    Begin           (ExprLoc,   Vec<Box<Node>>),
    Const           (ConstLoc,  Option<Box<Node>>, String),
    EncodingLiteral (ExprLoc),
    FileLiteral     (ExprLoc),
    Ident           (ExprLoc,   String),
    Integer         (ExprLoc,   String),
    LineLiteral     (ExprLoc),
    Send            (SendLoc,   Box<Node>, String, Vec<Box<Node>>),
    String          (ExprLoc,   String),
}

impl Node {
    pub fn loc(&self) -> &Loc {
        match self {
            &Node::Begin(ref loc, _) => loc,
            &Node::Const(ref loc, _, _) => loc,
            &Node::EncodingLiteral(ref loc) => loc,
            &Node::FileLiteral(ref loc) => loc,
            &Node::Ident(ref loc, _) => loc,
            &Node::Integer(ref loc, _) => loc,
            &Node::LineLiteral(ref loc) => loc,
            &Node::Send(ref loc, _, _, _) => loc,
            &Node::String(ref loc, _) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub filename: String,
    pub node: Option<Box<Node>>,
}
