#![allow(unused_variables)]

extern crate libc;

use std::ptr;

use ast::*;
use ffi;
use ffi::{Builder, NodeList, Token, Parser};
use self::libc::size_t;
use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

thread_local!{
    static SOURCE_FILE: RefCell<Option<Rc<SourceFile>>> = RefCell::new(None);
}

fn current_file() -> Rc<SourceFile> {
    SOURCE_FILE.with(|sf| sf.borrow().as_ref().unwrap().clone())
}

trait ToRaw {
    fn to_raw(self) -> *mut Rc<Node>;
}

impl ToRaw for Option<Rc<Node>> {
    fn to_raw(self) -> *mut Rc<Node> {
        match self {
            None => ptr::null_mut(),
            Some(x) => x.to_raw(),
        }
    }
}

impl ToRaw for Rc<Node> {
    fn to_raw(self) -> *mut Rc<Node> {
        Box::into_raw(Box::new(self))
    }
}

impl ToRaw for Option<Node> {
    fn to_raw(self) -> *mut Rc<Node> {
        match self {
            None => ptr::null_mut(),
            Some(x) => Box::new(x).to_raw(),
        }
    }
}

impl ToRaw for Node {
    fn to_raw(self) -> *mut Rc<Node> {
        Box::into_raw(Box::new(Rc::new(self)))
    }
}

unsafe fn from_maybe_raw(p: *mut Rc<Node>) -> Option<Rc<Node>> {
    if p == ptr::null_mut() {
        None
    } else {
        Some(*Box::from_raw(p))
    }
}

fn join_exprs(exprs: &[Rc<Node>]) -> Loc {
    assert!(!exprs.is_empty());

    let a = exprs.first().unwrap();
    let b = exprs.last().unwrap();

    a.loc().join(b.loc())
}

unsafe fn join_tokens(left: *const Token, right: *const Token) -> Loc {
    token_loc(left).join(&token_loc(right))
}

enum CallType {
    Send,
    CSend,
}

unsafe fn call_type_for_dot(dot: *const Token) -> CallType {
    if dot == ptr::null() {
        return CallType::Send;
    }

    match Token::string(dot).as_str() {
        "&." => CallType::CSend,
        _    => CallType::Send,
    }
}

unsafe fn token_id(tok: *const Token) -> Id {
    Id(token_loc(tok), Token::string(tok))
}

unsafe fn token_loc(token: *const Token) -> Loc {
    let (begin, end) = Token::loc(token);

    Loc {
        file: current_file(),
        begin_pos: begin,
        end_pos: end,
    }
}

unsafe fn from_raw(p: *mut Rc<Node>) -> Rc<Node> {
    if p == ptr::null_mut() {
        panic!("received null node pointer in from_raw!");
    }

    *Box::from_raw(p)
}

unsafe extern "C" fn accessible(parser: *mut Parser, node: *mut Rc<Node>) -> *mut Rc<Node> {
    let node = from_raw(node);

    match *node {
        Node::Ident(ref loc, ref name) => {
            if Parser::is_declared(parser, name) {
                Node::Lvar(loc.clone(), name.clone())
            } else {
                Node::Send(loc.clone(), None, Id(loc.clone(), name.clone()), vec![])
            }
        }.to_raw(),
        _ => node.clone().to_raw(),
    }
}

unsafe extern "C" fn alias(alias: *const Token, to: *mut Rc<Node>, from: *mut Rc<Node>) -> *mut Rc<Node> {
    let to = from_raw(to);
    let from = from_raw(from);

    Node::Alias(token_loc(alias).join(from.loc()), to, from).to_raw()
}

unsafe extern "C" fn arg(name: *const Token) -> *mut Rc<Node> {
    Node::Arg(token_loc(name), Token::string(name)).to_raw()
}

fn check_duplicate_args_inner<'a>(names: &mut HashSet<&'a str>, arg: &'a Node) {
    let (range, name) = match *arg {
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

unsafe fn collection_map(begin: *const Token, elements: &[Rc<Node>], end: *const Token) -> Option<Loc> {
    if begin != ptr::null() {
        assert!(end != ptr::null());

        Some(join_tokens(begin, end))
    } else {
        assert!(end == ptr::null());

        if elements.is_empty() {
            None
        } else {
            let first = elements.first().unwrap();
            let last = elements.last().unwrap();
            Some(first.loc().join(last.loc()))
        }
    }
}

unsafe extern "C" fn args(begin: *const Token, args: *mut NodeList, end: *const Token, check_args: bool) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    if check_args {
        check_duplicate_args(args.as_slice());
    }

    let loc = collection_map(begin, args.as_slice(), end).unwrap_or(
        // FIXME - we don't have any location information to work with here:
        Loc { file: current_file(), begin_pos: 0, end_pos: 0 }
    );

    Node::Args(loc, args).to_raw()
}

unsafe extern "C" fn array(begin: *const Token, elements: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let elements = ffi::node_list_from_raw(elements);
    Node::Array(collection_map(begin, elements.as_slice(), end).unwrap(), elements).to_raw()
}

unsafe extern "C" fn assign(lhs: *mut Rc<Node>, eql: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = Rc::try_unwrap(from_raw(lhs)).expect("unique ownership of AST nodes during parse");
    let rhs = from_raw(rhs);

    let asgn_loc = lhs.loc().join(rhs.loc());

    match lhs {
        Node::Send(loc, recv, mid, mut args) => {
            args.push(rhs);
            Node::Send(asgn_loc, recv, mid, args)
        },
        Node::CSend(loc, recv, mid, mut args) => {
            args.push(rhs);
            Node::CSend(asgn_loc, recv, mid, args)
        },
        Node::Lvlhs(loc, name) =>
            Node::Lvasgn(asgn_loc, Id(loc, name), rhs),
        Node::Const(loc, scope, name) =>
            Node::Casgn(asgn_loc, scope, name, rhs),
        Node::Cvar(loc, name) =>
            Node::Cvasgn(asgn_loc, Id(loc, name), rhs),
        Node::Ivlhs(loc, name) =>
            Node::Ivasgn(asgn_loc, Id(loc, name), rhs),
        Node::Gvar(loc, name) =>
            Node::Gvasgn(asgn_loc, Id(loc, name), rhs),
        _ => {
            panic!("unimplemented lhs: {:?}", lhs);
        }
    }.to_raw()
}

unsafe extern "C" fn assignable(parser: *mut Parser, node: *mut Rc<Node>) -> *mut Rc<Node> {
    let node = Rc::try_unwrap(from_raw(node)).expect("unique ownership of AST nodes during parse");
    match node {
        Node::Ident(loc, name) => {
            Parser::declare(parser, &name);
            Node::Lvlhs(loc, name)
        },
        Node::Ivar(loc, name) => {
            Node::Ivlhs(loc, name)
        },
        lhs @ Node::Const(_, _, _) |
        lhs @ Node::Cvar(_, _) => lhs,
        lhs @ Node::Gvar(_, _) => lhs,
        lhs =>
            panic!("not assignable on lhs: {:?}", lhs),
    }.to_raw()
}

unsafe extern "C" fn associate(begin: *const Token, pairs: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let pairs = ffi::node_list_from_raw(pairs);
    Node::Hash(collection_map(begin, &pairs, end).unwrap(), pairs).to_raw()
}

unsafe extern "C" fn attr_asgn(receiver: *mut Rc<Node>, dot: *const Token, selector: *const Token) -> *mut Rc<Node> {
    let recv = from_raw(receiver);

    let selector = Id(token_loc(selector), Token::string(selector) + "=");

    let loc = recv.loc().join(&selector.0);

    // this builds an incomplete AST node:
    match call_type_for_dot(dot) {
        CallType::CSend => Node::CSend(loc, Some(recv), selector, vec![]),
        CallType::Send => Node::Send(loc, Some(recv), selector, vec![]),
    }.to_raw()
}

unsafe extern "C" fn back_ref(tok: *const Token) -> *mut Rc<Node> {
    Node::Backref(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn begin(begin: *const Token, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let body = from_maybe_raw(body);

    let loc = if begin == ptr::null_mut() {
        assert!(end == ptr::null_mut());
        match body {
            Some(ref boxed_body) => boxed_body.loc().clone(),
            None => panic!("expected body to not be None"),
        }
    } else {
        assert!(end != ptr::null_mut());
        join_tokens(begin, end)
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
    }.to_raw()
}

unsafe extern "C" fn begin_body(body: *mut Rc<Node>, rescue_bodies: *mut NodeList, else_tok: *const Token, else_: *mut Rc<Node>, ensure_tok: *const Token, ensure: *mut Rc<Node>) -> *mut Rc<Node> {
    let mut compound_stmt = from_maybe_raw(body);
    let rescue_bodies = ffi::node_list_from_raw(rescue_bodies);
    let else_ = from_maybe_raw(else_);
    let ensure = from_maybe_raw(ensure);

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
                token_loc(else_tok).join(else_body.loc()),
                vec![else_body])));

        compound_stmt = Some(Rc::new(Node::Begin(join_exprs(stmts.as_slice()), stmts)));
    }

    if let Some(ensure_box) = ensure {
        let loc = {
            let ensure_loc = ensure_box.loc();

            match compound_stmt {
                Some(ref compound_stmt_box) => compound_stmt_box.loc().join(ensure_loc),
                None => token_loc(ensure_tok).join(ensure_loc),
            }
        };

        compound_stmt = Some(Rc::new(Node::Ensure(loc, compound_stmt, ensure_box)));
    }

    compound_stmt.to_raw()
}

unsafe extern "C" fn begin_keyword(begin: *const Token, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let body = from_maybe_raw(body);
    let tokens = join_tokens(begin, end);
    match body {
        Some(node) => {
            match *node {
                Node::Begin(_, ref beg) => Node::Kwbegin(tokens, beg.clone()),
                _ => Node::Kwbegin(tokens, vec![node.clone()]),
            }
        },
        None => Node::Kwbegin(tokens, vec![])
    }.to_raw()
}

unsafe extern "C" fn binary_op(recv: *mut Rc<Node>, oper: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let recv = from_raw(recv);
    let arg = from_raw(arg);

    Node::Send(recv.loc().join(arg.loc()), Some(recv), token_id(oper), vec![arg]).to_raw()
}

unsafe extern "C" fn block(method_call: *mut Rc<Node>, begin: *const Token, args: *mut Rc<Node>, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let method_call = from_raw(method_call);
    let args = from_raw(args);
    let body = from_maybe_raw(body);

    if let Node::Yield(_, _) = *method_call {
        // diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
    }

    match *method_call {
        Node::Send(_, _, _, ref args) |
        Node::CSend(_, _, _, ref args) |
        Node::Super(_, ref args) => {
            if let Some(ref last_arg) = args.last() {
                if let Node::BlockPass(ref loc, _) = ***last_arg {
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
            Node::Block(method_call.loc().join(&token_loc(end)), method_call, args, body),
        _ => panic!("unknown method call node: {:?}", method_call),
    }.to_raw()
}

unsafe extern "C" fn block_pass(amper: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let arg = from_raw(arg);
    Node::BlockPass(token_loc(amper).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn blockarg(amper: *const Token, name: *const Token) -> *mut Rc<Node> {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Blockarg(token_loc(amper).join(&id.0), Some(id))
    } else {
        Node::Blockarg(token_loc(amper), None)
    }.to_raw()
}

unsafe extern "C" fn call_lambda(lambda: *const Token) -> *mut Rc<Node> {
    Node::Lambda(token_loc(lambda)).to_raw()
}

unsafe extern "C" fn call_method(receiver: *mut Rc<Node>, dot: *const Token, selector: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let recv = from_maybe_raw(receiver);
    let args = ffi::node_list_from_raw(args);

    let loc = {
        let selector_loc =
            if selector != ptr::null_mut() {
                token_loc(selector)
            } else {
                // if there is no selector (in the case of the foo.() #call syntax)
                // syntactically there *must* be a dot:
                token_loc(dot)
            };

        let loc_start =
            match recv {
                Some(ref node) => node.loc(),
                _ => &selector_loc,
            };

        if rparen != ptr::null_mut() {
            loc_start.join(&token_loc(rparen))
        } else if args.len() > 0 {
            loc_start.join(args.last().unwrap().loc())
        } else {
            loc_start.join(&selector_loc)
        }
    };

    let selector =
        if selector != ptr::null_mut() {
            token_id(selector)
        } else {
            Id(token_loc(dot), "call".to_owned())
        };

    match call_type_for_dot(dot) {
        CallType::CSend => Node::CSend(loc, recv, selector, args),
        CallType::Send => Node::Send(loc, recv, selector, args),
    }.to_raw()
}

unsafe extern "C" fn case_(case_: *const Token, expr: *mut Rc<Node>, when_bodies: *mut NodeList, else_tok: *const Token, else_body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let expr = from_maybe_raw(expr);
    let whens = ffi::node_list_from_raw(when_bodies);
    let else_ = from_maybe_raw(else_body);

    Node::Case(join_tokens(case_, end), expr, whens, else_).to_raw()
}

unsafe extern "C" fn character(char_: *const Token) -> *mut Rc<Node> {
    Node::String(token_loc(char_), Token::string(char_)).to_raw()
}

unsafe extern "C" fn complex(tok: *const Token) -> *mut Rc<Node> {
    Node::Complex(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn compstmt(nodes: *mut NodeList) -> *mut Rc<Node> {
    let mut nodes = ffi::node_list_from_raw(nodes);

    match nodes.len() {
        0 => None,
        1 => Some(nodes.remove(0)),
        _ => Some(Rc::new(Node::Begin(join_exprs(nodes.as_slice()), nodes))),
    }.to_raw()
}

fn check_condition(cond: Rc<Node>) -> Rc<Node> {
    match *cond {
        Node::Begin(ref loc, ref stmts) => {
            if stmts.len() == 1 {
                check_condition(stmts[0].clone())
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

        Node::Regexp(ref loc, ref parts, ref options) =>
            Rc::new(Node::MatchCurLine(loc.clone(), cond.clone())),

        _ => cond.clone(),
    }
}

unsafe extern "C" fn condition(cond_tok: *const Token, cond: *mut Rc<Node>, then: *const Token, if_true: *mut Rc<Node>, else_: *const Token, if_false: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let if_true = from_maybe_raw(if_true);
    let if_false = from_maybe_raw(if_false);

    let mut loc = token_loc(cond_tok).join(cond.loc());

    if then != ptr::null() {
        loc = loc.join(&token_loc(then));
    }

    if let Some(ref true_branch) = if_true {
        loc = loc.join(true_branch.loc());
    }

    if else_ != ptr::null() {
        loc = loc.join(&token_loc(else_));
    }

    if let Some(ref false_branch) = if_false {
        loc = loc.join(false_branch.loc());
    }

    if end != ptr::null() {
        loc = loc.join(&token_loc(end));
    }

    Node::If(loc, check_condition(cond), if_true, if_false).to_raw()
}

unsafe extern "C" fn condition_mod(if_true: *mut Rc<Node>, if_false: *mut Rc<Node>, cond: *mut Rc<Node>) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let if_true = from_maybe_raw(if_true);
    let if_false = from_maybe_raw(if_false);

    let loc = cond.loc().join(if_true.as_ref().unwrap_or_else(|| if_false.as_ref().unwrap()).loc());

    Node::If(loc, check_condition(cond), if_true, if_false).to_raw()
}

unsafe extern "C" fn const_(name: *const Token) -> *mut Rc<Node> {
    Node::Const(token_loc(name), None, token_id(name)).to_raw()
}

unsafe extern "C" fn const_fetch(scope: *mut Rc<Node>, colon: *const Token, name: *const Token) -> *mut Rc<Node> {
    let scope = from_raw(scope);

    let loc = scope.loc().join(&token_loc(name));

    Node::Const(loc, Some(scope), token_id(name)).to_raw()
}

unsafe extern "C" fn const_global(colon: *const Token, name: *const Token) -> *mut Rc<Node> {
    let loc = join_tokens(colon, name);

    Node::Const(loc, Some(Rc::new(Node::Cbase(token_loc(colon)))), token_id(name)).to_raw()
}

unsafe extern "C" fn const_op_assignable(node: *mut Rc<Node>) -> *mut Rc<Node> {
    panic!("unimplemented");
}

unsafe extern "C" fn cvar(tok: *const Token) -> *mut Rc<Node> {
    Node::Cvar(token_loc(tok), Token::string(tok)).to_raw()
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
}

fn dedent_parts(parts: &[Rc<Node>], mut dedenter: Dedenter) -> Vec<Rc<Node>> {
    parts.iter().map(|part| {
        match **part {
            Node::String(ref loc, ref val) => Rc::new(Node::String(loc.clone(), dedenter.dedent(val))),
            _ => { dedenter.interrupt(); part.clone() },
        }
    }).collect()
}

unsafe extern "C" fn dedent_string(node: *mut Rc<Node>, dedent_level: size_t) -> *mut Rc<Node> {
    let node = from_raw(node);

    if dedent_level != 0 {
        let mut dedenter = Dedenter::new(dedent_level);
        match *node {
            Node::String(ref loc, ref val) => Rc::new(Node::String(loc.clone(), dedenter.dedent(val))),
            Node::DString(ref loc, ref parts) => Rc::new(Node::DString(loc.clone(), dedent_parts(parts, dedenter))),
            Node::XString(ref loc, ref parts) => Rc::new(Node::XString(loc.clone(), dedent_parts(parts, dedenter))),
            _ => panic!("unexpected node type"),
        }
    } else {
        node
    }.to_raw()
}

unsafe extern "C" fn def_class(class_: *const Token, name: *mut Rc<Node>, lt_: *const Token, superclass: *mut Rc<Node>, body: *mut Rc<Node>, end_: *const Token) -> *mut Rc<Node> {
    Node::Class(join_tokens(class_, end_), from_raw(name), from_maybe_raw(superclass), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_method(def: *const Token, name: *const Token, args: *mut Rc<Node>, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let loc = join_tokens(def, end);

    Node::Def(loc, token_id(name), from_maybe_raw(args), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_module(module: *const Token, name: *mut Rc<Node>, body: *mut Rc<Node>, end_: *const Token) -> *mut Rc<Node> {
    Node::Module(join_tokens(module, end_), from_raw(name), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_sclass(class_: *const Token, lshft_: *const Token, expr: *mut Rc<Node>, body: *mut Rc<Node>, end_: *const Token) -> *mut Rc<Node> {
    Node::SClass(join_tokens(class_, end_), from_raw(expr), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_singleton(def: *const Token, definee: *mut Rc<Node>, dot: *const Token, name: *const Token, args: *mut Rc<Node>, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let loc = join_tokens(def, end);

    Node::Defs(loc, from_raw(definee), token_id(name), from_maybe_raw(args), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn encoding_literal(tok: *const Token) -> *mut Rc<Node> {
    Node::EncodingLiteral(token_loc(tok)).to_raw()
}

unsafe extern "C" fn false_(tok: *const Token) -> *mut Rc<Node> {
    Node::False(token_loc(tok)).to_raw()
}

unsafe extern "C" fn file_literal(tok: *const Token) -> *mut Rc<Node> {
    Node::FileLiteral(token_loc(tok)).to_raw()
}

unsafe extern "C" fn float_(tok: *const Token) -> *mut Rc<Node> {
    Node::Float(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn float_complex(tok: *const Token) -> *mut Rc<Node> {
    panic!("unimplemented");
}

unsafe extern "C" fn for_(for_: *const Token, iterator: *mut Rc<Node>, in_: *const Token, iteratee: *mut Rc<Node>, do_: *const Token, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let iterator = from_raw(iterator);
    let iteratee = from_raw(iteratee);
    let body = from_maybe_raw(body);

    Node::For(token_loc(for_).join(&token_loc(end)), iterator, iteratee, body).to_raw()
}

unsafe extern "C" fn gvar(tok: *const Token) -> *mut Rc<Node> {
    Node::Gvar(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn ident(tok: *const Token) -> *mut Rc<Node> {
    Node::Ident(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn index(receiver: *mut Rc<Node>, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Rc<Node> {
    let recv = from_raw(receiver);
    let indexes = ffi::node_list_from_raw(indexes);

    Node::Send(recv.loc().join(&token_loc(rbrack)), Some(recv), Id(join_tokens(lbrack, rbrack), "[]".to_owned()), indexes).to_raw()
}

unsafe extern "C" fn index_asgn(receiver: *mut Rc<Node>, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Rc<Node> {
    // Incomplete method call
    let recv = from_raw(receiver);
    let id = Id(join_tokens(lbrack, rbrack), "[]=".to_owned());
    let indexes = ffi::node_list_from_raw(indexes);
    Node::Send(recv.loc().clone(), Some(recv), id, indexes).to_raw()
}

unsafe extern "C" fn integer(tok: *const Token) -> *mut Rc<Node> {
    Node::Integer(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn ivar(tok: *const Token) -> *mut Rc<Node> {
    Node::Ivar(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn keyword_break(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    let mut loc = token_loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Break(loc, args).to_raw()
}

unsafe extern "C" fn keyword_defined(keyword: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let arg = from_raw(arg);
    Node::Defined(token_loc(keyword).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn keyword_next(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    let mut loc = token_loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Next(loc, args).to_raw()
}

unsafe extern "C" fn keyword_redo(keyword: *const Token) -> *mut Rc<Node> {
    Node::Redo(token_loc(keyword)).to_raw()
}

unsafe extern "C" fn keyword_retry(keyword: *const Token) -> *mut Rc<Node> {
    Node::Retry(token_loc(keyword)).to_raw()
}

unsafe extern "C" fn keyword_return(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    let mut loc = token_loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Return(loc, args).to_raw()
}

unsafe extern "C" fn keyword_super(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    let mut loc = token_loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Super(loc, args).to_raw()
}

unsafe extern "C" fn keyword_yield(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Rc<Node> {
    let args = ffi::node_list_from_raw(args);

    let mut loc = token_loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Yield(loc, args).to_raw()
}

unsafe extern "C" fn keyword_zsuper(keyword: *const Token) -> *mut Rc<Node> {
    Node::ZSuper(token_loc(keyword)).to_raw()
}

unsafe extern "C" fn kwarg(name: *const Token) -> *mut Rc<Node> {
    Node::Kwarg(token_loc(name), Token::string(name)).to_raw()
}

unsafe extern "C" fn kwoptarg(name: *const Token, value: *mut Rc<Node>) -> *mut Rc<Node> {
    let value = from_raw(value);
    let id = token_id(name);
    Node::Kwoptarg(id.0.join(value.loc()), id, value).to_raw()
}

unsafe extern "C" fn kwrestarg(dstar: *const Token, name: *const Token) -> *mut Rc<Node> {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Kwrestarg(token_loc(dstar).join(&id.0), Some(id))
    } else {
        Node::Kwrestarg(token_loc(dstar), None)
    }.to_raw()
}

unsafe extern "C" fn kwsplat(dstar: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let arg = from_raw(arg);
    Node::Kwsplat(token_loc(dstar).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn line_literal(tok: *const Token) -> *mut Rc<Node> {
    Node::LineLiteral(token_loc(tok)).to_raw()
}

unsafe extern "C" fn logical_and(lhs: *mut Rc<Node>, op: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);
    Node::And(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn logical_or(lhs: *mut Rc<Node>, op: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);
    Node::Or(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn loop_until(keyword: *const Token, cond: *mut Rc<Node>, do_: *const Token, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let body = from_maybe_raw(body);
    Node::Until(join_tokens(keyword, end), cond, body).to_raw()
}

unsafe extern "C" fn loop_until_mod(body: *mut Rc<Node>, cond: *mut Rc<Node>) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let body = from_raw(body);
    let loc = body.loc().join(cond.loc());

    match *body {
        Node::Kwbegin(_, _) => Node::UntilPost(loc, cond, body),
        _ => Node::Until(loc, cond, Some(body))
    }.to_raw()
}

unsafe extern "C" fn loop_while(keyword: *const Token, cond: *mut Rc<Node>, do_: *const Token, body: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let body = from_maybe_raw(body);
    Node::While(join_tokens(keyword, end), cond, body).to_raw()
}

unsafe extern "C" fn loop_while_mod(body: *mut Rc<Node>, cond: *mut Rc<Node>) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let body = from_raw(body);
    let loc = body.loc().join(cond.loc());

    match *body {
        Node::Kwbegin(_, _) => Node::WhilePost(loc, cond, body),
        _ => Node::While(loc, cond, Some(body))
    }.to_raw()
}

unsafe extern "C" fn match_op(receiver: *mut Rc<Node>, oper: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let recv = from_raw(receiver);
    let arg = from_raw(arg);

    if let Node::Regexp(_, ref parts, _) = *recv {
        // TODO if parts are all static string literals, declare any named
        // captures as local variables and emit MatchWithLvasgn node
    }

    Node::Send(recv.loc().join(arg.loc()), Some(recv), token_id(oper), vec![arg]).to_raw()
}

unsafe extern "C" fn multi_assign(mlhs: *mut Rc<Node>, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let mlhs = from_raw(mlhs);
    let rhs = from_raw(rhs);

    Node::Masgn(mlhs.loc().join(rhs.loc()), mlhs, rhs).to_raw()
}

unsafe extern "C" fn multi_lhs(begin: *const Token, items: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let items = ffi::node_list_from_raw(items);

    Node::Mlhs(collection_map(begin, items.as_slice(), end).unwrap(), items).to_raw()
}

unsafe extern "C" fn negate(uminus: *const Token, numeric: *mut Rc<Node>) -> *mut Rc<Node> {
    let numeric = from_raw(numeric);
    let loc = token_loc(uminus).join(numeric.loc());

    match *numeric {
        Node::Integer(_, ref value) => Rc::new(Node::Integer(loc, "-".to_owned() + value.as_str())),
        Node::Float(_, ref value) => Rc::new(Node::Float(loc, "-".to_owned() + value.as_str())),
        _ => panic!("unimplemented numeric type: {:?}", numeric),
    }.to_raw()
}

unsafe extern "C" fn nil(tok: *const Token) -> *mut Rc<Node> {
    Node::Nil(token_loc(tok)).to_raw()
}

unsafe extern "C" fn not_op(not: *const Token, begin: *const Token, receiver: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let not_loc = token_loc(not);
    let id = Id(token_loc(not), "!".to_owned());

    match from_maybe_raw(receiver) {
        Some(expr) => {
            let loc = if end != ptr::null() {
                not_loc.join(&token_loc(end))
            } else {
                not_loc.join(expr.loc())
            };
            Node::Send(loc, Some(expr), id, vec![])
        },
        None => {
            assert!(begin != ptr::null() && end != ptr::null());
            let nil_loc = join_tokens(begin, end);
            let loc = not_loc.join(&nil_loc);
            let recv = Rc::new(Node::Begin(nil_loc.clone(), vec![Rc::new(Node::Nil(nil_loc))]));
            Node::Send(loc, Some(recv), id, vec![])
        }
    }.to_raw()
}

unsafe extern "C" fn nth_ref(tok: *const Token) -> *mut Rc<Node> {
    Node::NthRef(token_loc(tok), Token::string(tok).parse().unwrap()).to_raw()
}

unsafe extern "C" fn op_assign(lhs: *mut Rc<Node>, op: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);

    // match lhs {
    //  TODO error on back ref and nth ref
    // }

    match Token::string(op).as_str() {
        "&&" => Node::AndAsgn(lhs.loc().join(rhs.loc()), lhs, rhs),
        "||" => Node::OrAsgn(lhs.loc().join(rhs.loc()), lhs, rhs),
        _    => Node::OpAsgn(lhs.loc().join(rhs.loc()), lhs, token_id(op), rhs),
    }.to_raw()
}

unsafe extern "C" fn optarg(name: *const Token, eql: *const Token, value: *mut Rc<Node>) -> *mut Rc<Node> {
    let id = token_id(name);
    let value = from_raw(value);
    Node::Optarg(id.0.join(value.loc()), id, value).to_raw()
}

unsafe extern "C" fn pair(key: *mut Rc<Node>, assoc: *const Token, value: *mut Rc<Node>) -> *mut Rc<Node> {
    let key = from_raw(key);
    let value = from_raw(value);
    Node::Pair(key.loc().join(value.loc()), key, value).to_raw()
}

unsafe extern "C" fn pair_keyword(key: *const Token, value: *mut Rc<Node>) -> *mut Rc<Node> {
    let sym = Node::Symbol(token_loc(key), Token::string(key));
    let value = from_raw(value);
    Node::Pair(sym.loc().join(value.loc()), Rc::new(sym), value).to_raw()
}

unsafe extern "C" fn pair_quoted(begin: *const Token, parts: *mut NodeList, end: *const Token, value: *mut Rc<Node>) -> *mut Rc<Node> {
    let key = from_raw(symbol_compose(begin, parts, end));
    let value = from_raw(value);
    Node::Pair(key.loc().join(value.loc()), key, value).to_raw()
}

unsafe extern "C" fn postexe(begin: *const Token, node: *mut Rc<Node>, rbrace: *const Token) -> *mut Rc<Node> {
    let node = from_maybe_raw(node);
    Node::Postexe(token_loc(begin).join(&token_loc(rbrace)), node).to_raw()
}

unsafe extern "C" fn preexe(begin: *const Token, node: *mut Rc<Node>, rbrace: *const Token) -> *mut Rc<Node> {
    let node = from_maybe_raw(node);
    Node::Preexe(token_loc(begin).join(&token_loc(rbrace)), node).to_raw()
}

unsafe extern "C" fn procarg0(arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let arg = from_raw(arg);
    Node::Procarg0(arg.loc().clone(), arg).to_raw()
}

unsafe extern "C" fn prototype(genargs: *mut Rc<Node>, args: *mut Rc<Node>, return_type: *mut Rc<Node>) -> *mut Rc<Node> {
    let genargs = from_maybe_raw(genargs);
    let args = from_raw(args);
    let return_type = from_maybe_raw(return_type);

    let mut loc = args.loc().clone();

    if let Some(ref genargs_) = genargs {
        loc = loc.join(genargs_.loc());
    }

    if let Some(ref return_type_) = return_type {
        loc = loc.join(return_type_.loc());
    }

    Node::Prototype(loc, genargs, args, return_type).to_raw()
}

unsafe extern "C" fn range_exclusive(lhs: *mut Rc<Node>, oper: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);

    Node::ERange(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn range_inclusive(lhs: *mut Rc<Node>, oper: *const Token, rhs: *mut Rc<Node>) -> *mut Rc<Node> {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);

    Node::IRange(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn rational(tok: *const Token) -> *mut Rc<Node> {
    Node::Rational(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn rational_complex(tok: *const Token) -> *mut Rc<Node> {
    panic!("unimplemented");
}

unsafe extern "C" fn regexp_compose(begin: *const Token, parts: *mut NodeList, end: *const Token, options: *mut Rc<Node>) -> *mut Rc<Node> {
    let parts = ffi::node_list_from_raw(parts);
    let opts = from_maybe_raw(options);
    let begin_loc = token_loc(begin);
    let loc = match opts {
        Some(ref opt_box) => begin_loc.join(opt_box.loc()),
        None => begin_loc.join(&token_loc(end)),
    };
    Node::Regexp(loc, parts, opts).to_raw()
}

unsafe extern "C" fn regexp_options(regopt: *const Token) -> *mut Rc<Node> {
    let mut options: Vec<char> = Token::string(regopt).chars().collect();
    options.sort();
    options.dedup();
    Node::Regopt(token_loc(regopt), options).to_raw()
}

unsafe extern "C" fn rescue_body(rescue: *const Token, exc_list: *mut Rc<Node>, assoc: *const Token, exc_var: *mut Rc<Node>, then: *const Token, body: *mut Rc<Node>) -> *mut Rc<Node> {
    let exc_list = from_maybe_raw(exc_list);
    let exc_var = from_maybe_raw(exc_var);
    let body = from_maybe_raw(body);

    let mut loc = token_loc(rescue);

    if let Some(ref boxed_exc_list) = exc_list {
        loc = loc.join(boxed_exc_list.loc());
    }

    if let Some(ref boxed_exc_var) = exc_var {
        loc = loc.join(boxed_exc_var.loc());
    }

    if let Some(ref boxed_body) = body {
        loc = loc.join(boxed_body.loc());
    }

    Node::Resbody(loc, exc_list, exc_var, body).to_raw()
}

unsafe extern "C" fn restarg(star: *const Token, name: *const Token) -> *mut Rc<Node> {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Restarg(token_loc(star).join(&id.0), Some(id))
    } else {
        Node::Restarg(token_loc(star), None)
    }.to_raw()
}

unsafe extern "C" fn self_(tok: *const Token) -> *mut Rc<Node> {
    Node::Self_(token_loc(tok)).to_raw()
}

unsafe extern "C" fn shadowarg(name: *const Token) -> *mut Rc<Node> {
    panic!("unimplemented");
}

unsafe extern "C" fn splat(star: *const Token, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let arg = from_maybe_raw(arg);
    let loc = match arg {
        Some(ref box_arg) => token_loc(star).join(box_arg.loc()),
        None => token_loc(star),
    };
    Node::Splat(loc, arg).to_raw()
}

unsafe extern "C" fn string(string_: *const Token) -> *mut Rc<Node> {
    Node::String(token_loc(string_), Token::string(string_)).to_raw()
}

unsafe extern "C" fn string_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let parts = ffi::node_list_from_raw(parts);

    let loc = collection_map(begin, parts.as_slice(), end).unwrap();

    if parts.len() == 1 {
        match *parts[0] {
            Node::String(ref loc, ref val) =>
                Node::String(loc.clone(), val.clone()),
            Node::DString(ref loc, ref val) =>
                Node::DString(loc.clone(), val.clone()),
            _ => Node::DString(loc.clone(), vec![parts[0].clone()]),
        }
    } else {
        Node::DString(loc, parts)
    }.to_raw()
}

unsafe extern "C" fn string_internal(string: *const Token) -> *mut Rc<Node> {
    Node::String(token_loc(string), Token::string(string)).to_raw()
}

unsafe extern "C" fn symbol(symbol: *const Token) -> *mut Rc<Node> {
    Node::Symbol(token_loc(symbol), Token::string(symbol)).to_raw()
}

unsafe extern "C" fn symbol_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let parts = ffi::node_list_from_raw(parts);

    let loc = collection_map(begin, parts.as_slice(), end).unwrap();

    if parts.len() == 1 {
        match *parts[0] {
            Node::Symbol(ref loc, ref val) =>
                Node::Symbol(loc.clone(), val.clone()),

            _ => Node::DSymbol(loc, vec![parts[0].clone()]),
        }
    } else {
        Node::DSymbol(loc, parts)
    }.to_raw()
}

unsafe extern "C" fn symbol_internal(symbol: *const Token) -> *mut Rc<Node> {
    Node::Symbol(token_loc(symbol), Token::string(symbol)).to_raw()
}

unsafe extern "C" fn symbols_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let parts = ffi::node_list_from_raw(parts);

    let parts = parts.iter().map(|part| {
        match **part {
            Node::String(ref loc, ref val) =>
                Rc::new(Node::Symbol(loc.clone(), val.clone())),

            Node::DString(ref loc, ref parts) =>
                Rc::new(Node::DSymbol(loc.clone(), parts.clone())),

            _ => part.clone(),
        }
    }).collect::<Vec<_>>();

    Node::Array(collection_map(begin, parts.as_slice(), end).unwrap(), parts).to_raw()
}

unsafe extern "C" fn ternary(cond: *mut Rc<Node>, question: *const Token, if_true: *mut Rc<Node>, colon: *const Token, if_false: *mut Rc<Node>) -> *mut Rc<Node> {
    let cond = from_raw(cond);
    let if_true = from_raw(if_true);
    let if_false = from_raw(if_false);

    Node::If(cond.loc().join(if_false.loc()), check_condition(cond), Some(if_true), Some(if_false)).to_raw()
}

unsafe extern "C" fn tr_any(special: *const Token) -> *mut Rc<Node> {
    Node::TyAny(token_loc(special)).to_raw()
}

unsafe extern "C" fn tr_array(begin: *const Token, type_: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let type_ = from_raw(type_);

    Node::TyArray(join_tokens(begin, end), type_).to_raw()
}

unsafe extern "C" fn tr_cast(begin: *const Token, expr: *mut Rc<Node>, colon: *const Token, type_: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let expr = from_raw(expr);
    let type_ = from_raw(type_);

    Node::TyCast(join_tokens(begin, end), expr, type_).to_raw()
}

unsafe extern "C" fn tr_class(special: *const Token) -> *mut Rc<Node> {
    Node::TyClass(token_loc(special)).to_raw()
}

unsafe extern "C" fn tr_cpath(cpath: *mut Rc<Node>) -> *mut Rc<Node> {
    let cpath = from_raw(cpath);

    Node::TyCpath(cpath.loc().clone(), cpath).to_raw()
}

unsafe extern "C" fn tr_genargs(begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let genargs = ffi::node_list_from_raw(genargs);

    Node::TyGenargs(join_tokens(begin, end), genargs).to_raw()
}

unsafe extern "C" fn tr_gendecl(cpath: *mut Rc<Node>, begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let cpath = from_raw(cpath);
    let genargs = ffi::node_list_from_raw(genargs);

    Node::TyGendecl(cpath.loc().join(&token_loc(end)), cpath, genargs).to_raw()
}

unsafe extern "C" fn tr_gendeclarg(tok: *const Token) -> *mut Rc<Node> {
    Node::TyGendeclarg(token_loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn tr_geninst(cpath: *mut Rc<Node>, begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let cpath = from_raw(cpath);
    let genargs = ffi::node_list_from_raw(genargs);

    Node::TyGeninst(cpath.loc().join(&token_loc(end)), cpath, genargs).to_raw()
}

unsafe extern "C" fn tr_hash(begin: *const Token, key_type: *mut Rc<Node>, assoc: *const Token, value_type: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let key_type = from_raw(key_type);
    let value_type = from_raw(value_type);

    Node::TyHash(join_tokens(begin, end), key_type, value_type).to_raw()
}

unsafe extern "C" fn tr_instance(special: *const Token) -> *mut Rc<Node> {
    Node::TyInstance(token_loc(special)).to_raw()
}

unsafe extern "C" fn tr_ivardecl(name: *const Token, type_: *mut Rc<Node>) -> *mut Rc<Node> {
    let name = token_id(name);
    let type_ = from_raw(type_);

    Node::TyIvardecl(name.0.join(type_.loc()), name, type_).to_raw()
}

unsafe extern "C" fn tr_nil(nil: *const Token) -> *mut Rc<Node> {
    Node::TyNil(token_loc(nil)).to_raw()
}

unsafe extern "C" fn tr_nillable(tilde: *const Token, type_: *mut Rc<Node>) -> *mut Rc<Node> {
    let type_ = from_raw(type_);

    Node::TyNillable(token_loc(tilde).join(type_.loc()), type_).to_raw()
}

unsafe extern "C" fn tr_or(a: *mut Rc<Node>, b: *mut Rc<Node>) -> *mut Rc<Node> {
    let a = from_raw(a);
    let b = from_raw(b);

    Node::TyOr(a.loc().join(b.loc()), a, b).to_raw()
}

unsafe extern "C" fn tr_proc(begin: *const Token, args: *mut Rc<Node>, end: *const Token) -> *mut Rc<Node> {
    let args = from_raw(args);

    Node::TyProc(join_tokens(begin, end), args).to_raw()
}

unsafe extern "C" fn tr_self(special: *const Token) -> *mut Rc<Node> {
    Node::TySelf(token_loc(special)).to_raw()
}

unsafe extern "C" fn tr_tuple(begin: *const Token, types: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let types = ffi::node_list_from_raw(types);

    Node::TyTuple(join_tokens(begin, end), types).to_raw()
}

unsafe extern "C" fn true_(tok: *const Token) -> *mut Rc<Node> {
    Node::True(token_loc(tok)).to_raw()
}

unsafe extern "C" fn typed_arg(type_: *mut Rc<Node>, arg: *mut Rc<Node>) -> *mut Rc<Node> {
    let type_ = from_raw(type_);
    let arg = from_raw(arg);

    Node::TypedArg(type_.loc().join(arg.loc()), type_, arg).to_raw()
}

unsafe extern "C" fn unary_op(oper: *const Token, receiver: *mut Rc<Node>) -> *mut Rc<Node> {
    let id = token_id(oper);
    let recv = from_raw(receiver);

    let id = match id.1.as_str() {
        "+" => Id(id.0, "+@".to_owned()),
        "-" => Id(id.0, "-@".to_owned()),
        _   => id,
    };

    Node::Send(id.0.join(recv.loc()), Some(recv), id, vec![]).to_raw()
}

unsafe extern "C" fn undef_method(undef: *const Token, name_list: *mut NodeList) -> *mut Rc<Node> {
    let name_list = ffi::node_list_from_raw(name_list);

    let loc = match name_list.last() {
        Some(ref node) => token_loc(undef).join(node.loc()),
        None => token_loc(undef),
    };

    Node::Undef(loc, name_list).to_raw()
}

unsafe extern "C" fn when(when: *const Token, patterns: *mut NodeList, then: *const Token, body: *mut Rc<Node>) -> *mut Rc<Node> {
    let patterns = ffi::node_list_from_raw(patterns);
    let body = from_maybe_raw(body);

    let when_loc = token_loc(when);

    let loc = if let Some(ref body_box) = body {
        when_loc.join(body_box.loc())
    } else if then != ptr::null() {
        when_loc.join(&token_loc(then))
    } else {
        when_loc.join(patterns.last().unwrap().loc())
    };

    Node::When(loc, patterns, body).to_raw()
}

unsafe extern "C" fn word(parts: *mut NodeList) -> *mut Rc<Node> {
    let mut parts = ffi::node_list_from_raw(parts);

    if parts.len() == 1 {
        parts.remove(0)
    } else {
        assert!(!parts.is_empty());
        let loc = parts.first().unwrap().loc().join(parts.last().unwrap().loc());
        Rc::new(Node::DString(loc, parts))
    }.to_raw()
}

unsafe extern "C" fn words_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let words = ffi::node_list_from_raw(parts);
    Node::Array(collection_map(begin, words.as_slice(), end).unwrap(), words).to_raw()
}

unsafe extern "C" fn xstring_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Rc<Node> {
    let parts = ffi::node_list_from_raw(parts);
    Node::XString(collection_map(begin, parts.as_slice(), end).unwrap(), parts).to_raw()
}

static BUILDER: Builder = Builder {
    accessible: accessible,
    alias: alias,
    arg: arg,
    args: args,
    array: array,
    assign: assign,
    assignable: assignable,
    associate: associate,
    attr_asgn: attr_asgn,
    back_ref: back_ref,
    begin: begin,
    begin_body: begin_body,
    begin_keyword: begin_keyword,
    binary_op: binary_op,
    block: block,
    block_pass: block_pass,
    blockarg: blockarg,
    call_lambda: call_lambda,
    call_method: call_method,
    case_: case_,
    character: character,
    complex: complex,
    compstmt: compstmt,
    condition: condition,
    condition_mod: condition_mod,
    const_: const_,
    const_fetch: const_fetch,
    const_global: const_global,
    const_op_assignable: const_op_assignable,
    cvar: cvar,
    dedent_string: dedent_string,
    def_class: def_class,
    def_method: def_method,
    def_module: def_module,
    def_sclass: def_sclass,
    def_singleton: def_singleton,
    encoding_literal: encoding_literal,
    false_: false_,
    file_literal: file_literal,
    float_: float_,
    float_complex: float_complex,
    for_: for_,
    gvar: gvar,
    ident: ident,
    index: index,
    index_asgn: index_asgn,
    integer: integer,
    ivar: ivar,
    keyword_break: keyword_break,
    keyword_defined: keyword_defined,
    keyword_next: keyword_next,
    keyword_redo: keyword_redo,
    keyword_retry: keyword_retry,
    keyword_return: keyword_return,
    keyword_super: keyword_super,
    keyword_yield: keyword_yield,
    keyword_zsuper: keyword_zsuper,
    kwarg: kwarg,
    kwoptarg: kwoptarg,
    kwrestarg: kwrestarg,
    kwsplat: kwsplat,
    line_literal: line_literal,
    logical_and: logical_and,
    logical_or: logical_or,
    loop_until: loop_until,
    loop_until_mod: loop_until_mod,
    loop_while: loop_while,
    loop_while_mod: loop_while_mod,
    match_op: match_op,
    multi_assign: multi_assign,
    multi_lhs: multi_lhs,
    negate: negate,
    nil: nil,
    not_op: not_op,
    nth_ref: nth_ref,
    op_assign: op_assign,
    optarg: optarg,
    pair: pair,
    pair_keyword: pair_keyword,
    pair_quoted: pair_quoted,
    postexe: postexe,
    preexe: preexe,
    procarg0: procarg0,
    prototype: prototype,
    range_exclusive: range_exclusive,
    range_inclusive: range_inclusive,
    rational: rational,
    rational_complex: rational_complex,
    regexp_compose: regexp_compose,
    regexp_options: regexp_options,
    rescue_body: rescue_body,
    restarg: restarg,
    self_: self_,
    shadowarg: shadowarg,
    splat: splat,
    string: string,
    string_compose: string_compose,
    string_internal: string_internal,
    symbol: symbol,
    symbol_compose: symbol_compose,
    symbol_internal: symbol_internal,
    symbols_compose: symbols_compose,
    ternary: ternary,
    tr_any: tr_any,
    tr_array: tr_array,
    tr_cast: tr_cast,
    tr_class: tr_class,
    tr_cpath: tr_cpath,
    tr_genargs: tr_genargs,
    tr_gendecl: tr_gendecl,
    tr_gendeclarg: tr_gendeclarg,
    tr_geninst: tr_geninst,
    tr_hash: tr_hash,
    tr_instance: tr_instance,
    tr_ivardecl: tr_ivardecl,
    tr_nil: tr_nil,
    tr_nillable: tr_nillable,
    tr_or: tr_or,
    tr_proc: tr_proc,
    tr_self: tr_self,
    tr_tuple: tr_tuple,
    true_: true_,
    typed_arg: typed_arg,
    unary_op: unary_op,
    undef_method: undef_method,
    when: when,
    word: word,
    words_compose: words_compose,
    xstring_compose: xstring_compose,
};

pub fn parse(source_file: Rc<SourceFile>) -> Ast {
    SOURCE_FILE.with(|sf|
        *sf.borrow_mut() = Some(source_file.clone())
    );

    let parser = unsafe { Parser::new(source_file.source(), &BUILDER) };
    let ast = unsafe { Parser::parse(parser) };
    let diagnostics = unsafe { Parser::diagnostics(parser) };
    unsafe { Parser::free(parser) };

    SOURCE_FILE.with(|sf|
        *sf.borrow_mut() = None
    );

    Ast {
        node: ast.map(|node| *node),
        diagnostics: diagnostics.into_iter().map(|(level, message, begin, end)|
            Diagnostic {
                level: level,
                message: message,
                loc: Loc {
                    file: source_file.clone(),
                    begin_pos: begin,
                    end_pos: end,
                },
            }
        ).collect(),
    }
}
