use std::io::{self, Write};
use std::path::PathBuf;
use std::rc::Rc;
use std::fs::File;
use ast::{parse, IntoNode, Ast, SourceFile, Diagnostic, Node, Loc, Level};
use config::StripConfig;
use debug;

#[derive(Debug)]
pub struct ByteRange(pub usize, pub usize);

#[derive(Debug)]
pub enum StripError {
    Io(io::Error),
    Syntax(Vec<Diagnostic>),
}

pub fn strip_file(path: PathBuf, config: &StripConfig) -> Result<(), StripError> {
    let source_file = Rc::new(SourceFile::open(path).map_err(StripError::Io)?);

    let remove = Strip::strip(source_file.clone())?;

    if config.annotate {
        return debug::annotate_file(&source_file, &remove).map_err(StripError::Io);
    }

    if remove.is_empty() {
        if config.print {
            print!("{}", source_file.source());
        }

        return Ok(());
    }

    let stripped = remove_byte_ranges(source_file.source(), remove);

    if config.print {
        print!("{}", stripped);
        Ok(())
    } else {
        File::create(source_file.filename())
            .and_then(|mut file| file.write_all(stripped.as_bytes()))
            .map_err(StripError::Io)
    }
}

fn trim_around_range(source: &str, ByteRange(start, end): ByteRange) -> ByteRange {
    // first try to expand the byte range to the end of its line:
    let (trimmed_end, nl) = source[end..].char_indices()
        .skip_while(|&(_, c)| line_whitespace(c))
        .nth(0)
        .map(|(i, c)| (i + end, c))
        .unwrap_or((source.len(), '\n'));

    if !newline(nl) {
        // range was not at the end of the line
        return ByteRange(start, end);
    }

    // if the range was at the end of its line, it's safe to try trimming
    // whitespace backwards from the start:
    let trimmed_start = source[..start].char_indices().rev()
        .take_while(|&(_, c)| line_whitespace(c))
        .last()
        .map(|(i, _)| i)
        .unwrap_or(start);

    return ByteRange(trimmed_start, trimmed_end);

    fn newline(c: char) -> bool {
        c == '\r' || c == '\n'
    }

    fn line_whitespace(c: char) -> bool {
        !newline(c) && (c == ' ' || c == '\t')
    }
}

fn strip_trailing_whitespace(source: &str, remove: Vec<ByteRange>) -> Vec<ByteRange> {
    remove.into_iter().map(|range| trim_around_range(source, range)).collect()
}

fn remove_byte_ranges(source: &str, mut remove: Vec<ByteRange>) -> String {
    let source = source.as_bytes();
    let mut result : Vec<u8> = Vec::new();
    let mut src_pos : usize = 0;

    remove.sort_by_key(|&ByteRange(start, _)| start);

    for ByteRange(start, end) in remove {
        if src_pos < start {
            result.extend_from_slice(&source[src_pos..start]);
        }
        if end > src_pos {
            src_pos = end;
        }
    }

    if src_pos < source.len() {
        result.extend_from_slice(&source[src_pos..])
    }

    String::from_utf8(result).expect("malformed UTF8 when processing file")
}

pub struct Strip {
    remove: Vec<ByteRange>,
}

impl Strip {
    fn new() -> Self {
        Strip { remove: Vec::new() }
    }

    pub fn strip(file: Rc<SourceFile>) -> Result<Vec<ByteRange>, StripError> {
        let Ast { node, diagnostics } = parse(file.clone());

        // Return early if any errors found
        if diagnostics.iter().any(|d| d.level == Level::Error) {
            return Err(StripError::Syntax(diagnostics));
        }

        // Handle empty source file
        let node = match node {
            Some(node) => node,
            None => return Ok(vec![]),
        };

        let mut strip = Strip::new();
        strip.strip_node(&node);

        let remove = strip_trailing_whitespace(file.source(), strip.remove);

        Ok(remove)
    }

    fn remove(&mut self, loc: &Loc) {
        self.remove.push(ByteRange(loc.begin_pos, loc.end_pos));
    }

    fn remove_around(&mut self, enclosing: &Loc, inner: &Loc) {
        assert!(enclosing.begin_pos <= inner.begin_pos);
        assert!(enclosing.end_pos >= inner.end_pos);

        if enclosing.begin_pos < inner.begin_pos {
            self.remove.push(ByteRange(enclosing.begin_pos, inner.begin_pos));
        }

        if inner.end_pos < enclosing.end_pos {
            self.remove.push(ByteRange(inner.end_pos, enclosing.end_pos));
        }
    }

    fn strip_nodes(&mut self, nodes: &[Rc<Node>]) {
        for n in nodes {
            self.strip_node(n)
        }
    }

    fn remove_node(&mut self, node: &Option<Rc<Node>>) {
        match *node {
            Some(ref node) => self.remove(node.loc()),
            None => {},
        }
    }

    fn is_anonymous_block(&self, node: Option<&Rc<Node>>) -> bool {
        match node {
            Some(node) => {
                match **node {
                    Node::TyTypedArg(_, _, ref arg) => self.is_anonymous_block(Some(arg)),
                    Node::Blockarg(_, ref id) => id.is_none(),
                    _ => false,
                }
            },
            None => false,
        }
    }

    fn strip_node<'a, T: IntoNode<'a>>(&mut self, node: T) {
        let node = match node.into_node() {
            Some(node) => node,
            None => return,
        };

        match *node {
            Node::TyCpath(ref loc, _) |
            Node::TyGenargs(ref loc, _, _) |
            Node::TyGendeclarg(ref loc, _, _) |
            Node::TyIvardecl(ref loc, _, _) => {
                self.remove(loc)
            }
            Node::TyTypedArg(ref loc, _, ref arg) => {
                self.remove_around(loc, arg.loc());
            }
            Node::TyPrototype(_, ref genargs, ref args, ref ret) => {
                self.remove_node(genargs);
                self.strip_node(args);
                self.remove_node(ret);
            }
            Node::TyConstInstance(ref loc, ref cons, _) => {
                self.remove_around(loc, cons.loc());
            }
            Node::TyGendecl(ref loc, ref args, _, _) => {
                self.remove_around(loc, args.loc());
            }
            Node::TyCast(ref loc, ref expr, _) => {
                self.remove_around(loc, expr.loc());
            }

            Node::TyAny(..) |
            Node::TyArray(..) |
            Node::TyClass(..) |
            Node::TyConSubtype(..) |
            Node::TyGeninst(..) |
            Node::TyHash(..) |
            Node::TyInstance(..) |
            Node::TyNil(..) |
            Node::TyNillable(..) |
            Node::TyOr(..) |
            Node::TyParen(..) |
            Node::TyProc(..) |
            Node::TyReturnSig(..) |
            Node::TySelf(..) |
            Node::TyTuple(..) => {
                panic!("node {:?} should be unreachable by prune", node);
            }

            Node::Args(ref loc, ref nodes) => {
                let blockarg = nodes.last();

                if self.is_anonymous_block(blockarg) {
                    match nodes.len() {
                        0 => unreachable!(),
                        1 => self.remove(loc),
                        _ => {
                            let before = nodes.get(nodes.len() - 2);
                            self.remove.push(ByteRange(
                                before.unwrap().loc().end_pos,
                                blockarg.unwrap().loc().end_pos
                            ));
                            self.strip_nodes(&nodes[0..nodes.len()-1]);
                        }
                    }
                } else {
                    self.strip_nodes(nodes);
                }
            }

            Node::Alias(_, ref a, ref b) |
            Node::And(_, ref a, ref b) |
            Node::AndAsgn(_, ref a, ref b) |
            Node::EFlipflop(_, ref a, ref b) |
            Node::ERange(_, ref a, ref b) |
            Node::IFlipflop(_, ref a, ref b) |
            Node::IRange(_, ref a, ref b) |
            Node::Masgn(_, ref a, ref b) |
            Node::MatchAsgn(_, ref a, ref b) |
            Node::Or(_, ref a, ref b) |
            Node::OrAsgn(_, ref a, ref b) |
            Node::OpAsgn(_, ref a, _, ref b) |
            Node::Pair(_, ref a, ref b) |
            Node::UntilPost(_, ref a, ref b) |
            Node::WhilePost(_, ref a, ref b) => {
                self.strip_node(a);
                self.strip_node(b);
            }
            Node::When(_, ref a, ref b) => {
                self.strip_nodes(a);
                self.strip_node(b);
            }
            Node::Ensure(_, ref a, ref b) => {
                self.strip_node(a);
                self.strip_node(b);
            }
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
            Node::Kwarg(..) |
            Node::Backref(..) |
            Node::Blockarg(..) |
            Node::Cbase(..) |
            Node::Complex(..) |
            Node::Cvar(..) |
            Node::CvarLhs(..) |
            Node::EncodingLiteral(..) |
            Node::False(..) |
            Node::FileLiteral(..) |
            Node::Float(..) |
            Node::Gvar(.. ) |
            Node::GvarLhs(..) |
            Node::Ident(..) |
            Node::Integer(..) |
            Node::Ivar(..) |
            Node::IvarLhs(..) |
            Node::Kwrestarg(..) |
            Node::Lambda(..) |
            Node::LineLiteral(..) |
            Node::Lvar(..) |
            Node::LvarLhs(..) |
            Node::Nil(..) |
            Node::NthRef(..) |
            Node::Rational(..) |
            Node::Redo(..) |
            Node::Regopt(..) |
            Node::Restarg(..) |
            Node::Retry(..) |
            Node::Self_(..) |
            Node::ShadowArg(..) |
            Node::String(..) |
            Node::Symbol(..) |
            Node::True(..) |
            Node::ZSuper(..) => {
                // No-op
            }

            Node::Array(_, ref nodes) |
            Node::Hash(_, ref nodes) |
            Node::Begin(_, ref nodes) |
            Node::Break(_, ref nodes) |
            Node::DString(_, ref nodes) |
            Node::DSymbol(_, ref nodes) |
            Node::Kwbegin(_, ref nodes) |
            Node::Mlhs(_, ref nodes) |
            Node::Next(_, ref nodes) |
            Node::Regexp(_, ref nodes, _) |
            Node::Return(_, ref nodes) |
            Node::Super(_, ref nodes) |
            Node::Undef(_, ref nodes) |
            Node::XString(_, ref nodes) |
            Node::Yield(_, ref nodes) => {
                self.strip_nodes(nodes);
            }
            Node::Block(_, ref send, ref args, ref body) => {
                self.strip_node(send);
                self.strip_node(args);
                self.strip_node(body);
            }
            Node::BlockPass(_, ref node) |
            Node::CvarAsgn(_, _, ref node) |
            Node::Defined(_, ref node) |
            Node::GvarAsgn(_, _, ref node) |
            Node::IvarAsgn(_, _, ref node) |
            Node::Kwoptarg(_, _, ref node) |
            Node::Kwsplat(_, ref node) |
            Node::LvarAsgn(_, _, ref node) |
            Node::MatchCurLine(_, ref node) |
            Node::Optarg(_, _, ref node) |
            Node::Procarg0(_, ref node) |
            Node::Splat(_, ref node) => {
                self.strip_node(node);
            }
            Node::Const(_, ref node, _) |
            Node::ConstLhs(_, ref node, _) |
            Node::Postexe(_, ref node) |
            Node::Preexe(_, ref node) |
            Node::SplatLhs(_, ref node) => {
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
            Node::CSend(_, ref recv, _, ref args) |
            Node::Send(_, ref recv, _, ref args) => {
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
            Node::For(_, ref lhs, ref rhs, ref body) => {
                self.strip_node(lhs);
                self.strip_node(rhs);
                self.strip_node(body);
            }
            Node::If(_, ref cond, ref then, ref else_) => {
                self.strip_node(cond);
                self.strip_node(then);
                self.strip_node(else_);
            }
            Node::Resbody(_, ref class, ref var, ref body) => {
                self.strip_node(class);
                self.strip_node(var);
                self.strip_node(body);
            }
            Node::Rescue(_, ref body, ref resbodies, ref else_) => {
                self.strip_node(body);
                self.strip_nodes(resbodies);
                self.strip_node(else_);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate glob;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use super::*;

    fn strip(source: Rc<SourceFile>) -> String {
        let remove = Strip::strip(source.clone()).unwrap();
        remove_byte_ranges(source.source(), remove)
    }

    fn verify_stripped_syntax(source: Rc<SourceFile>) {
        let stripped = strip(source.clone());

        let mut ruby_child = Command::new("ruby")
            .arg("-c")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn().ok().expect("Failed to spawn Ruby for syntax checking");

        ruby_child.stdin.as_mut().unwrap()
            .write_all(stripped.as_bytes()).unwrap();

        let ruby_result = ruby_child.wait_with_output().unwrap();
        assert!(ruby_result.status.success(),
            "stripped syntax for '{}' is not valid Ruby.\nruby -c output:\n\n{}\n",
            source.filename().display(), String::from_utf8_lossy(&ruby_result.stderr)
        );
    }

    fn verify_stripped_sources(path: &str) {
        for path in glob::glob(path).unwrap().filter_map(Result::ok) {
            println!("checking: {}...", path.display());
            let source = Rc::new(SourceFile::open(path.clone())
                .expect("failed to open source"));
            verify_stripped_syntax(source);
        }
    }

    #[test]
    fn test_annotation_stripping() {
        use std::env;

        verify_stripped_sources("tests/fixtures/*.rb");
        verify_stripped_sources("definitions/lib/*.rb");

        match env::var("TYPEDRUBY_EXTRA_SOURCES") {
            Ok(ref path) => verify_stripped_sources(path),
            Err(..) => {},
        };
    }

    #[test]
    fn strips_trailing_whitespace() {
        let source = Rc::new(SourceFile::new("(test)".into(), "def foo => T  \n\n  123\nend".to_owned()));

        let stripped = strip(source);

        assert_eq!("def foo\n\n  123\nend", stripped);
    }
}
