use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::rc::Rc;
use std::iter;

use itertools::Itertools;
use serde_json;

use ast::{parse, IntoNode, Id, Ast, SourceFile, Diagnostic, Node, Level};

struct Annotation<'a> {
    line: usize,
    method: Cow<'a, str>,
    args: Vec<Cow<'a, str>>,
    retn: Cow<'a, str>,
}

#[derive(Debug)]
pub enum AnnotateError {
    Io(io::Error),
    Json(serde_json::Error),
    Syntax(Vec<Diagnostic>),
}

pub fn apply_annotations(path: &Path) -> Result<(), AnnotateError> {
    let mut buff = String::new();

    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buff))
        .map_err(AnnotateError::Io)?;

    let annos = load_annotations(&buff)?;

    for (file, annos) in annos {
        annotate_file(&file, annos)?;
    }

    Ok(())
}

fn load_annotations<'a>(buff: &'a str) -> Result<HashMap<Cow<'a, Path>, Vec<Annotation<'a>>>, AnnotateError> {
    #[derive(Deserialize)]
    struct AnnotationLine<'a> {
        file: Cow<'a, Path>,
        line: usize,
        method: Cow<'a, str>,
        args: Vec<Cow<'a, str>>,
        retn: Cow<'a, str>,
    }

    let mut annos = HashMap::new();

    for line in buff.lines() {
        let anno = serde_json::from_str::<AnnotationLine>(line)
            .map_err(AnnotateError::Json)?;

        annos.entry(anno.file)
            .or_insert(Vec::new())
            .push(Annotation {
                line: anno.line,
                method: anno.method,
                args: anno.args,
                retn: anno.retn,
            });
    }

    Ok(annos)
}

fn annotate_file<'a>(path: &Path, annos: Vec<Annotation<'a>>)
    -> Result<(), AnnotateError>
{
    let source_file = Rc::new(SourceFile::open(path.to_owned()).map_err(AnnotateError::Io)?);

    let insertions = Annotate::annotate(annos, Rc::clone(&source_file))?;

    let annotated = insert(source_file.source(), &insertions);

    println!("{}", annotated);

    Ok(())
}

fn insert<'a>(buffer: &str, insertions: &[Insertion<'a>]) -> String {
    iter::once(0)
        .chain(insertions.iter().map(|ins| ins.byte_pos))
        .chain(iter::once(buffer.len()))
        .tuple_windows()
        .map(|(begin, end)| &buffer[begin..end])
        .interleave(insertions.iter().map(|ins| &*ins.string))
        .fold(String::new(), |out, part| out + part)
}

#[derive(Debug)]
struct Insertion<'a> {
    byte_pos: usize,
    string: Cow<'a, str>,
}

struct Annotate<'a> {
    annotations: Vec<Annotation<'a>>, /* reverse sorted */
    ignored: Vec<Annotation<'a>>,
    insert: Vec<Insertion<'a>>,
}

impl<'a> Annotate<'a> {
    fn new(mut annotations: Vec<Annotation<'a>>) -> Self {
        // reverse sort by line so we can efficiently 'iterate' the list of
        // annotations by popping rather than shifting:
        annotations.sort_by(|a, b| b.line.cmp(&a.line));

        Annotate {
            annotations: annotations,
            ignored: Vec::new(),
            insert: Vec::new(),
        }
    }

    pub fn annotate(annotations: Vec<Annotation<'a>>, file: Rc<SourceFile>)
        -> Result<Vec<Insertion<'a>>, AnnotateError>
    {
        let Ast { node, diagnostics } = parse(file.clone());

        // Return early if any errors found
        if diagnostics.iter().any(|d| d.level == Level::Error) {
            return Err(AnnotateError::Syntax(diagnostics));
        }

        let mut anno = Self::new(annotations);
        anno.annotate_node(&node);

        Ok(anno.insert)
    }

    fn annotate_nodes(&mut self, nodes: &[Rc<Node>]) {
        for n in nodes {
            self.annotate_node(n)
        }
    }

    fn annotation_for_def(&mut self, line: usize, name: &str) -> Option<Annotation<'a>> {
        // remove any annotations that cannot possibly be matched anymore:
        while let Some(true) = self.annotations.last().map(|anno| anno.line < line) {
            self.annotations.pop();
        }

        self.annotations
            .iter()
            .enumerate()
            .rev()
            .take_while(|&(_, anno)| anno.line == line)
            .find(|&(_, anno)| anno.method == name)
            .map(|(index, _)| index)
            .map(|index| {
                // this removes the matched annotation from the annotations
                // vector so we an return an owned annotation without holding
                // a mut borrow alive.
                //
                // despite swap_remove not preserving vec order (it replaces
                // the removed element with the last element), because we
                // already popped all annotations not belonging to the current
                // line, this is guaranteed to replace our removed element with
                // another element of the same line, preserving our line number
                // sorted invariant on the annotations vec.
                self.annotations.swap_remove(index)
            })
    }

    fn annotate_def(&mut self, def: &Node, name: &Id, args: Option<&Node>) {
        let line = def.loc().begin().line;

        let anno = match self.annotation_for_def(line, &name.1) {
            Some(anno) => anno,
            None => return,
        };

        let retn_loc = match args {
            None => {
                // no args, nothing to annotate
                // TODO perhaps warn if the annotation specifies args but the
                // source file has none?
                Some(name.0.end_pos)
            }
            Some(&Node::Args(ref args_loc, ref arg_nodes)) => {
                for (node, ty) in arg_nodes.iter().zip(anno.args) {
                    let pos = node.loc().begin_pos;

                    self.insert.push(Insertion {
                        byte_pos: pos,
                        string: ty,
                    });

                    self.insert.push(Insertion {
                        byte_pos: pos,
                        string: Cow::Borrowed(" "),
                    });
                }

                Some(args_loc.end_pos)
            }
            Some(&Node::TyPrototype(..)) => {
                // already annotated, don't reannotate
                // TODO perhaps warn on this case?
                None
            }
            _ => panic!("unexpected node type in args position"),
        };

        if let Some(retn_loc) = retn_loc {
            self.insert.push(Insertion {
                byte_pos: retn_loc,
                string: Cow::Borrowed(" => "),
            });

            self.insert.push(Insertion {
                byte_pos: retn_loc,
                string: anno.retn,
            })
        }
    }

    fn annotate_node<'n, T: IntoNode<'n>>(&mut self, node: T) {
        let node = match node.into_node() {
            Some(node) => node,
            None => return,
        };

        match *node {
            Node::Def(_, ref name, ref args, ref body) => {
                self.annotate_def(node, name, args.as_ref().map(Rc::as_ref));
                self.annotate_node(body);
            }
            Node::Defs(_, ref definee, ref name, ref args, ref body) => {
                self.annotate_node(definee);
                self.annotate_def(node, name, args.as_ref().map(Rc::as_ref));
                self.annotate_node(body);
            }
            Node::TyTypedArg(..) |
            Node::TyPrototype(..) |
            Node::Args(..) => {
                panic!("should be unreachable!")
            }

            Node::TyCpath(..) |
            Node::TyGenargs(..) |
            Node::TyGendeclarg(..) |
            Node::TyIvardecl(..) |
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
            Node::TyTuple(..) |
            Node::TyConstInstance(..) |
            Node::TyGendecl(..) => {
                // cannot contain defs
            }

            Node::TyCast(_, ref expr, _) => {
                self.annotate_node(expr);
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
                self.annotate_node(a);
                self.annotate_node(b);
            }
            Node::When(_, ref a, ref b) => {
                self.annotate_nodes(a);
                self.annotate_node(b);
            }
            Node::Ensure(_, ref a, ref b) => {
                self.annotate_node(a);
                self.annotate_node(b);
            }
            Node::Until(_, ref a, ref b) |
            Node::While(_, ref a, ref b) |
            Node::SClass(_, ref a, ref b) |
            Node::Module(_, ref a, ref b) => {
                self.annotate_node(a);
                self.annotate_node(b);
            }
            Node::ConstAsgn(_, ref base, _, ref expr) => {
                self.annotate_node(base);
                self.annotate_node(expr);
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
                self.annotate_nodes(nodes);
            }
            Node::Block(_, ref send, ref args, ref body) => {
                self.annotate_node(send);
                self.annotate_node(args);
                self.annotate_node(body);
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
                self.annotate_node(node);
            }
            Node::Const(_, ref node, _) |
            Node::ConstLhs(_, ref node, _) |
            Node::Postexe(_, ref node) |
            Node::Preexe(_, ref node) |
            Node::SplatLhs(_, ref node) => {
                self.annotate_node(node);
            }
            Node::Case(_, ref scrut, ref whens, ref else_) => {
                self.annotate_node(scrut);
                self.annotate_nodes(whens);
                self.annotate_node(else_);
            }
            Node::Class(_, ref name, ref super_, ref body) => {
                self.annotate_node(name);
                self.annotate_node(super_);
                self.annotate_node(body);
            }
            Node::CSend(_, ref recv, _, ref args) |
            Node::Send(_, ref recv, _, ref args) => {
                self.annotate_node(recv);
                self.annotate_nodes(args);
            }
            Node::For(_, ref lhs, ref rhs, ref body) => {
                self.annotate_node(lhs);
                self.annotate_node(rhs);
                self.annotate_node(body);
            }
            Node::If(_, ref cond, ref then, ref else_) => {
                self.annotate_node(cond);
                self.annotate_node(then);
                self.annotate_node(else_);
            }
            Node::Resbody(_, ref class, ref var, ref body) => {
                self.annotate_node(class);
                self.annotate_node(var);
                self.annotate_node(body);
            }
            Node::Rescue(_, ref body, ref resbodies, ref else_) => {
                self.annotate_node(body);
                self.annotate_nodes(resbodies);
                self.annotate_node(else_);
            }
        }
    }
}
