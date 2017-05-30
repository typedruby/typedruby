use std::ops::Deref;
use std::fmt;
use std::rc::Rc;
use ast::*;

pub trait Sexp {
    fn sexp(&self, f: &mut SexpFormatter) -> fmt::Result;
}

pub struct SexpFormatter<'a> {
    indent: usize,
    buf: &'a mut (fmt::Write+'a),
    print_loc: bool,
    print_str: bool,
}

impl<'a> SexpFormatter<'a> {
    pub fn write_str(&mut self, data: &str) -> fmt::Result {
        self.buf.write_str(data)
    }

    pub fn write_fmt(&mut self, fmt: fmt::Arguments) -> fmt::Result {
        fmt::write(self.buf, fmt)
    }

    #[inline]
    pub fn new_node<'b>(&'b mut self, name: &str) -> SexpNode<'b, 'a> {
        sexp_node_new(self, name)
    }
}

pub struct SexpNode<'a, 'b: 'a> {
    fmt: &'a mut SexpFormatter<'b>,
    result: fmt::Result,
}

pub fn sexp_node_new<'a, 'b>(fmt: &'a mut SexpFormatter<'b>, name: &str) -> SexpNode<'a, 'b> {
    let indent = fmt.indent*2;
    let result = write!(fmt, "{:width$}({}", "", name.to_lowercase(), width=indent);
    SexpNode {
        fmt: fmt,
        result: result,
    }
}

fn escape_rb(f: &mut SexpFormatter, s: &String) -> fmt::Result {
    f.buf.write_char('"')?;
    let mut from = 0;
    for (i, c) in s.char_indices() {
        let esc = c.escape_default();
        if esc.len() != 1 {
            f.write_str(&s[from..i])?;
            for c in esc {
                f.buf.write_char(c)?;
            }
            from = i + c.len_utf8();
        }
    }
    f.buf.write_str(&s[from..])?;
    f.buf.write_char('"')
}

impl<'a, 'b: 'a> SexpNode<'a, 'b> {
    pub fn field(&mut self, value: &Sexp) -> &mut SexpNode<'a, 'b> {
        self.result = self.result.and_then(|_| {
            self.fmt.indent += 1;
            let res = value.sexp(self.fmt);
            self.fmt.indent -= 1;
            res
        });
        self
    }

    pub fn string(&mut self, value: &String) -> &mut SexpNode<'a, 'b> {
        self.result = self.result.and_then(|_| {
            if self.fmt.print_str {
                self.fmt.buf.write_char(' ')?;
                escape_rb(self.fmt, value)
            } else {
                write!(self.fmt, " [STRING]")
            }
        });
        self
    }

    pub fn numeric(&mut self, value: &String) -> &mut SexpNode<'a, 'b> {
        self.result = self.result.and_then(|_| {
            write!(self.fmt, " {}", value)
        });
        self
    }

    pub fn finish(&mut self) -> fmt::Result {
        self.result.and_then(|_| {
            self.fmt.write_str(")")
        })
    }
}

impl Sexp for Vec<Rc<Node>> {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        for ref n in self.iter() {
            n.sexp(w)?;
        }
        Ok(())
    }
}

impl Sexp for Loc {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        if w.print_loc {
            write!(w, " @{:?}", self)
        } else {
            Ok(())
        }
    }
}

impl Sexp for Rc<Node> {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        if w.indent > 0 {
            write!(w, "\n")?;
        }
        self.deref().sexp(w)
    }
}

impl Sexp for Option<Rc<Node>> {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        match self {
            &Some(ref n) => n.sexp(w),
            &None => write!(w, " nil"),
        }
    }
}

impl Sexp for Id {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        match self {
            &Id(_, ref id) => write!(w, " :{}", id)
        }
    }
}

impl Sexp for String {
    fn sexp(&self, f: &mut SexpFormatter) -> fmt::Result {
        write!(f, " :{}", self)
    }
}

impl Sexp for Vec<char> {
    fn sexp(&self, f: &mut SexpFormatter) -> fmt::Result {
        if !self.is_empty() {
            for b in self.iter() {
                write!(f, " :{}", b)?;
            }
        }
        Ok(())
    }
}

impl Sexp for usize {
    fn sexp(&self, f: &mut SexpFormatter) -> fmt::Result {
        write!(f, " {}", self)
    }
}

impl Sexp for Option<Id> {
    fn sexp(&self, w: &mut SexpFormatter) -> fmt::Result {
        match self {
            &Some(ref n) => { n.sexp(w) },
            &None => Ok(())
        }
    }
}

impl Sexp for Node {
    fn sexp(&self, __arg_0: &mut SexpFormatter) -> fmt::Result {
        match (&*self,) {
            (&Node::Alias(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Alias");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::And(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("And");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::AndAsgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("and-asgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Arg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Arg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Args(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Args");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Array(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Array");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Backref(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("back-ref");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Begin(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Begin");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Block(ref __self_0, ref __self_1, ref __self_2,
                          ref __self_3),) => {
                let mut builder = __arg_0.new_node("Block");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Blockarg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Blockarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::BlockPass(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("block-pass");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Break(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Break");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Case(ref __self_0, ref __self_1, ref __self_2,
                         ref __self_3),) => {
                let mut builder = __arg_0.new_node("Case");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Casgn(ref __self_0, ref __self_1, ref __self_2,
                          ref __self_3),) => {
                let mut builder = __arg_0.new_node("Casgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Cbase(ref __self_0),) => {
                let mut builder = __arg_0.new_node("Cbase");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Class(ref __self_0, ref __self_1, ref __self_2,
                          ref __self_3),) => {
                let mut builder = __arg_0.new_node("Class");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Complex(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("complex");
                let _ = builder.field(__self_0);
                let _ = builder.numeric(__self_1);
                builder.finish()
            }
            (&Node::Const(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Const");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::CSend(ref __self_0, ref __self_1, ref __self_2,
                          ref __self_3),) => {
                let mut builder = __arg_0.new_node("CSend");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Cvar(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Cvar");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Cvasgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Cvasgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Def(ref __self_0, ref __self_1, ref __self_2,
                        ref __self_3),) => {
                let mut builder = __arg_0.new_node("Def");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Defined(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("defined?");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Defs(ref __self_0, ref __self_1, ref __self_2,
                         ref __self_3, ref __self_4),) => {
                let mut builder = __arg_0.new_node("Defs");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                let _ = builder.field(__self_4);
                builder.finish()
            }
            (&Node::DString(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("dstr");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::DSymbol(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("dsym");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::EFlipflop(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("EFlipflop");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::EncodingLiteral(ref __self_0),) => {
                let mut builder = __arg_0.new_node("EncodingLiteral");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Ensure(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Ensure");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::ERange(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("ERange");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::False(ref __self_0),) => {
                let mut builder = __arg_0.new_node("False");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::FileLiteral(ref __self_0),) => {
                let mut builder = __arg_0.new_node("FileLiteral");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::For(ref __self_0, ref __self_1, ref __self_2,
                        ref __self_3),) => {
                let mut builder = __arg_0.new_node("For");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Float(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Float");
                let _ = builder.field(__self_0);
                let _ = builder.numeric(__self_1);
                builder.finish()
            }
            (&Node::Gvar(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Gvar");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Gvasgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Gvasgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Hash(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Hash");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Ident(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Ident");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::If(ref __self_0, ref __self_1, ref __self_2,
                       ref __self_3),) => {
                let mut builder = __arg_0.new_node("If");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::IFlipflop(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("IFlipflop");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Integer(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("int");
                let _ = builder.field(__self_0);
                let _ = builder.numeric(__self_1);
                builder.finish()
            }
            (&Node::IRange(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("IRange");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Ivar(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Ivar");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Ivasgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Ivasgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                match *__self_2 {
                    Some(ref x) => { let _ = builder.field(x); },
                    None => {},
                };
                builder.finish()
            }
            (&Node::Kwarg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Kwarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Kwbegin(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Kwbegin");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Kwoptarg(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Kwoptarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Kwrestarg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Kwrestarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Kwsplat(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Kwsplat");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Lambda(ref __self_0),) => {
                let mut builder = __arg_0.new_node("Lambda");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::LineLiteral(ref __self_0),) => {
                let mut builder = __arg_0.new_node("LineLiteral");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Lvar(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Lvar");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Lvasgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("lvasgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                match *__self_2 {
                    Some(ref x) => { let _ = builder.field(x); },
                    None => {},
                };
                builder.finish()
            }
            (&Node::MatchCurLine(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("MatchCurLine");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Masgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Masgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Mlhs(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Mlhs");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Module(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Module");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Next(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Next");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Nil(ref __self_0),) => {
                let mut builder = __arg_0.new_node("Nil");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::NthRef(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("nth-ref");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::OpAsgn(ref __self_0, ref __self_1, ref __self_2,
                           ref __self_3),) => {
                let mut builder = __arg_0.new_node("op-asgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Optarg(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Optarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Or(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Or");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::OrAsgn(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("or-asgn");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Pair(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Pair");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Postexe(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Postexe");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Preexe(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Preexe");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Procarg0(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Procarg0");
                let _ = builder.field(__self_0);
                let _ = match **__self_1 {
                    Node::Arg(_, ref arg) => builder.field(arg),
                    _ => builder.field(__self_1),
                };
                builder.finish()
            }
            (&Node::Prototype(ref __self_0, ref __self_1, ref __self_2,
                              ref __self_3),) => {
                let mut builder = __arg_0.new_node("Prototype");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Rational(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("rational");
                let _ = builder.field(__self_0);
                let _ = builder.numeric(__self_1);
                builder.finish()
            }
            (&Node::Redo(ref __self_0),) => {
                let mut builder = __arg_0.new_node("Redo");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Regexp(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Regexp");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Regopt(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Regopt");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Resbody(ref __self_0, ref __self_1, ref __self_2,
                            ref __self_3),) => {
                let mut builder = __arg_0.new_node("Resbody");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Rescue(ref __self_0, ref __self_1, ref __self_2,
                           ref __self_3),) => {
                let mut builder = __arg_0.new_node("Rescue");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Restarg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Restarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Retry(ref __self_0),) => {
                let mut builder = __arg_0.new_node("Retry");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Return(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Return");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::SClass(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("SClass");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::Self_(ref __self_0),) => {
                let mut builder = __arg_0.new_node("self");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::Send(ref __self_0, ref __self_1, ref __self_2,
                         ref __self_3),) => {
                let mut builder = __arg_0.new_node("Send");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                let _ = builder.field(__self_3);
                builder.finish()
            }
            (&Node::Splat(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Splat");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::String(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("str");
                let _ = builder.field(__self_0);
                let _ = builder.string(__self_1);
                builder.finish()
            }
            (&Node::Super(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Super");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Symbol(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("sym");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::True(ref __self_0),) => {
                let mut builder = __arg_0.new_node("True");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyAny(ref __self_0),) => {
                let mut builder = __arg_0.new_node("TyAny");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyArray(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyArray");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TyCast(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TyCast");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyClass(ref __self_0),) => {
                let mut builder = __arg_0.new_node("TyClass");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyCpath(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyCpath");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TyGenargs(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyGenargs");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TyGendecl(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TyGendecl");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyGendeclarg(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyGendeclarg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TyGeninst(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TyGeninst");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyHash(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TyHash");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyInstance(ref __self_0),) => {
                let mut builder = __arg_0.new_node("TyInstance");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyIvardecl(ref __self_0, ref __self_1, ref __self_2),) =>
            {
                let mut builder = __arg_0.new_node("TyIvardecl");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyNil(ref __self_0),) => {
                let mut builder = __arg_0.new_node("TyNil");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyNillable(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyNillable");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TyOr(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TyOr");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TypedArg(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("TypedArg");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::TyProc(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyProc");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::TySelf(ref __self_0),) => {
                let mut builder = __arg_0.new_node("TySelf");
                let _ = builder.field(__self_0);
                builder.finish()
            }
            (&Node::TyTuple(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("TyTuple");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Undef(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Undef");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Until(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("Until");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::UntilPost(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("UntilPost");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::When(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("When");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::While(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("While");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::WhilePost(ref __self_0, ref __self_1, ref __self_2),) => {
                let mut builder = __arg_0.new_node("WhilePost");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                let _ = builder.field(__self_2);
                builder.finish()
            }
            (&Node::XString(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("xstr");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::Yield(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.new_node("Yield");
                let _ = builder.field(__self_0);
                let _ = builder.field(__self_1);
                builder.finish()
            }
            (&Node::ZSuper(ref __self_0),) => {
                let mut builder = __arg_0.new_node("ZSuper");
                let _ = builder.field(__self_0);
                builder.finish()
            }
        }
    }
}

impl Ast {
    pub fn to_sexp(&self, output: &mut fmt::Write) -> fmt::Result {
        match self.node {
            Some(ref node) => {
                let mut formatter = SexpFormatter {
                    indent: 0,
                    buf: output,
                    print_loc: false,
                    print_str: true,
                };
                node.sexp(&mut formatter)
            }
            None => Ok(())
        }
    }
}
