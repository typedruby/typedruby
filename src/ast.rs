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
    Alias           (Loc,   Box<Node>, Box<Node>),
    And             (Loc,   Box<Node>, Box<Node>),
    AndAsgn         (Loc,   Box<Node>, Box<Node>),
    Arg             (Loc,   String),
    Args            (Loc,   Vec<Box<Node>>),
    Array           (Loc,   Vec<Box<Node>>),
    Begin           (Loc,   Vec<Box<Node>>),
    Block           (Loc,   Box<Node>, Box<Node>, Option<Box<Node>>),
    BlockPass       (Loc,   Box<Node>),
    Break           (Loc,   Vec<Box<Node>>),
    Case            (Loc,   Option<Box<Node>>, Vec<Box<Node>>, Option<Box<Node>>),
    Casgn           (Loc,   Option<Box<Node>>, Id, Box<Node>),
    Cbase           (Loc),
    Class           (Loc,   Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    Const           (Loc,   Option<Box<Node>>, Id),
    CSend           (Loc,   Option<Box<Node>>, Id, Vec<Box<Node>>),
    Cvar            (Loc,   String),
    Cvasgn          (Loc,   Id, Box<Node>),
    Def             (Loc,   Id, Option<Box<Node>>, Option<Box<Node>>),
    Defined         (Loc,   Box<Node>),
    Defs            (Loc,   Box<Node>, Id, Option<Box<Node>>, Option<Box<Node>>),
    DString         (Loc,   Vec<Box<Node>>),
    EFlipflop       (Loc,   Box<Node>, Box<Node>),
    EncodingLiteral (Loc),
    Ensure          (Loc,   Option<Box<Node>>, Box<Node>),
    ERange          (Loc,   Box<Node>, Box<Node>),
    False           (Loc),
    FileLiteral     (Loc),
    Hash            (Loc,   Vec<Box<Node>>),
    Ident           (Loc,   String),
    If              (Loc,   Box<Node>, Option<Box<Node>>, Option<Box<Node>>),
    IFlipflop       (Loc,   Box<Node>, Box<Node>),
    Integer         (Loc,   String),
    IRange          (Loc,   Box<Node>, Box<Node>),
    Ivar            (Loc,   String),
    Ivasgn          (Loc,   Id, Box<Node>),
    Kwarg           (Loc,   String),
    KwBegin         (Loc,   Box<Node>),
    Kwoptarg        (Loc,   Id, Box<Node>),
    Kwsplat         (Loc,   Box<Node>),
    Lambda          (Loc),
    LineLiteral     (Loc),
    Lvar            (Loc,   String),
    Lvasgn          (Loc,   Id, Box<Node>),
    Lvassignable    (Loc,   String),
    MatchCurLine    (Loc,   Box<Node>),
    Masgn           (Loc,   Box<Node>, Box<Node>),
    Mlhs            (Loc,   Vec<Box<Node>>),
    Next            (Loc,   Vec<Box<Node>>),
    Nil             (Loc),
    OpAsgn          (Loc,   Box<Node>, Id, Box<Node>),
    Optarg          (Loc,   Id, Box<Node>),
    Or              (Loc,   Box<Node>, Box<Node>),
    OrAsgn          (Loc,   Box<Node>, Box<Node>),
    Pair            (Loc,   Box<Node>, Box<Node>),
    Procarg0        (Loc,   Box<Node>),
    Redo            (Loc),
    Regexp          (Loc,   Vec<Box<Node>>, Option<Box<Node>>),
    Regopt          (Loc,   Vec<char>),
    Resbody         (Loc,   Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>),
    Rescue          (Loc,   Option<Box<Node>>, Vec<Box<Node>>, Option<Box<Node>>),
    Restarg         (Loc,   Id),
    Retry           (Loc),
    Return          (Loc,   Vec<Box<Node>>),
    SClass          (Loc,   Box<Node>, Option<Box<Node>>),
    Self_           (Loc),
    Send            (Loc,   Option<Box<Node>>, Id, Vec<Box<Node>>),
    Splat           (Loc,   Box<Node>),
    String          (Loc,   String),
    Super           (Loc,   Vec<Box<Node>>),
    Symbol          (Loc,   String),
    True            (Loc),
    Until           (Loc,   Box<Node>, Option<Box<Node>>),
    UntilPost       (Loc,   Box<Node>, Box<Node>),
    When            (Loc,   Vec<Box<Node>>, Option<Box<Node>>),
    While           (Loc,   Box<Node>, Option<Box<Node>>),
    WhilePost       (Loc,   Box<Node>, Box<Node>),
    Yield           (Loc,   Vec<Box<Node>>),
    ZSuper          (Loc),
}

impl Node {
    pub fn loc(&self) -> &Loc {
        match self {
            &Node::Alias(ref loc, _, _) => loc,
            &Node::And(ref loc, _, _) => loc,
            &Node::AndAsgn(ref loc, _, _) => loc,
            &Node::Arg(ref loc, _) => loc,
            &Node::Args(ref loc, _) => loc,
            &Node::Array(ref loc, _) => loc,
            &Node::Begin(ref loc, _) => loc,
            &Node::Block(ref loc, _, _, _) => loc,
            &Node::BlockPass(ref loc, _) => loc,
            &Node::Break(ref loc, _) => loc,
            &Node::Case(ref loc, _, _, _) => loc,
            &Node::Casgn(ref loc, _, _, _) => loc,
            &Node::Cbase(ref loc) => loc,
            &Node::Class(ref loc, _, _, _) => loc,
            &Node::Const(ref loc, _, _) => loc,
            &Node::CSend(ref loc, _, _, _) => loc,
            &Node::Cvar(ref loc, _) => loc,
            &Node::Cvasgn(ref loc, _, _) => loc,
            &Node::Def(ref loc, _, _, _) => loc,
            &Node::Defined(ref loc, _) => loc,
            &Node::Defs(ref loc, _, _, _, _) => loc,
            &Node::DString(ref loc, _) => loc,
            &Node::EFlipflop(ref loc, _, _) => loc,
            &Node::EncodingLiteral(ref loc) => loc,
            &Node::Ensure(ref loc, _, _) => loc,
            &Node::ERange(ref loc, _, _) => loc,
            &Node::False(ref loc) => loc,
            &Node::FileLiteral(ref loc) => loc,
            &Node::Hash(ref loc, _) => loc,
            &Node::Ident(ref loc, _) => loc,
            &Node::If(ref loc, _, _, _) => loc,
            &Node::IFlipflop(ref loc, _, _) => loc,
            &Node::Integer(ref loc, _) => loc,
            &Node::IRange(ref loc, _, _) => loc,
            &Node::Ivar(ref loc, _) => loc,
            &Node::Ivasgn(ref loc, _, _) => loc,
            &Node::Kwarg(ref loc, _) => loc,
            &Node::KwBegin(ref loc, _) => loc,
            &Node::Kwoptarg(ref loc, _, _) => loc,
            &Node::Kwsplat(ref loc, _) => loc,
            &Node::Lambda(ref loc) => loc,
            &Node::LineLiteral(ref loc) => loc,
            &Node::Lvar(ref loc, _) => loc,
            &Node::Lvassignable(ref loc, _) => loc,
            &Node::Lvasgn(ref loc, _, _) => loc,
            &Node::MatchCurLine(ref loc, _) => loc,
            &Node::Masgn(ref loc, _, _) => loc,
            &Node::Mlhs(ref loc, _) => loc,
            &Node::Next(ref loc, _) => loc,
            &Node::Nil(ref loc) => loc,
            &Node::OpAsgn(ref loc, _, _, _) => loc,
            &Node::Optarg(ref loc, _, _) => loc,
            &Node::Or(ref loc, _, _) => loc,
            &Node::OrAsgn(ref loc, _, _) => loc,
            &Node::Pair(ref loc, _, _) => loc,
            &Node::Procarg0(ref loc, _) => loc,
            &Node::Redo(ref loc) => loc,
            &Node::Regexp(ref loc, _, _) => loc,
            &Node::Regopt(ref loc, _) => loc,
            &Node::Resbody(ref loc, _, _, _) => loc,
            &Node::Rescue(ref loc, _, _, _) => loc,
            &Node::Restarg(ref loc, _) => loc,
            &Node::Retry(ref loc) => loc,
            &Node::Return(ref loc, _) => loc,
            &Node::SClass(ref loc, _, _) => loc,
            &Node::Self_(ref loc) => loc,
            &Node::Send(ref loc, _, _, _) => loc,
            &Node::Splat(ref loc, _) => loc,
            &Node::String(ref loc, _) => loc,
            &Node::Super(ref loc, _) => loc,
            &Node::Symbol(ref loc, _) => loc,
            &Node::True(ref loc) => loc,
            &Node::Until(ref loc, _, _) => loc,
            &Node::UntilPost(ref loc, _, _) => loc,
            &Node::When(ref loc, _, _) => loc,
            &Node::While(ref loc, _, _) => loc,
            &Node::WhilePost(ref loc, _, _) => loc,
            &Node::Yield(ref loc, _) => loc,
            &Node::ZSuper(ref loc) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub filename: String,
    pub node: Option<Box<Node>>,
}
