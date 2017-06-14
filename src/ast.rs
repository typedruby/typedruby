use std::cmp::{min, max};
use std::fs::File;
use std::io;
use std::path::{PathBuf, Path};
use std::rc::Rc;
use std::vec::Vec;
use std::io::prelude::*;
use std::fmt;
use diagnostics::{Error};

pub struct SourceFile {
    filename: PathBuf,
    source: String,
    line_map: Vec<usize>,
}

pub struct SourceLine {
    pub number: usize,
    pub begin_pos: usize,
    pub end_pos: usize,
}

#[derive(Clone)]
pub struct Loc {
    pub file: Rc<SourceFile>,
    pub begin_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[repr(C)]
pub enum Level {
	Note    = 1,
	Warning = 2,
	Error   = 3,
	Fatal   = 4,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub error: Error,
    pub level: Level,
    pub loc: Loc,
    pub data: Option<String>,
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}:{}-{}", self.file.filename.display(), self.begin_pos, self.end_pos)
    }
}

impl Loc {
    pub fn join(&self, other: &Loc) -> Loc {
        if self.file.filename != other.file.filename {
            panic!("can't join Locs of disparate files");
        }

        Loc {
            file: self.file.clone(),
            begin_pos: min(self.begin_pos, other.begin_pos),
            end_pos: max(self.end_pos, other.end_pos),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Id(pub Loc, pub String);

#[derive(Debug)]
pub enum Node {
    Alias           (Loc,   Rc<Node>, Rc<Node>),
    And             (Loc,   Rc<Node>, Rc<Node>),
    AndAsgn         (Loc,   Rc<Node>, Rc<Node>),
    Arg             (Loc,   String),
    Args            (Loc,   Vec<Rc<Node>>),
    Array           (Loc,   Vec<Rc<Node>>),
    Backref         (Loc,   String),
    Begin           (Loc,   Vec<Rc<Node>>),
    Block           (Loc,   Rc<Node>, Rc<Node>, Option<Rc<Node>>),
    Blockarg        (Loc,   Option<Id>),
    BlockPass       (Loc,   Rc<Node>),
    Break           (Loc,   Vec<Rc<Node>>),
    Case            (Loc,   Option<Rc<Node>>, Vec<Rc<Node>>, Option<Rc<Node>>),
    Cbase           (Loc),
    Class           (Loc,   Rc<Node>, Option<Rc<Node>>, Option<Rc<Node>>),
    Complex         (Loc,   String),
    Const           (Loc,   Option<Rc<Node>>, Id),
    ConstLhs        (Loc,   Option<Rc<Node>>, Id),
    ConstAsgn       (Loc,   Option<Rc<Node>>, Id, Rc<Node>),
    CSend           (Loc,   Option<Rc<Node>>, Id, Vec<Rc<Node>>),
    Cvar            (Loc,   String),
    CvarLhs         (Loc,   Id),
    CvarAsgn        (Loc,   Id, Rc<Node>),
    Def             (Loc,   Id, Option<Rc<Node>>, Option<Rc<Node>>),
    Defined         (Loc,   Rc<Node>),
    Defs            (Loc,   Rc<Node>, Id, Option<Rc<Node>>, Option<Rc<Node>>),
    DString         (Loc,   Vec<Rc<Node>>),
    DSymbol         (Loc,   Vec<Rc<Node>>),
    EFlipflop       (Loc,   Rc<Node>, Rc<Node>),
    EncodingLiteral (Loc),
    Ensure          (Loc,   Option<Rc<Node>>, Option<Rc<Node>>),
    ERange          (Loc,   Rc<Node>, Rc<Node>),
    False           (Loc),
    FileLiteral     (Loc),
    For             (Loc,   Rc<Node>, Rc<Node>, Option<Rc<Node>>),
    Float           (Loc,   String),
    Gvar            (Loc,   String),
    GvarAsgn        (Loc,   Id, Rc<Node>),
    GvarLhs         (Loc,   Id),
    Hash            (Loc,   Vec<Rc<Node>>),
    Ident           (Loc,   String),
    If              (Loc,   Rc<Node>, Option<Rc<Node>>, Option<Rc<Node>>),
    IFlipflop       (Loc,   Rc<Node>, Rc<Node>),
    Integer         (Loc,   String),
    IRange          (Loc,   Rc<Node>, Rc<Node>),
    Ivar            (Loc,   String),
    IvarAsgn        (Loc,   Id, Rc<Node>),
    IvarLhs         (Loc,   Id),
    Kwarg           (Loc,   String),
    Kwbegin         (Loc,   Vec<Rc<Node>>),
    Kwoptarg        (Loc,   Id, Rc<Node>),
    Kwrestarg       (Loc,   Option<Id>),
    Kwsplat         (Loc,   Rc<Node>),
    Lambda          (Loc),
    LineLiteral     (Loc),
    Lvar            (Loc,   String),
    LvarAsgn        (Loc,   Id, Rc<Node>),
    LvarLhs         (Loc,   Id),
    MatchAsgn       (Loc,   Rc<Node>, Rc<Node>),
    MatchCurLine    (Loc,   Rc<Node>),
    Masgn           (Loc,   Rc<Node>, Rc<Node>),
    Mlhs            (Loc,   Vec<Rc<Node>>),
    Module          (Loc,   Rc<Node>, Option<Rc<Node>>),
    Next            (Loc,   Vec<Rc<Node>>),
    Nil             (Loc),
    NthRef          (Loc,   usize),
    OpAsgn          (Loc,   Rc<Node>, Id, Rc<Node>),
    Optarg          (Loc,   Id, Rc<Node>),
    Or              (Loc,   Rc<Node>, Rc<Node>),
    OrAsgn          (Loc,   Rc<Node>, Rc<Node>),
    Pair            (Loc,   Rc<Node>, Rc<Node>),
    Postexe         (Loc,   Option<Rc<Node>>),
    Preexe          (Loc,   Option<Rc<Node>>),
    Procarg0        (Loc,   Rc<Node>),
    Prototype       (Loc,   Option<Rc<Node>>, Option<Rc<Node>>, Option<Rc<Node>>),
    Rational        (Loc,   String),
    Redo            (Loc),
    Regexp          (Loc,   Vec<Rc<Node>>, Option<Rc<Node>>),
    Regopt          (Loc,   Vec<char>),
    Resbody         (Loc,   Option<Rc<Node>>, Option<Rc<Node>>, Option<Rc<Node>>),
    Rescue          (Loc,   Option<Rc<Node>>, Vec<Rc<Node>>, Option<Rc<Node>>),
    Restarg         (Loc,   Option<Id>),
    Retry           (Loc),
    Return          (Loc,   Vec<Rc<Node>>),
    SClass          (Loc,   Rc<Node>, Option<Rc<Node>>),
    Self_           (Loc),
    Send            (Loc,   Option<Rc<Node>>, Id, Vec<Rc<Node>>),
    ShadowArg       (Loc,   Id),
    Splat           (Loc,   Option<Rc<Node>>),
    String          (Loc,   String),
    Super           (Loc,   Vec<Rc<Node>>),
    Symbol          (Loc,   String),
    True            (Loc),
    TyAny           (Loc),
    TyArray         (Loc,   Rc<Node>),
    TyCast          (Loc,   Rc<Node>, Rc<Node>),
    TyClass         (Loc),
    TyConSubtype    (Loc,   Rc<Node>, Rc<Node>),
    TyConUnify      (Loc,   Rc<Node>, Rc<Node>),
    TyCpath         (Loc,   Rc<Node>),
    TyGenargs       (Loc,   Vec<Rc<Node>>),
    TyGendecl       (Loc,   Rc<Node>, Vec<Rc<Node>>),
    TyGendeclarg    (Loc,   String, Option<Rc<Node>>),
    TyGeninst       (Loc,   Rc<Node>, Vec<Rc<Node>>),
    TyHash          (Loc,   Rc<Node>, Rc<Node>),
    TyInstance      (Loc),
    TyIvardecl      (Loc,   Id, Rc<Node>),
    TyNil           (Loc),
    TyNillable      (Loc,   Rc<Node>),
    TyOr            (Loc,   Rc<Node>, Rc<Node>),
    TypedArg        (Loc,   Rc<Node>, Rc<Node>),
    TyProc          (Loc,   Rc<Node>),
    TySelf          (Loc),
    TyTuple         (Loc,   Vec<Rc<Node>>),
    Undef           (Loc,   Vec<Rc<Node>>),
    Until           (Loc,   Rc<Node>, Option<Rc<Node>>),
    UntilPost       (Loc,   Rc<Node>, Rc<Node>),
    When            (Loc,   Vec<Rc<Node>>, Option<Rc<Node>>),
    While           (Loc,   Rc<Node>, Option<Rc<Node>>),
    WhilePost       (Loc,   Rc<Node>, Rc<Node>),
    XString         (Loc,   Vec<Rc<Node>>),
    Yield           (Loc,   Vec<Rc<Node>>),
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
            &Node::Backref(ref loc, _) => loc,
            &Node::Begin(ref loc, _) => loc,
            &Node::Block(ref loc, _, _, _) => loc,
            &Node::Blockarg(ref loc, _) => loc,
            &Node::BlockPass(ref loc, _) => loc,
            &Node::Break(ref loc, _) => loc,
            &Node::Case(ref loc, _, _, _) => loc,
            &Node::Cbase(ref loc) => loc,
            &Node::Class(ref loc, _, _, _) => loc,
            &Node::Complex(ref loc, _) => loc,
            &Node::Const(ref loc, _, _) => loc,
            &Node::ConstAsgn(ref loc, _, _, _) => loc,
            &Node::ConstLhs(ref loc, _, _) => loc,
            &Node::CSend(ref loc, _, _, _) => loc,
            &Node::Cvar(ref loc, _) => loc,
            &Node::CvarAsgn(ref loc, _, _) => loc,
            &Node::CvarLhs(ref loc, _) => loc,
            &Node::Def(ref loc, _, _, _) => loc,
            &Node::Defined(ref loc, _) => loc,
            &Node::Defs(ref loc, _, _, _, _) => loc,
            &Node::DString(ref loc, _) => loc,
            &Node::DSymbol(ref loc, _) => loc,
            &Node::EFlipflop(ref loc, _, _) => loc,
            &Node::EncodingLiteral(ref loc) => loc,
            &Node::Ensure(ref loc, _, _) => loc,
            &Node::ERange(ref loc, _, _) => loc,
            &Node::False(ref loc) => loc,
            &Node::FileLiteral(ref loc) => loc,
            &Node::For(ref loc, _, _, _) => loc,
            &Node::Float(ref loc, _) => loc,
            &Node::Gvar(ref loc, _) => loc,
            &Node::GvarAsgn(ref loc, _, _) => loc,
            &Node::GvarLhs(ref loc, _) => loc,
            &Node::Hash(ref loc, _) => loc,
            &Node::Ident(ref loc, _) => loc,
            &Node::If(ref loc, _, _, _) => loc,
            &Node::IFlipflop(ref loc, _, _) => loc,
            &Node::Integer(ref loc, _) => loc,
            &Node::IRange(ref loc, _, _) => loc,
            &Node::Ivar(ref loc, _) => loc,
            &Node::IvarAsgn(ref loc, _, _) => loc,
            &Node::IvarLhs(ref loc, _) => loc,
            &Node::Kwarg(ref loc, _) => loc,
            &Node::Kwbegin(ref loc, _) => loc,
            &Node::Kwoptarg(ref loc, _, _) => loc,
            &Node::Kwrestarg(ref loc, _) => loc,
            &Node::Kwsplat(ref loc, _) => loc,
            &Node::Lambda(ref loc) => loc,
            &Node::LineLiteral(ref loc) => loc,
            &Node::Lvar(ref loc, _) => loc,
            &Node::LvarAsgn(ref loc, _, _) => loc,
            &Node::LvarLhs(ref loc, _) => loc,
            &Node::MatchAsgn(ref loc, _, _) => loc,
            &Node::MatchCurLine(ref loc, _) => loc,
            &Node::Masgn(ref loc, _, _) => loc,
            &Node::Mlhs(ref loc, _) => loc,
            &Node::Module(ref loc, _, _) => loc,
            &Node::Next(ref loc, _) => loc,
            &Node::NthRef(ref loc, _) => loc,
            &Node::Nil(ref loc) => loc,
            &Node::OpAsgn(ref loc, _, _, _) => loc,
            &Node::Optarg(ref loc, _, _) => loc,
            &Node::Or(ref loc, _, _) => loc,
            &Node::OrAsgn(ref loc, _, _) => loc,
            &Node::Pair(ref loc, _, _) => loc,
            &Node::Postexe(ref loc, _) => loc,
            &Node::Preexe(ref loc, _) => loc,
            &Node::Procarg0(ref loc, _) => loc,
            &Node::Prototype(ref loc, _, _, _) => loc,
            &Node::Rational(ref loc, _) => loc,
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
            &Node::ShadowArg(ref loc, _) => loc,
            &Node::Splat(ref loc, _) => loc,
            &Node::String(ref loc, _) => loc,
            &Node::Super(ref loc, _) => loc,
            &Node::Symbol(ref loc, _) => loc,
            &Node::True(ref loc) => loc,
            &Node::TyAny(ref loc) => loc,
            &Node::TyArray(ref loc, _) => loc,
            &Node::TyCast(ref loc, _, _) => loc,
            &Node::TyClass(ref loc) => loc,
            &Node::TyConSubtype(ref loc, _, _) => loc,
            &Node::TyConUnify(ref loc, _, _) => loc,
            &Node::TyCpath(ref loc, _) => loc,
            &Node::TyGenargs(ref loc, _) => loc,
            &Node::TyGendecl(ref loc, _, _) => loc,
            &Node::TyGendeclarg(ref loc, _, _) => loc,
            &Node::TyGeninst(ref loc, _, _) => loc,
            &Node::TyHash(ref loc, _, _) => loc,
            &Node::TyInstance(ref loc) => loc,
            &Node::TyIvardecl(ref loc, _, _) => loc,
            &Node::TyNil(ref loc) => loc,
            &Node::TyNillable(ref loc, _) => loc,
            &Node::TyOr(ref loc, _, _) => loc,
            &Node::TypedArg(ref loc, _, _) => loc,
            &Node::TyProc(ref loc, _) => loc,
            &Node::TySelf(ref loc) => loc,
            &Node::TyTuple(ref loc, _) => loc,
            &Node::Undef(ref loc, _) => loc,
            &Node::Until(ref loc, _, _) => loc,
            &Node::UntilPost(ref loc, _, _) => loc,
            &Node::When(ref loc, _, _) => loc,
            &Node::While(ref loc, _, _) => loc,
            &Node::WhilePost(ref loc, _, _) => loc,
            &Node::XString(ref loc, _) => loc,
            &Node::Yield(ref loc, _) => loc,
            &Node::ZSuper(ref loc) => loc,
        }
    }
}

#[derive(Debug)]
pub struct Ast {
    pub node: Option<Rc<Node>>,
    pub diagnostics: Vec<Diagnostic>,
}

fn line_map_from_source(source: &str) -> Vec<usize> {
    let mut line_map = vec![];
    let mut previous_index = 0;

    for (index, c) in source.char_indices() {
        if c == '\n' {
            line_map.push(previous_index);
            previous_index = index + 1;
        }
    }

    if line_map.is_empty() {
        line_map.push(0);
    }

    if line_map.last().unwrap() == &previous_index {
        line_map.push(source.len());
    } else {
        line_map.push(previous_index);
    }

    line_map
}

impl SourceFile {
    pub fn new(filename: PathBuf, source: String) -> SourceFile {
        let line_map = line_map_from_source(&source);

        SourceFile {
            filename: filename,
            source: source,
            line_map: line_map,
        }
    }

    pub fn open(filename: PathBuf) -> io::Result<SourceFile> {
        let mut file = File::open(&filename)?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        Ok(SourceFile::new(filename, source))
    }

    pub fn line_for_pos(&self, byte_pos: usize) -> SourceLine {
        let idx = match self.line_map.binary_search(&byte_pos) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };

        SourceLine {
            number: idx + 1,
            begin_pos: self.line_map[idx],
            end_pos: self.line_map[idx + 1],
        }
    }

    pub fn filename(&self) -> &Path {
        &self.filename
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
