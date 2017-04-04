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
pub enum Node {
    Begin   (ExprLoc,   Vec<Box<Node>>),
    Integer (ExprLoc,   String),
    String  (ExprLoc,   String),
    Send    (SendLoc,   Box<Node>, String, Vec<Box<Node>>),
}

impl Node {
    pub fn loc(&self) -> &Loc {
        match self {
            &Node::Begin(ref loc, _) => loc,
            &Node::Integer(ref loc, _) => loc,
            &Node::String(ref loc, _) => loc,
            &Node::Send(ref loc, _, _, _) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub filename: String,
    pub node: Option<Box<Node>>,
}
