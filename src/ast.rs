use std::vec::Vec;
use std::cmp::{min, max};

#[derive(Debug,Clone)]
pub struct Loc {
    pub begin_pos: usize,
    pub end_pos: usize,
}

impl Loc {
    pub fn join(&self, other: &Loc) -> Loc {
        Loc {
            begin_pos: min(self.begin_pos, other.begin_pos),
            end_pos: max(self.end_pos, other.end_pos),
        }
    }
}

#[derive(Debug)]
pub struct Id(pub Loc, pub String);

#[derive(Debug)]
pub enum Node {
    Arg             (Loc,   String),
    Args            (Loc,   Vec<Box<Node>>),
    Array           (Loc,   Vec<Box<Node>>),
    Begin           (Loc,   Vec<Box<Node>>),
    Cbase           (Loc),
    Const           (Loc,   Option<Box<Node>>, Id),
    Class           (Loc,   Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    CSend           (Loc,   Option<Box<Node>>, Id, Vec<Box<Node>>),
    Def             (Loc,   Id, Option<Box<Node>>, Option<Box<Node>>),
    DString         (Loc,   Vec<Box<Node>>),
    EncodingLiteral (Loc),
    Ensure          (Loc,   Option<Box<Node>>, Box<Node>),
    False           (Loc),
    FileLiteral     (Loc),
    Integer         (Loc,   String),
    LineLiteral     (Loc),
    Lvar            (Loc,   String),
    Nil             (Loc),
    Rescue          (Loc,   Option<Box<Node>>, Vec<Box<Node>>, Option<Box<Node>>),
    Self_           (Loc),
    Send            (Loc,   Option<Box<Node>>, Id, Vec<Box<Node>>),
    String          (Loc,   String),
    Symbol          (Loc,   String),
    True            (Loc),
}

impl Node {
    pub fn loc(&self) -> &Loc {
        match self {
            &Node::Arg(ref loc, _) => loc,
            &Node::Args(ref loc, _) => loc,
            &Node::Array(ref loc, _) => loc,
            &Node::Begin(ref loc, _) => loc,
            &Node::Cbase(ref loc) => loc,
            &Node::Class(ref loc, _, _, _) => loc,
            &Node::Const(ref loc, _, _) => loc,
            &Node::CSend(ref loc, _, _, _) => loc,
            &Node::Def(ref loc, _, _, _) => loc,
            &Node::DString(ref loc, _) => loc,
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
            &Node::Symbol(ref loc, _) => loc,
            &Node::True(ref loc) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub filename: String,
    pub node: Option<Box<Node>>,
}
