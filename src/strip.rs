use std::rc::Rc;
use ast::{parse, Ast, SourceFile, Diagnostic, Node, Loc, Id};

struct ByteRange(usize, usize);

#[derive(Debug)]
pub enum StripError {
    SyntaxError(Vec<Diagnostic>),
}

pub fn strip(file: Rc<SourceFile>) -> Result<String, StripError> {
    let Ast { node, diagnostics } = parse(file);
    let node = node.ok_or_else(|| StripError::SyntaxError(diagnostics))?;
    let mut strip = Strip::new();
    strip.strip_node(&node);
    panic!("done")
}

trait IntoNode<'a> {
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

struct Strip {
    remove: Vec<ByteRange>,
}

impl Strip {
    fn new() -> Self {
        Strip { remove: Vec::new() }
    }

    fn remove(&mut self, loc: &Loc) {
        self.remove.push(ByteRange(loc.begin_pos, loc.end_pos));
    }

    fn remove_around(&mut self, enclosing: &Loc, inner: &Loc) {
        assert!(enclosing.begin_pos <= inner.begin_pos);
        assert!(enclosing.end_pos >= inner.end_pos);

        self.remove.push(ByteRange(enclosing.begin_pos, inner.begin_pos));
        self.remove.push(ByteRange(inner.end_pos, enclosing.end_pos));
    }

    fn strip_nodes(&mut self, nodes: &[Rc<Node>]) {
        for n in nodes {
            self.strip_node(n)
        }
    }

    fn strip_node<'a, T: IntoNode<'a>>(&mut self, node: T) {
        let node = match node.into_node() {
            Some(node) => node,
            None => return,
        };

        match *node {
            Node::Alias(_, ref a, ref b) |
            Node::And(_, ref a, ref b) |
            Node::AndAsgn(_, ref a, ref b) |
            Node::EFlipflop(_, ref a, ref b) |
            Node::ERange(_, ref a, ref b) |
            Node::IFlipflop(_, ref a, ref b) |
            Node::IRange(_, ref a, ref b) |
            Node::Masgn(_, ref a, ref b) |
            Node::Or(_, ref a, ref b) |
            Node::OrAsgn(_, ref a, ref b) |
            Node::Pair(_, ref a, ref b) |
            Node::UntilPost(_, ref a, ref b) |
            Node::WhilePost(_, ref a, ref b) => {
                self.strip_node(a);
                self.strip_node(b);
            }
            Node::When(_, ref a, ref b) |
            Node::Ensure(_, ref a, ref b) |
            Node::Until(_, ref a, ref b) |
            Node::While(_, ref a, ref b) |
            Node::SClass(_, ref a, ref b) |
            Node::Module(_, ref a, ref b) => {
                self.strip_node(a);
                self.strip_node(b);
            }
            Node::ConstAsgn(_, ref base, _, ref expr) => {
                self.strip_node(base);
                self.strip_node(expr);
            }
            Node::Arg(..) |
            Node::Backref(..) |
            Node::Blockarg(..) |
            Node::Cbase(..) |
            Node::Complex(..) |
            Node::Cvar(..) |
            Node::Ivar(..) |
            Node::CvarLhs(..) |
            Node::EncodingLiteral(..) =>
                {}
            Node::Args(_, ref nodes) |
            Node::Array(_, ref nodes) |
            Node::Begin(_, ref nodes) |
            Node::Break(_, ref nodes) |
            Node::DString(_, ref nodes) |
            Node::DSymbol(_, ref nodes) => {
                self.strip_nodes(nodes);
            }
            Node::Block(_, ref send, ref args, ref body) => {
                self.strip_node(send);
                self.strip_node(args);
                self.strip_node(body);
            }
            Node::BlockPass(_, ref node) |
            Node::CvarAsgn(_, _, ref node) |
            Node::Defined(_, ref node) => {
                self.strip_node(node);
            }
            Node::Const(_, ref node, _) |
            Node::ConstLhs(_, ref node, _) => {
                self.strip_node(node);
            }
            Node::Case(_, ref scrut, ref whens, ref else_) => {
                self.strip_node(scrut);
                self.strip_nodes(whens);
                self.strip_node(else_);
            }
            Node::Class(_, ref name, ref super_, ref body) => {
                self.strip_node(name);
                self.strip_node(super_);
                self.strip_node(body);
            }
            Node::CSend(_, ref recv, _, ref args) => {
                self.strip_node(recv);
                self.strip_nodes(args);
            }
            Node::Def(_, _, ref args, ref body) => {
                self.strip_node(args);
                self.strip_node(body);
            }
            Node::Defs(_, ref definee, _, ref args, ref body) => {
                self.strip_node(definee);
                self.strip_node(args);
                self.strip_node(body);
            }

            Node::False(ref loc) |
            Node::FileLiteral(ref loc) |
            Node::For(ref loc, _, _, _) |
            Node::Float(ref loc, _) |
            Node::Gvar(ref loc, _) |
            Node::GvarAsgn(ref loc, _, _) |
            Node::GvarLhs(ref loc, _) |
            Node::Hash(ref loc, _) |
            Node::Ident(ref loc, _) |
            Node::If(ref loc, _, _, _) |
            Node::Integer(ref loc, _) |
            Node::IvarAsgn(ref loc, _, _) |
            Node::IvarLhs(ref loc, _) |
            Node::Kwarg(ref loc, _) |
            Node::Kwbegin(ref loc, _) |
            Node::Kwoptarg(ref loc, _, _) |
            Node::Kwrestarg(ref loc, _) |
            Node::Kwsplat(ref loc, _) |
            Node::Lambda(ref loc) |
            Node::LineLiteral(ref loc) |
            Node::Lvar(ref loc, _) |
            Node::LvarAsgn(ref loc, _, _) |
            Node::LvarLhs(ref loc, _) |
            Node::MatchAsgn(ref loc, _, _) |
            Node::MatchCurLine(ref loc, _) |
            Node::Mlhs(ref loc, _) |
            Node::Next(ref loc, _) |
            Node::NthRef(ref loc, _) |
            Node::Nil(ref loc) |
            Node::OpAsgn(ref loc, _, _, _) |
            Node::Postexe(ref loc, _) |
            Node::Preexe(ref loc, _) |
            Node::Procarg0(ref loc, _) |
            Node::Prototype(ref loc, _, _, _) |
            Node::Rational(ref loc, _) |
            Node::Redo(ref loc) |
            Node::Regexp(ref loc, _, _) |
            Node::Regopt(ref loc, _) |
            Node::Resbody(ref loc, _, _, _) |
            Node::Rescue(ref loc, _, _, _) |
            Node::Restarg(ref loc, _) |
            Node::Retry(ref loc) |
            Node::Return(ref loc, _) |
            Node::Self_(ref loc) |
            Node::Send(ref loc, _, _, _) |
            Node::ShadowArg(ref loc, _) |
            Node::Splat(ref loc, _) |
            Node::String(ref loc, _) |
            Node::Super(ref loc, _) |
            Node::Symbol(ref loc, _) |
            Node::True(ref loc) |
            Node::TyAny(ref loc) |
            Node::TyArray(ref loc, _) |
            Node::TyCast(ref loc, _, _) |
            Node::TyClass(ref loc) |
            Node::TyConstInstance(ref loc, _, _) |
            Node::TyConSubtype(ref loc, _, _) |
            Node::TyConUnify(ref loc, _, _) |
            Node::TyCpath(ref loc, _) |
            Node::TyGenargs(ref loc, _, _) |
            Node::TyGendecl(ref loc, _, _, _) |
            Node::TyGendeclarg(ref loc, _, _) |
            Node::TyGeninst(ref loc, _, _) |
            Node::TyHash(ref loc, _, _) |
            Node::TyInstance(ref loc) |
            Node::TyIvardecl(ref loc, _, _) |
            Node::TyNil(ref loc) |
            Node::TyNillable(ref loc, _) |
            Node::TyOr(ref loc, _, _) |
            Node::TypedArg(ref loc, _, _) |
            Node::TyProc(ref loc, _) |
            Node::TySelf(ref loc) |
            Node::TyTuple(ref loc, _) |
            Node::Undef(ref loc, _) |
            Node::XString(ref loc, _) |
            Node::Yield(ref loc, _) |
            Node::ZSuper(ref loc) =>
                unimplemented!()
        }
    }
}
