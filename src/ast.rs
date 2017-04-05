use std::vec::Vec;
use std::cmp::{min, max};

#[derive(Debug,Clone)]
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
    Arg             (ExprLoc,   String),
    Args            (ExprLoc,   Vec<Box<Node>>),
    Begin           (ExprLoc,   Vec<Box<Node>>),
    Const           (ConstLoc,  Option<Box<Node>>, String),
    CSend           (SendLoc,   Option<Box<Node>>, String, Vec<Box<Node>>),
    Def             (ExprLoc,   String, Option<Box<Node>>, Option<Box<Node>>),
    EncodingLiteral (ExprLoc),
    Ensure          (ExprLoc,   Option<Box<Node>>, Box<Node>),
    False           (ExprLoc),
    FileLiteral     (ExprLoc),
    Integer         (ExprLoc,   String),
    LineLiteral     (ExprLoc),
    Lvar            (ExprLoc,   String),
    Nil             (ExprLoc),
    Rescue          (ExprLoc,   Option<Box<Node>>, Vec<Box<Node>>, Option<Box<Node>>),
    Self_           (ExprLoc),
    Send            (SendLoc,   Option<Box<Node>>, String, Vec<Box<Node>>),
    String          (ExprLoc,   String),
    True            (ExprLoc),
}

impl Node {
    pub fn loc(&self) -> &Loc {
        match self {
            &Node::Arg(ref loc, _) => loc,
            &Node::Args(ref loc, _) => loc,
            &Node::Begin(ref loc, _) => loc,
            &Node::Const(ref loc, _, _) => loc,
            &Node::CSend(ref loc, _, _, _) => loc,
            &Node::Def(ref loc, _, _, _) => loc,
            &Node::EncodingLiteral(ref loc) => loc,
            &Node::Ensure(ref loc, _, _) => loc,
            &Node::False(ref loc) => loc,
            &Node::FileLiteral(ref loc) => loc,
            &Node::Integer(ref loc, _) => loc,
            &Node::LineLiteral(ref loc) => loc,
            &Node::Lvar(ref loc, _) => loc,
            &Node::Nil(ref loc) => loc,
            &Node::Rescue(ref loc, _, _, _) => loc,
            &Node::Self_(ref loc) => loc,
            &Node::Send(ref loc, _, _, _) => loc,
            &Node::String(ref loc, _) => loc,
            &Node::True(ref loc) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub filename: String,
    pub node: Option<Box<Node>>,
}
