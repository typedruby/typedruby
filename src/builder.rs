use ffi::{Token, Driver};
use std::rc::Rc;
use ast::{Node, Id, Loc, SourceFile};
use std::collections::HashSet;

pub struct Builder<'a> {
    pub driver: &'a mut Driver,
    pub emit_file_vars_as_literals: bool,
    pub cookie: usize,
}

fn collapse_string_parts(parts: &[Rc<Node>]) -> bool {
    if parts.len() == 1 {
        match *parts[0] {
            Node::DString(_, _) => true,
            Node::String(_, _) => true,
            _ => false,
        }
    } else {
        false
    }
}

fn check_duplicate_args_inner<'a>(names: &mut HashSet<&'a str>, arg: &'a Node) {
    let (_, name) = match *arg {
        // nodes that wrap other arg nodes:
        Node::Procarg0(_, ref arg) |
        Node::TypedArg(_, _, ref arg) => {
            return check_duplicate_args_inner(names, arg);
        },
        Node::Mlhs(_, ref mlhs_items) => {
            for mlhs_item in mlhs_items {
                check_duplicate_args_inner(names, mlhs_item);
            }
            return;
        },
        // normal arg nodes:
        Node::Arg(ref loc, ref name) => (loc, name),
        Node::Blockarg(_, None) => return,
        Node::Blockarg(_, Some(Id(ref loc, ref name))) => (loc, name),
        Node::Kwarg(ref loc, ref name) => (loc, name),
        Node::Kwoptarg(_, Id(ref loc, ref name), _) => (loc, name),
        Node::Kwrestarg(_, None) => return,
        Node::Kwrestarg(_, Some(Id(ref loc, ref name))) => (loc, name),
        Node::Optarg(_, Id(ref loc, ref name), _) => (loc, name),
        Node::Restarg(_, None) => return,
        Node::Restarg(_, Some(Id(ref loc, ref name))) => (loc, name),
        _ => panic!("not an arg node {:?}", arg),
    };

    if name.starts_with("_") {
        return;
    }

    if names.contains(name.as_str()) {
        // TODO error
    }

    names.insert(name);
}

fn check_duplicate_args<'a>(args: &[Rc<Node>]) {
    for arg in args {
        check_duplicate_args_inner(&mut HashSet::new(), arg);
    }
}

fn check_condition(cond: Rc<Node>) -> Rc<Node> {
    match *cond {
        Node::Begin(ref loc, ref stmts) => {
            if stmts.len() == 1 {
                Rc::new(Node::Begin(loc.clone(), vec![check_condition(stmts[0].clone())]))
            } else {
                cond.clone()
            }
        },

        Node::And(ref loc, ref a, ref b) =>
            Rc::new(Node::And(loc.clone(), check_condition(a.clone()), check_condition(b.clone()))),

        Node::Or(ref loc, ref a, ref b) =>
            Rc::new(Node::Or(loc.clone(), check_condition(a.clone()), check_condition(b.clone()))),

        Node::IRange(ref loc, ref a, ref b) =>
            Rc::new(Node::IFlipflop(loc.clone(), check_condition(a.clone()), check_condition(b.clone()))),

        Node::ERange(ref loc, ref a, ref b) =>
            Rc::new(Node::EFlipflop(loc.clone(), check_condition(a.clone()), check_condition(b.clone()))),

        Node::Regexp(ref loc, _, _) =>
            Rc::new(Node::MatchCurLine(loc.clone(), cond.clone())),

        _ => cond.clone(),
    }
}

struct Dedenter {
    dedent_level: usize,
    at_line_begin: bool,
    indent_level: usize,
}

impl Dedenter {
    fn new(dedent_level: usize) -> Dedenter {
        Dedenter {
            dedent_level: dedent_level,
            at_line_begin: true,
            indent_level: 0,
        }
    }

    fn dedent(&mut self, string: &str) -> String {
        let mut space_begin = 0;
        let mut space_end = 0;
        let mut offset = 0;

        let bytes = string.as_bytes();
        let mut result_bytes = bytes.to_vec();

        let mut index = 0;
        while index < bytes.len() {
            let c = bytes[index];
            if self.at_line_begin {
                if c == 10 /* \n */ || self.indent_level >= self.dedent_level {
                    result_bytes.drain(space_begin..space_end);
                    offset += space_end - space_begin;
                    self.at_line_begin = false;
                    if c == 10 /* \n */ {
                        continue; // redo current index
                    }
                }

                if c == 32 /* space */ {
                    self.indent_level += 1;
                    space_end += 1;
                }

                if c == 9 /* tab */ {
                    self.indent_level += 8 - (self.indent_level % 8);
                    space_end += 1;
                }
            } else if c == 10 /* \n */ && index == bytes.len() - 1 {
                self.at_line_begin = true;
                self.indent_level = 0;
                space_begin = index - offset;
                space_end = space_begin;
            }

            index += 1;
        }

        if self.at_line_begin {
            result_bytes.drain(space_begin..space_end);
        }

        String::from_utf8(result_bytes).unwrap()
    }

    fn interrupt(&mut self) {
        self.at_line_begin = false
    }

    fn dedent_parts(parts: &[Rc<Node>], mut dedenter: Dedenter) -> Vec<Rc<Node>> {
        parts.iter().map(|part| {
            match **part {
                Node::String(ref loc, ref val) => Rc::new(Node::String(loc.clone(), dedenter.dedent(val))),
                _ => { dedenter.interrupt(); part.clone() },
            }
        }).collect()
    }
}

fn join_exprs(exprs: &[Rc<Node>]) -> Loc {
    assert!(!exprs.is_empty());

    let a = exprs.first().unwrap();
    let b = exprs.last().unwrap();

    a.loc().join(b.loc())
}

enum CallType {
    Send,
    CSend,
}

fn call_type_for_dot(dot: Option<Token>) -> CallType {
    match dot {
        Some(dot) => {
            match dot.string().as_str() {
                "&." => CallType::CSend,
                _    => CallType::Send,
            }
        },
        None => CallType::Send,
    }
}

macro_rules! loc {
    ($self:expr, $tok:expr) => ({
        $tok.as_ref().unwrap().location($self.current_file())
    })
}

macro_rules! tok_split {
    ($self:expr, $tok:expr) => ({
        let tok = $tok.unwrap();
        let loc = tok.location($self.current_file());
        let s = tok.string();
        (loc, s)
    })
}

macro_rules! tok_id {
    ($self:expr, $tok:expr) => ({
        let (loc, name) = tok_split!($self, $tok);
        Id(loc, name)
    })
}

macro_rules! tok_join {
    ($self:expr, $left:expr, $right:expr) => ({
        loc!($self, $left).join(&loc!($self, $right))
    })
}

impl<'a> Builder<'a> {
    /*
     * Helpers
     */
    fn current_file(&self) -> Rc<SourceFile> {
        self.driver.current_file.clone()
    }

    fn collection_map(&self, begin: Option<Token>, elements: &[Rc<Node>], end: Option<Token>) -> Option<Loc> {
        if begin.is_some() {
            assert!(end.is_some());
            Some(tok_join!(self, begin, end))
        } else {
            assert!(end.is_none());
            if elements.is_empty() {
                None
            } else {
                let first = elements.first().unwrap();
                let last = elements.last().unwrap();
                Some(first.loc().join(last.loc()))
            }
        }
    }

    /*
     * Implementation
     */

    pub fn accessible(&self, node: Option<Rc<Node>>) -> Rc<Node> {
        let node = node.unwrap();

        match *node {
            Node::Ident(ref loc, ref name) => {
                if self.driver.is_declared(name) {
                    Rc::new(Node::Lvar(loc.clone(), name.clone()))
                } else {
                    Rc::new(Node::Send(loc.clone(), None, Id(loc.clone(), name.clone()), vec![]))
                }
            }
            _ => node.clone()
        }
    }

    pub fn alias(&self, alias: Option<Token>, to: Option<Rc<Node>>, from: Option<Rc<Node>>) -> Node {
        let to = to.unwrap();
        let from = from.unwrap();
        Node::Alias(loc!(self, alias).join(from.loc()), to, from)
    }

    pub fn arg(&self, name: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, name);
        Node::Arg(loc, id)
    }

    pub fn args(&self, begin: Option<Token>, args: Vec<Rc<Node>>, end: Option<Token>, check_args: bool) -> Node {
        if check_args {
            check_duplicate_args(args.as_slice());
        }

        let loc = self.collection_map(begin, args.as_slice(), end).unwrap_or(
            // FIXME - we don't have any location information to work with here:
            Loc { file: self.current_file(), begin_pos: 0, end_pos: 0 }
        );

        Node::Args(loc, args)
    }

    pub fn array(&self, begin: Option<Token>, elements: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::Array(self.collection_map(begin, elements.as_slice(), end).unwrap(), elements)
    }

    pub fn assign(&self, lhs: Option<Rc<Node>>, _eql: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = Rc::try_unwrap(lhs.unwrap()).expect("unique ownership of AST nodes during parse");
        let rhs = rhs.unwrap();
        let asgn_loc = lhs.loc().join(rhs.loc());

        match lhs {
            Node::Send(_, recv, mid, mut args) => {
                args.push(rhs);
                Node::Send(asgn_loc, recv, mid, args)
            },
            Node::CSend(_, recv, mid, mut args) => {
                args.push(rhs);
                Node::CSend(asgn_loc, recv, mid, args)
            },
            Node::LvarLhs(_, name) =>
                Node::LvarAsgn(asgn_loc, name, rhs),
            Node::ConstLhs(_, scope, name) =>
                Node::ConstAsgn(asgn_loc, scope, name, rhs),
            Node::CvarLhs(_, id) =>
                Node::CvarAsgn(asgn_loc, id, rhs),
            Node::IvarLhs(_, id) =>
                Node::IvarAsgn(asgn_loc, id, rhs),
            Node::GvarLhs(_, id) =>
                Node::GvarAsgn(asgn_loc, id, rhs),
            _ => {
                panic!("unimplemented lhs: {:?}", lhs);
            }
        }
    }

    pub fn assignable(&mut self, node: Option<Rc<Node>>) -> Node {
        let node = Rc::try_unwrap(node.unwrap()).expect("unique ownership of AST nodes during parse");
        match node {
            Node::Ident(loc, name) => {
                self.driver.declare(&name);
                Node::LvarLhs(loc.clone(), Id(loc.clone(), name))
            },
            Node::Ivar(loc, name) =>
                Node::IvarLhs(loc.clone(), Id(loc.clone(), name)),
            Node::Const(loc, lhs, name) =>
                Node::ConstLhs(loc.clone(), lhs, name),
            Node::Cvar(loc, name) =>
                Node::CvarLhs(loc.clone(), Id(loc.clone(), name)),
            Node::Gvar(loc, name) =>
                Node::GvarLhs(loc.clone(), Id(loc.clone(), name)),
            lhs =>
                panic!("not assignable on lhs: {:?}", lhs),
        }
    }

    pub fn associate(&self, begin: Option<Token>, pairs: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::Hash(self.collection_map(begin, &pairs, end).unwrap(), pairs)
    }

    pub fn attr_asgn(&self, receiver: Option<Rc<Node>>, dot: Option<Token>, selector: Option<Token>) -> Node {
        let recv = receiver.unwrap();
        let (sel_loc, sel_name) = tok_split!(self, selector);
        let selector = Id(sel_loc, sel_name + "=");
        let loc = recv.loc().join(&selector.0);

        // this builds an incomplete AST node:
        match call_type_for_dot(dot) {
            CallType::CSend => Node::CSend(loc, Some(recv), selector, vec![]),
            CallType::Send => Node::Send(loc, Some(recv), selector, vec![]),
        }
    }

    pub fn back_ref(&self, tok: Option<Token>) -> Node {
        let (loc, name) = tok_split!(self, tok);
        Node::Backref(loc, name)
    }

    pub fn begin(&self, begin: Option<Token>, body: Option<Rc<Node>>, end: Option<Token>) -> Rc<Node> {
        let loc = match begin {
            Some(_) => {
                assert!(end.is_some());
                tok_join!(self, begin, end)
            },
            None => {
                assert!(end.is_none());
                match body {
                    Some(ref boxed_body) => boxed_body.loc().clone(),
                    None => panic!("expected body to not be None"),
                }
            }
        };

        match body {
            None => Rc::new(Node::Begin(loc, vec![])),
            Some(boxed_body) => {
                match *boxed_body {
                    Node::Begin(_, _) => boxed_body.clone(),
                    Node::Mlhs(_, _) => boxed_body.clone(),
                    _ => Rc::new(Node::Begin(loc, vec![boxed_body])),
                }
            },
        }
    }

    pub fn begin_body(&self,
        body: Option<Rc<Node>>, rescue_bodies: Vec<Rc<Node>>,
        else_tok: Option<Token>, else_: Option<Rc<Node>>,
        ensure_tok: Option<Token>, ensure: Option<Rc<Node>>) -> Option<Rc<Node>> {
        let mut compound_stmt = body;

        if !rescue_bodies.is_empty() {
            match else_ {
                Some(else_body) => {
                    let loc = {
                        let loc = else_body.loc();
                        match compound_stmt {
                            Some(ref body) => body.loc().join(loc),
                            None => loc.clone(),
                        }
                    };
                    compound_stmt = Some(Rc::new(Node::Rescue(loc, compound_stmt, rescue_bodies, Some(else_body))));
                },
                None => {
                    let loc = {
                        let loc = rescue_bodies.last().unwrap().loc();
                        match compound_stmt {
                            Some(ref body) => body.loc().join(loc),
                            None => loc.clone(),
                        }
                    };
                    compound_stmt = Some(Rc::new(Node::Rescue(loc, compound_stmt, rescue_bodies, None)));
                }
            }
        } else if let Some(else_body) = else_ {
            let mut stmts = match compound_stmt {
                Some(ref node) => match **node {
                    Node::Begin(_, ref begin_stmts) => begin_stmts.clone(),
                    _ => vec![node.clone()],
                },
                _ => vec![],
            };

            stmts.push(Rc::new(
                    Node::Begin(
                        loc!(self, else_tok).join(else_body.loc()),
                        vec![else_body])));

            compound_stmt = Some(Rc::new(Node::Begin(join_exprs(stmts.as_slice()), stmts)));
        }

        if let Some(ensure_box) = ensure {
            let loc = {
                let ensure_loc = ensure_box.loc();

                match compound_stmt {
                    Some(ref compound_stmt_box) => compound_stmt_box.loc().join(ensure_loc),
                    None => loc!(self, ensure_tok).join(ensure_loc),
                }
            };

            compound_stmt = Some(Rc::new(Node::Ensure(loc, compound_stmt, ensure_box)));
        }

        compound_stmt
    }

    pub fn begin_keyword(&self, begin: Option<Token>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let tokens = tok_join!(self, begin, end);
        match body {
            Some(node) => {
                match *node {
                    Node::Begin(_, ref beg) => Node::Kwbegin(tokens, beg.clone()),
                    _ => Node::Kwbegin(tokens, vec![node.clone()]),
                }
            },
            None => Node::Kwbegin(tokens, vec![])
        }
    }

    pub fn binary_op(&self, recv: Option<Rc<Node>>, oper: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let recv = recv.unwrap();
        let arg = arg.unwrap();
        Node::Send(recv.loc().join(arg.loc()), Some(recv), tok_id!(self, oper), vec![arg])
    }

    pub fn block(&self, method_call: Option<Rc<Node>>, _begin: Option<Token>, args: Option<Rc<Node>>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let method_call = method_call.unwrap();
        let args = args.unwrap();

        if let Node::Yield(_, _) = *method_call {
            // diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
        }

        match *method_call {
            Node::Send(_, _, _, ref args) |
                Node::CSend(_, _, _, ref args) |
                Node::Super(_, ref args) => {
                    if let Some(ref last_arg) = args.last() {
                        if let Node::BlockPass(ref _loc, _) = ***last_arg {
                            // diagnostic :error, :block_and_blockarg, nil, last_arg.loc.expression, [loc(begin_t)]
                        }
                    }
                },
            _ => (),
        }

        match *method_call {
            Node::Send(_, _, _, _) |
                Node::CSend(_, _, _, _) |
                Node::Super(_, _) |
                Node::ZSuper(_) |
                Node::Lambda(_) =>
                Node::Block(method_call.loc().join(&loc!(self, end)), method_call, args, body),
            _ => panic!("unknown method call node: {:?}", method_call),
        }
    }

    pub fn block_pass(&self, amper: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let arg = arg.unwrap();
        Node::BlockPass(loc!(self, amper).join(arg.loc()), arg)
    }

    pub fn blockarg(&self, amper: Option<Token>, name: Option<Token>) -> Node {
        match name {
            Some(_) => {
                let id = tok_id!(self, name);
                Node::Blockarg(loc!(self, amper).join(&id.0), Some(id))
            },
            None => Node::Blockarg(loc!(self, amper), None)
        }
    }

    pub fn call_lambda(&self, lambda: Option<Token>) -> Node {
        Node::Lambda(loc!(self, lambda))
    }

    pub fn call_method(&self, recv: Option<Rc<Node>>, dot: Option<Token>, selector: Option<Token>, _lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let loc = {
            let selector_loc = match selector {
                Some(_) => loc!(self, selector),
                // if there is no selector (in the case of the foo.() #call syntax)
                // syntactically there *must* be a dot:
                None => loc!(self, dot),
            };

            let loc_start = match recv {
                Some(ref node) => node.loc(),
                _ => &selector_loc,
            };

            if rparen.is_some() {
                loc_start.join(&loc!(self, rparen))
            } else if args.len() > 0 {
                loc_start.join(args.last().unwrap().loc())
            } else {
                loc_start.join(&selector_loc)
            }
        };

        let selector = match selector {
            Some(sel) => tok_id!(self, Some(sel)),
            None => Id(loc!(self, dot), "call".to_owned()),
        };

        match call_type_for_dot(dot) {
            CallType::CSend => Node::CSend(loc, recv, selector, args),
            CallType::Send => Node::Send(loc, recv, selector, args),
        }
    }

    pub fn case_(&self, case_: Option<Token>, expr: Option<Rc<Node>>, when_bodies: Vec<Rc<Node>>, _else_tok: Option<Token>, else_body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        Node::Case(tok_join!(self, case_, end), expr, when_bodies, else_body)
    }

    pub fn character(&self, char_: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, char_);
        Node::String(loc, id)
    }

    pub fn complex(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Complex(loc, id)
    }

    pub fn compstmt(&self, mut nodes: Vec<Rc<Node>>) -> Option<Rc<Node>> {
        match nodes.len() {
            0 => None,
            1 => Some(nodes.remove(0)),
            _ => Some(Rc::new(Node::Begin(join_exprs(nodes.as_slice()), nodes))),
        }
    }

    pub fn condition(&self, cond_tok: Option<Token>, cond: Option<Rc<Node>>, then: Option<Token>, if_true: Option<Rc<Node>>, else_: Option<Token>, if_false: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let cond = cond.unwrap();
        let mut loc = loc!(self, cond_tok).join(cond.loc());

        if then.is_some() {
            loc = loc.join(&loc!(self, then));
        }

        if let Some(ref true_branch) = if_true {
            loc = loc.join(true_branch.loc());
        }

        if else_ .is_some() {
            loc = loc.join(&loc!(self, else_));
        }

        if let Some(ref false_branch) = if_false {
            loc = loc.join(false_branch.loc());
        }

        if end.is_some() {
            loc = loc.join(&loc!(self, end));
        }

        Node::If(loc, check_condition(cond), if_true, if_false)
    }

    pub fn condition_mod(&self, if_true: Option<Rc<Node>>, if_false: Option<Rc<Node>>, cond: Option<Rc<Node>>) -> Node {
        let cond = cond.unwrap();
        let loc = cond.loc().join(if_true.as_ref().unwrap_or_else(|| if_false.as_ref().unwrap()).loc());

        Node::If(loc, check_condition(cond), if_true, if_false)
    }

    pub fn const_(&self, name: Option<Token>) -> Node {
        assert!(name.is_some());
        Node::Const(loc!(self, name), None, tok_id!(self, name))
    }

    pub fn const_fetch(&self, scope: Option<Rc<Node>>, _colon: Option<Token>, name: Option<Token>) -> Node {
        let scope = scope.unwrap();
        let loc = scope.loc().join(&loc!(self, name));

        Node::Const(loc, Some(scope), tok_id!(self, name))
    }

    pub fn const_global(&self, colon: Option<Token>, name: Option<Token>) -> Node {
        let loc = tok_join!(self, colon, name);
        Node::Const(loc, Some(Rc::new(Node::Cbase(loc!(self, colon)))), tok_id!(self, name))
    }

    pub fn const_op_assignable(&self, _node: Option<Rc<Node>>) -> Node {
        unimplemented!();
    }

    pub fn cvar(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Cvar(loc, id)
    }

    pub fn dedent_string(&self, node: Option<Rc<Node>>, dedent_level: usize) -> Rc<Node> {
        let node = node.unwrap();
        if dedent_level != 0 {
            let mut dedenter = Dedenter::new(dedent_level);
            match *node {
                Node::String(ref loc, ref val) =>
                    Rc::new(Node::String(loc.clone(), dedenter.dedent(val))),
                Node::DString(ref loc, ref parts) =>
                    Rc::new(Node::DString(loc.clone(), Dedenter::dedent_parts(parts, dedenter))),
                Node::XString(ref loc, ref parts) =>
                    Rc::new(Node::XString(loc.clone(), Dedenter::dedent_parts(parts, dedenter))),
                _ => panic!("unexpected node type"),
            }
        } else {
            node
        }
    }

    pub fn def_class(&self, class_: Option<Token>, name: Option<Rc<Node>>, _lt: Option<Token>, superclass: Option<Rc<Node>>, body: Option<Rc<Node>>, end_: Option<Token>) -> Node {
        Node::Class(tok_join!(self, class_, end_), name.unwrap(), superclass, body)
    }

    pub fn def_method(&self, def: Option<Token>, name: Option<Token>, args: Option<Rc<Node>>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let loc = tok_join!(self, def, end);
        Node::Def(loc, tok_id!(self, name), args, body)
    }

    pub fn def_module(&self, module: Option<Token>, name: Option<Rc<Node>>, body: Option<Rc<Node>>, end_: Option<Token>) -> Node {
        Node::Module(tok_join!(self, module, end_), name.unwrap(), body)
    }

    pub fn def_sclass(&self, class_: Option<Token>, _lshft: Option<Token>, expr: Option<Rc<Node>>, body: Option<Rc<Node>>, end_: Option<Token>) -> Node {
        Node::SClass(tok_join!(self, class_, end_), expr.unwrap(), body)
    }

    pub fn def_singleton(&self, def: Option<Token>, definee: Option<Rc<Node>>, _dot: Option<Token>, name: Option<Token>, args: Option<Rc<Node>>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let loc = tok_join!(self, def, end);
        Node::Defs(loc, definee.unwrap(), tok_id!(self, name), args, body)
    }

    pub fn encoding_literal(&self, tok: Option<Token>) -> Node {
        if self.emit_file_vars_as_literals {
            let loc = loc!(self, tok);
            Node::Const(loc.clone(),
                Some(
                    Rc::new(Node::Const(
                        loc.clone(), None,
                        Id(loc.clone(), "Encoding".to_string()),
                    ))
                ),
                Id(loc.clone(), "UTF_8".to_string())
            )
        } else {
            Node::EncodingLiteral(loc!(self, tok))
        }
    }

    pub fn false_(&self, tok: Option<Token>) -> Node {
        Node::False(loc!(self, tok))
    }

    pub fn file_literal(&self, tok: Option<Token>) -> Node {
        if self.emit_file_vars_as_literals {
            let loc = loc!(self, tok);
            let filename = loc.file.filename().to_str().unwrap();
            Node::String(loc.clone(), filename.to_string())
        } else {
            Node::FileLiteral(loc!(self, tok))
        }
    }

    pub fn float_(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Float(loc, id)
    }

    pub fn float_complex(&self, _tok: Option<Token>) -> Node {
        unimplemented!();
    }

    pub fn for_(&self, for_: Option<Token>, iterator: Option<Rc<Node>>, _in: Option<Token>, iteratee: Option<Rc<Node>>, _do: Option<Token>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let iterator = iterator.unwrap();
        let iteratee = iteratee.unwrap();
        Node::For(loc!(self, for_).join(&loc!(self, end)), iterator, iteratee, body)
    }

    pub fn gvar(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Gvar(loc, id)
    }

    pub fn ident(&self, tok: Option<Token>) -> Node {
        let (tok, id) = tok_split!(self, tok);
        Node::Ident(tok, id)
    }

    pub fn index(&self, recv: Option<Rc<Node>>, lbrack: Option<Token>, indexes: Vec<Rc<Node>>, rbrack: Option<Token>) -> Node {
        let recv = recv.unwrap();
        let id = Id(tok_join!(self, lbrack, rbrack), "[]".to_owned());
        Node::Send(recv.loc().join(&loc!(self, rbrack)), Some(recv), id, indexes)
    }

    pub fn index_asgn(&self, recv: Option<Rc<Node>>, lbrack: Option<Token>, indexes: Vec<Rc<Node>>, rbrack: Option<Token>) -> Node {
        let recv = recv.unwrap();
        let id = Id(tok_join!(self, lbrack, rbrack), "[]=".to_owned());
        Node::Send(recv.loc().clone(), Some(recv), id, indexes)
    }

    pub fn integer(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Integer(loc, id)
    }

    pub fn ivar(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Ivar(loc, id)
    }

    pub fn keyword_break(&self, keyword: Option<Token>, lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let mut loc = loc!(self, keyword);
        if let Some(operand_loc) = self.collection_map(lparen, args.as_slice(), rparen) {
            loc = loc.join(&operand_loc);
        }

        Node::Break(loc, args)
    }

    pub fn keyword_defined(&self, keyword: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let arg = arg.unwrap();
        Node::Defined(loc!(self, keyword).join(arg.loc()), arg)
    }

    pub fn keyword_next(&self, keyword: Option<Token>, lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let mut loc = loc!(self, keyword);
        if let Some(operand_loc) = self.collection_map(lparen, args.as_slice(), rparen) {
            loc = loc.join(&operand_loc);
        }

        Node::Next(loc, args)
    }

    pub fn keyword_redo(&self, keyword: Option<Token>) -> Node {
        Node::Redo(loc!(self, keyword))
    }

    pub fn keyword_retry(&self, keyword: Option<Token>) -> Node {
        Node::Retry(loc!(self, keyword))
    }

    pub fn keyword_return(&self, keyword: Option<Token>, lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let mut loc = loc!(self, keyword);
        if let Some(operand_loc) = self.collection_map(lparen, args.as_slice(), rparen) {
            loc = loc.join(&operand_loc);
        }
        Node::Return(loc, args)
    }

    pub fn keyword_super(&self, keyword: Option<Token>, lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let mut loc = loc!(self, keyword);
        if let Some(operand_loc) = self.collection_map(lparen, args.as_slice(), rparen) {
            loc = loc.join(&operand_loc);
        }
        Node::Super(loc, args)
    }

    pub fn keyword_yield(&self, keyword: Option<Token>, lparen: Option<Token>, args: Vec<Rc<Node>>, rparen: Option<Token>) -> Node {
        let mut loc = loc!(self, keyword);
        if let Some(operand_loc) = self.collection_map(lparen, args.as_slice(), rparen) {
            loc = loc.join(&operand_loc);
        }
        Node::Yield(loc, args)
    }

    pub fn keyword_zsuper(&self, keyword: Option<Token>) -> Node {
        Node::ZSuper(loc!(self, keyword))
    }

    pub fn kwarg(&self, name: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, name);
        Node::Kwarg(loc, id)
    }

    pub fn kwoptarg(&self, name: Option<Token>, value: Option<Rc<Node>>) -> Node {
        let value = value.unwrap();
        let id = tok_id!(self, name);
        Node::Kwoptarg(id.0.join(value.loc()), id, value)
    }

    pub fn kwrestarg(&self, dstar: Option<Token>, name: Option<Token>) -> Node {
        match name {
            Some(_) => {
                let id = tok_id!(self, name);
                Node::Kwrestarg(loc!(self, dstar).join(&id.0), Some(id))
            },
            None => Node::Kwrestarg(loc!(self, dstar), None),
        }
    }

    pub fn kwsplat(&self, dstar: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let arg = arg.unwrap();
        Node::Kwsplat(loc!(self, dstar).join(arg.loc()), arg)
    }

    pub fn line_literal(&self, tok: Option<Token>) -> Node {
        if self.emit_file_vars_as_literals {
            let loc = loc!(self, tok);
            let line = loc.file.line_for_pos(loc.begin_pos);
            Node::Integer(loc.clone(), line.number.to_string())
        } else {
            Node::LineLiteral(loc!(self, tok))
        }
    }

    pub fn logical_and(&self, lhs: Option<Rc<Node>>, _op: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();
        Node::And(lhs.loc().join(rhs.loc()), lhs, rhs)
    }

    pub fn logical_or(&self, lhs: Option<Rc<Node>>, _op: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();
        Node::Or(lhs.loc().join(rhs.loc()), lhs, rhs)
    }

    pub fn loop_until(&self, keyword: Option<Token>, cond: Option<Rc<Node>>, _do: Option<Token>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let cond = cond.unwrap();
        Node::Until(tok_join!(self, keyword, end), cond, body)
    }

    pub fn loop_until_mod(&self, body: Option<Rc<Node>>, cond: Option<Rc<Node>>) -> Node {
        let cond = cond.unwrap();
        let body = body.unwrap();
        let loc = body.loc().join(cond.loc());

        match *body {
            Node::Kwbegin(_, _) => Node::UntilPost(loc, cond, body),
            _ => Node::Until(loc, cond, Some(body))
        }
    }

    pub fn loop_while(&self, keyword: Option<Token>, cond: Option<Rc<Node>>, _do: Option<Token>, body: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let cond = cond.unwrap();
        Node::While(tok_join!(self, keyword, end), cond, body)
    }

    pub fn loop_while_mod(&self, body: Option<Rc<Node>>, cond: Option<Rc<Node>>) -> Node {
        let cond = cond.unwrap();
        let body = body.unwrap();
        let loc = body.loc().join(cond.loc());

        match *body {
            Node::Kwbegin(_, _) => Node::WhilePost(loc, cond, body),
            _ => Node::While(loc, cond, Some(body))
        }
    }

    pub fn match_op(&self, recv: Option<Rc<Node>>, oper: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let recv = recv.unwrap();
        let arg = arg.unwrap();

        if let Node::Regexp(_, ref _parts, _) = *recv {
            // TODO if parts are all static string literals, declare any named
            // captures as local variables and emit MatchWithLvasgn node
        }

        Node::Send(recv.loc().join(arg.loc()), Some(recv), tok_id!(self, oper), vec![arg])
    }

    pub fn multi_assign(&self, mlhs: Option<Rc<Node>>, rhs: Option<Rc<Node>>) -> Node {
        let mlhs = mlhs.unwrap();
        let rhs = rhs.unwrap();
        Node::Masgn(mlhs.loc().join(rhs.loc()), mlhs, rhs)
    }

    pub fn multi_lhs(&self, begin: Option<Token>, items: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::Mlhs(self.collection_map(begin, items.as_slice(), end).unwrap(), items)
    }

    pub fn negate(&self, uminus: Option<Token>, numeric: Option<Rc<Node>>) -> Node {
        let numeric = numeric.unwrap();
        let loc = loc!(self, uminus).join(numeric.loc());

        match *numeric {
            Node::Integer(_, ref value) => Node::Integer(loc, "-".to_owned() + value.as_str()),
            Node::Float(_, ref value) => Node::Float(loc, "-".to_owned() + value.as_str()),
            _ => panic!("unimplemented numeric type: {:?}", numeric),
        }
    }

    pub fn nil(&self, tok: Option<Token>) -> Node {
        Node::Nil(loc!(self, tok))
    }

    pub fn not_op(&self, not_: Option<Token>, begin: Option<Token>, receiver: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let not_loc = loc!(self, not_);
        let id = Id(loc!(self, not_), "!".to_owned());

        match receiver {
            Some(expr) => {
                let loc = match end {
                    Some(_) => not_loc.join(&loc!(self, end)),
                    None => not_loc.join(expr.loc()),
                };
                Node::Send(loc, Some(expr), id, vec![])
            },
            None => {
                assert!(begin.is_some() && end.is_some());
                let nil_loc = tok_join!(self, begin, end);
                let loc = not_loc.join(&nil_loc);
                let recv = Rc::new(Node::Begin(nil_loc.clone(), vec![Rc::new(Node::Nil(nil_loc))]));
                Node::Send(loc, Some(recv), id, vec![])
            }
        }
    }

    pub fn nth_ref(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::NthRef(loc, id.parse().unwrap())
    }

    pub fn op_assign(&self, lhs: Option<Rc<Node>>, op: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();
        let op = op.unwrap();

        // match lhs {
        //  TODO error on back ref and nth ref
        // }

        match op.string().as_str() {
            "&&" => Node::AndAsgn(lhs.loc().join(rhs.loc()), lhs, rhs),
            "||" => Node::OrAsgn(lhs.loc().join(rhs.loc()), lhs, rhs),
            _    => Node::OpAsgn(lhs.loc().join(rhs.loc()), lhs, tok_id!(self, Some(op)), rhs),
        }
    }

    pub fn optarg(&self, name: Option<Token>, _eql: Option<Token>, value: Option<Rc<Node>>) -> Node {
        let id = tok_id!(self, name);
        let value = value.unwrap();
        Node::Optarg(id.0.join(value.loc()), id, value)
    }

    pub fn pair(&self, key: Option<Rc<Node>>, _assoc: Option<Token>, value: Option<Rc<Node>>) -> Node {
        let key = key.unwrap();
        let value = value.unwrap();
        Node::Pair(key.loc().join(value.loc()), key, value)
    }

    pub fn pair_keyword(&self, key: Option<Token>, value: Option<Rc<Node>>) -> Node {
        let value = value.unwrap();
        let (loc, id) = tok_split!(self, key);
        let sym = Node::Symbol(loc, id);
        Node::Pair(sym.loc().join(value.loc()), Rc::new(sym), value)
    }

    pub fn pair_quoted(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>, value: Option<Rc<Node>>) -> Node {
        let key = self.symbol_compose(begin, parts, end);
        let value = value.unwrap();
        Node::Pair(key.loc().join(value.loc()), Rc::new(key), value)
    }

    pub fn postexe(&self, begin: Option<Token>, node: Option<Rc<Node>>, rbrace: Option<Token>) -> Node {
        Node::Postexe(loc!(self, begin).join(&loc!(self, rbrace)), node)
    }

    pub fn preexe(&self, begin: Option<Token>, node: Option<Rc<Node>>, rbrace: Option<Token>) -> Node {
        Node::Preexe(loc!(self, begin).join(&loc!(self, rbrace)), node)
    }

    pub fn procarg0(&self, arg: Option<Rc<Node>>) -> Node {
        let arg = arg.unwrap();
        Node::Procarg0(arg.loc().clone(), arg)
    }

    pub fn prototype(&self, genargs: Option<Rc<Node>>, args: Option<Rc<Node>>, return_type: Option<Rc<Node>>) -> Node {
        let args = args.unwrap();
        let mut loc = args.loc().clone();

        if let Some(ref genargs_) = genargs {
            loc = loc.join(genargs_.loc());
        }

        if let Some(ref return_type_) = return_type {
            loc = loc.join(return_type_.loc());
        }

        Node::Prototype(loc, genargs, args, return_type)
    }

    pub fn range_exclusive(&self, lhs: Option<Rc<Node>>, _oper: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        Node::ERange(lhs.loc().join(rhs.loc()), lhs, rhs)
    }

    pub fn range_inclusive(&self, lhs: Option<Rc<Node>>, _oper: Option<Token>, rhs: Option<Rc<Node>>) -> Node {
        let lhs = lhs.unwrap();
        let rhs = rhs.unwrap();

        Node::IRange(lhs.loc().join(rhs.loc()), lhs, rhs)
    }

    pub fn rational(&self, tok: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::Rational(loc, id)
    }

    pub fn rational_complex(&self, _tok: Option<Token>) -> Node {
        unimplemented!();
    }

    pub fn regexp_compose(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>, options: Option<Rc<Node>>) -> Node {
        let begin_loc = loc!(self, begin);
        let loc = match options {
            Some(ref opt_box) => begin_loc.join(opt_box.loc()),
            None => begin_loc.join(&loc!(self, end)),
        };
        Node::Regexp(loc, parts, options)
    }

    pub fn regexp_options(&self, regopt: Option<Token>) -> Node {
        let regopt = regopt.unwrap();
        let mut options: Vec<char> = regopt.string().chars().collect();
        options.sort();
        options.dedup();
        Node::Regopt(loc!(self, Some(regopt)), options)
    }

    pub fn rescue_body(&self, rescue: Option<Token>, exc_list: Option<Rc<Node>>, _assoc: Option<Token>, exc_var: Option<Rc<Node>>, _then: Option<Token>, body: Option<Rc<Node>>) -> Node {
        let mut loc = loc!(self, rescue);

        if let Some(ref boxed_exc_list) = exc_list {
            loc = loc.join(boxed_exc_list.loc());
        }

        if let Some(ref boxed_exc_var) = exc_var {
            loc = loc.join(boxed_exc_var.loc());
        }

        if let Some(ref boxed_body) = body {
            loc = loc.join(boxed_body.loc());
        }

        Node::Resbody(loc, exc_list, exc_var, body)
    }

    pub fn restarg(&self, star: Option<Token>, name: Option<Token>) -> Node {
        match name {
            Some(_) => {
                let id = tok_id!(self, name);
                Node::Restarg(loc!(self, star).join(&id.0), Some(id))
            },
            None => Node::Restarg(loc!(self, star), None),
        }
    }

    pub fn self_(&self, tok: Option<Token>) -> Node {
        Node::Self_(loc!(self, tok))
    }

    pub fn shadowarg(&self, _name: Option<Token>) -> Node {
        unimplemented!();
    }

    pub fn splat(&self, star: Option<Token>, arg: Option<Rc<Node>>) -> Node {
        let loc = match arg {
            Some(ref box_arg) => loc!(self, star).join(box_arg.loc()),
            None => loc!(self, star),
        };
        Node::Splat(loc, arg)
    }

    pub fn string(&self, string_: Option<Token>) -> Node {
        Node::String(loc!(self, string_), string_.unwrap().string())
    }

    pub fn string_compose(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        let loc = self.collection_map(begin, parts.as_slice(), end).unwrap();

        if collapse_string_parts(&parts) {
            match *parts[0] {
                Node::String(ref loc, ref val) =>
                    Node::String(loc.clone(), val.clone()),
                    Node::DString(ref loc, ref val) =>
                        Node::DString(loc.clone(), val.clone()),
                    _ => Node::DString(loc.clone(), vec![parts[0].clone()]),
            }
        } else {
            Node::DString(loc, parts)
        }
    }

    pub fn string_internal(&self, string_: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, string_);
        Node::String(loc, id)
    }

    pub fn symbol(&self, symbol: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, symbol);
        Node::Symbol(loc, id)
    }

    pub fn symbol_compose(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        let loc = self.collection_map(begin, parts.as_slice(), end).unwrap();

        if collapse_string_parts(&parts) {
            match *parts[0] {
                Node::Symbol(ref loc, ref val) =>
                    Node::Symbol(loc.clone(), val.clone()),
                Node::String(ref loc, ref val) =>
                    Node::Symbol(loc.clone(), val.clone()),
                _ => Node::DSymbol(loc, vec![parts[0].clone()]),
            }
        } else {
            Node::DSymbol(loc, parts)
        }
    }

    pub fn symbol_internal(&self, symbol: Option<Token>) -> Node {
        let (loc, id) = tok_split!(self, symbol);
        Node::Symbol(loc, id)
    }

    pub fn symbols_compose(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        let parts = parts.iter().map(|part| {
            match **part {
                Node::String(ref loc, ref val) =>
                    Rc::new(Node::Symbol(loc.clone(), val.clone())),

                Node::DString(ref loc, ref parts) =>
                    Rc::new(Node::DSymbol(loc.clone(), parts.clone())),

                _ => part.clone(),
            }
        }).collect::<Vec<_>>();

        Node::Array(self.collection_map(begin, parts.as_slice(), end).unwrap(), parts)
    }

    pub fn ternary(&self, cond: Option<Rc<Node>>, _question: Option<Token>, if_true: Option<Rc<Node>>, _colon: Option<Token>, if_false: Option<Rc<Node>>) -> Node {
        let cond = cond.unwrap();
        let if_true = if_true.unwrap();
        let if_false = if_false.unwrap();

        Node::If(cond.loc().join(if_false.loc()), check_condition(cond), Some(if_true), Some(if_false))
    }

    pub fn tr_any(&self, special: Option<Token>) -> Node {
        Node::TyAny(loc!(self, special))
    }

    pub fn tr_array(&self, begin: Option<Token>, type_: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let type_ = type_.unwrap();
        Node::TyArray(tok_join!(self, begin, end), type_)
    }

    pub fn tr_cast(&self, begin: Option<Token>, expr: Option<Rc<Node>>, _colon: Option<Token>, type_: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let expr = expr.unwrap();
        let type_ = type_.unwrap();

        Node::TyCast(tok_join!(self, begin, end), expr, type_)
    }

    pub fn tr_class(&self, special: Option<Token>) -> Node {
        Node::TyClass(loc!(self, special))
    }

    pub fn tr_consubtype(&self, sub: Option<Rc<Node>>, super_: Option<Rc<Node>>) -> Node {
        let sub = sub.unwrap();
        let super_ = super_.unwrap();
        let loc = sub.loc().join(super_.loc());

        Node::TyConSubtype(loc, sub, super_)
    }

    pub fn tr_conunify(&self, a: Option<Rc<Node>>, b: Option<Rc<Node>>) -> Node {
        let a = a.unwrap();
        let b = b.unwrap();
        let loc = a.loc().join(b.loc());

        Node::TyConUnify(loc, a, b)
    }

    pub fn tr_cpath(&self, cpath: Option<Rc<Node>>) -> Node {
        let cpath = cpath.unwrap();
        Node::TyCpath(cpath.loc().clone(), cpath)
    }

    pub fn tr_genargs(&self, begin: Option<Token>, genargs: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::TyGenargs(tok_join!(self, begin, end), genargs)
    }

    pub fn tr_gendecl(&self, cpath: Option<Rc<Node>>, _begin: Option<Token>, genargs: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        let cpath = cpath.unwrap();
        Node::TyGendecl(cpath.loc().join(&loc!(self, end)), cpath, genargs)
    }

    pub fn tr_gendeclarg(&self, tok: Option<Token>, constraint: Option<Rc<Node>>) -> Node {
        let (loc, id) = tok_split!(self, tok);
        Node::TyGendeclarg(loc, id, constraint)
    }

    pub fn tr_geninst(&self, cpath: Option<Rc<Node>>, _begin: Option<Token>, genargs: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        let cpath = cpath.unwrap();
        Node::TyGeninst(cpath.loc().join(&loc!(self, end)), cpath, genargs)
    }

    pub fn tr_hash(&self, begin: Option<Token>, key_type: Option<Rc<Node>>, _assoc: Option<Token>, value_type: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let key_type = key_type.unwrap();
        let value_type = value_type.unwrap();

        Node::TyHash(tok_join!(self, begin, end), key_type, value_type)
    }

    pub fn tr_instance(&self, special: Option<Token>) -> Node {
        Node::TyInstance(loc!(self, special))
    }

    pub fn tr_ivardecl(&self, name: Option<Token>, type_: Option<Rc<Node>>) -> Node {
        let name = tok_id!(self, name);
        let type_ = type_.unwrap();

        Node::TyIvardecl(name.0.join(type_.loc()), name, type_)
    }

    pub fn tr_nil(&self, nil: Option<Token>) -> Node {
        Node::TyNil(loc!(self, nil))
    }

    pub fn tr_nillable(&self, tilde: Option<Token>, type_: Option<Rc<Node>>) -> Node {
        let type_ = type_.unwrap();
        Node::TyNillable(loc!(self, tilde).join(type_.loc()), type_)
    }

    pub fn tr_or(&self, a: Option<Rc<Node>>, b: Option<Rc<Node>>) -> Node {
        let a = a.unwrap();
        let b = b.unwrap();
        Node::TyOr(a.loc().join(b.loc()), a, b)
    }

    pub fn tr_proc(&self, begin: Option<Token>, args: Option<Rc<Node>>, end: Option<Token>) -> Node {
        let args = args.unwrap();
        Node::TyProc(tok_join!(self, begin, end), args)
    }

    pub fn tr_self(&self, special: Option<Token>) -> Node {
        Node::TySelf(loc!(self, special))
    }

    pub fn tr_tuple(&self, begin: Option<Token>, types: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::TyTuple(tok_join!(self, begin, end), types)
    }

    pub fn true_(&self, tok: Option<Token>) -> Node {
        Node::True(loc!(self, tok))
    }

    pub fn typed_arg(&self, type_: Option<Rc<Node>>, arg: Option<Rc<Node>>) -> Node {
        let type_ = type_.unwrap();
        let arg = arg.unwrap();
        Node::TypedArg(type_.loc().join(arg.loc()), type_, arg)
    }

    pub fn unary_op(&self, oper: Option<Token>, receiver: Option<Rc<Node>>) -> Node {
        let id = tok_id!(self, oper);
        let recv = receiver.unwrap();

        let id = match id.1.as_str() {
            "+" => Id(id.0, "+@".to_owned()),
            "-" => Id(id.0, "-@".to_owned()),
            _   => id,
        };

        Node::Send(id.0.join(recv.loc()), Some(recv), id, vec![])
    }

    pub fn undef_method(&self, undef: Option<Token>, name_list: Vec<Rc<Node>>) -> Node {
        let loc = match name_list.last() {
            Some(ref node) => loc!(self, undef).join(node.loc()),
            None => loc!(self, undef),
        };

        Node::Undef(loc, name_list)
    }

    pub fn when(&self, when: Option<Token>, patterns: Vec<Rc<Node>>, then: Option<Token>, body: Option<Rc<Node>>) -> Node {
        let when_loc = loc!(self, when);

        let loc = if let Some(ref body_box) = body {
            when_loc.join(body_box.loc())
        } else if then.is_some() {
            when_loc.join(&loc!(self, then))
        } else {
            when_loc.join(patterns.last().unwrap().loc())
        };

        Node::When(loc, patterns, body)
    }

    pub fn word(&self, parts: Vec<Rc<Node>>) -> Rc<Node> {
        if collapse_string_parts(&parts) {
            parts.clone().remove(0)
        } else {
            assert!(!parts.is_empty());
            let loc = parts.first().unwrap().loc().join(parts.last().unwrap().loc());
            Rc::new(Node::DString(loc, parts))
        }
    }

    pub fn words_compose(&self, begin: Option<Token>, words: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::Array(self.collection_map(begin, words.as_slice(), end).unwrap(), words)
    }

    pub fn xstring_compose(&self, begin: Option<Token>, parts: Vec<Rc<Node>>, end: Option<Token>) -> Node {
        Node::XString(self.collection_map(begin, parts.as_slice(), end).unwrap(), parts)
    }
}
