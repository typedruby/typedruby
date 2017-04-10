#![allow(unused_variables)]

extern crate libc;

use std::ptr;

use ast::*;
use ffi;
use ffi::{Builder, NodeList, Token, Parser};
use self::libc::size_t;
use std::collections::HashSet;

trait ToRaw {
    fn to_raw(self) -> *mut Node;
}

impl ToRaw for Option<Box<Node>> {
    fn to_raw(self) -> *mut Node {
        match self {
            None => ptr::null_mut(),
            Some(x) => x.to_raw(),
        }
    }
}

impl ToRaw for Box<Node> {
    fn to_raw(self) -> *mut Node {
        Box::into_raw(self)
    }
}

impl ToRaw for Option<Node> {
    fn to_raw(self) -> *mut Node {
        match self {
            None => ptr::null_mut(),
            Some(x) => Box::new(x).to_raw(),
        }
    }
}

impl ToRaw for Node {
    fn to_raw(self) -> *mut Node {
        Box::into_raw(Box::new(self))
    }
}

unsafe fn from_maybe_raw(p: *mut Node) -> Option<Box<Node>> {
    if p == ptr::null_mut() {
        None
    } else {
        Some(Box::from_raw(p))
    }
}

fn join_exprs(exprs: &[Box<Node>]) -> Loc {
    assert!(!exprs.is_empty());

    let a = exprs.first().unwrap();
    let b = exprs.last().unwrap();

    a.loc().join(b.loc())
}

unsafe fn join_tokens(left: *const Token, right: *const Token) -> Loc {
    Token::loc(left).join(&Token::loc(right))
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
    Id(Token::loc(tok), Token::string(tok))
}

unsafe fn from_raw(p: *mut Node) -> Box<Node> {
    if p == ptr::null_mut() {
        panic!("received null node pointer in from_raw!");
    }

    Box::from_raw(p)
}

unsafe extern "C" fn accessible(parser: *mut Parser, node: *mut Node) -> *mut Node {
    match *from_raw(node) {
        Node::Ident(loc, name) => {
            if Parser::is_declared(parser, &name) {
                Node::Lvar(loc, name)
            } else {
                Node::Send(loc.clone(), None, Id(loc, name), vec![])
            }
        },
        boxed_node => boxed_node,
    }.to_raw()
}

unsafe extern "C" fn alias(alias: *const Token, to: *mut Node, from: *mut Node) -> *mut Node {
    let to = from_raw(to);
    let from = from_raw(from);

    Node::Alias(Token::loc(alias).join(from.loc()), to, from).to_raw()
}

unsafe extern "C" fn arg(name: *const Token) -> *mut Node {
    Node::Arg(Token::loc(name), Token::string(name)).to_raw()
}

fn check_duplicate_args_inner<'a>(names: &mut HashSet<&'a str>, arg: &'a Node) {
    if let Node::Procarg0(_, ref arg) = *arg {
        check_duplicate_args_inner(names, arg);
        return;
    }

    if let Node::Mlhs(_, ref mlhs_items) = *arg {
        for mlhs_item in mlhs_items {
            check_duplicate_args_inner(names, mlhs_item);
        }
        return;
    }

    let (range, name) = match arg {
        &Node::Arg(ref loc, ref name) => (loc, name),
        &Node::Blockarg(_, None) => return,
        &Node::Blockarg(_, Some(Id(ref loc, ref name))) => (loc, name),
        &Node::Kwarg(ref loc, ref name) => (loc, name),
        &Node::Kwoptarg(_, Id(ref loc, ref name), _) => (loc, name),
        &Node::Kwrestarg(_, None) => return,
        &Node::Kwrestarg(_, Some(Id(ref loc, ref name))) => (loc, name),
        &Node::Optarg(_, Id(ref loc, ref name), _) => (loc, name),
        &Node::Restarg(_, None) => return,
        &Node::Restarg(_, Some(Id(ref loc, ref name))) => (loc, name),
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

fn check_duplicate_args<'a>(args: &[Box<Node>]) {
    for arg in args {
        check_duplicate_args_inner(&mut HashSet::new(), arg);
    }
}

unsafe fn collection_map(begin: *const Token, elements: &[Box<Node>], end: *const Token) -> Option<Loc> {
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

unsafe extern "C" fn args(begin: *const Token, args: *mut NodeList, end: *const Token, check_args: bool) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    if check_args {
        check_duplicate_args(args.as_slice());
    }

    let loc = collection_map(begin, args.as_slice(), end).unwrap_or(
        // FIXME - we don't have any location information to work with here:
        Loc { begin_pos: 0, end_pos: 0 }
    );

    Node::Args(loc, args).to_raw()
}

unsafe extern "C" fn array(begin: *const Token, elements: *mut NodeList, end: *const Token) -> *mut Node {
    let elements = ffi::node_list_from_raw(elements);
    Node::Array(collection_map(begin, elements.as_slice(), end).unwrap(), elements).to_raw()
}

unsafe extern "C" fn assign(lhs: *mut Node, eql: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = *from_raw(lhs);
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
        Node::Lvassignable(loc, name) =>
            Node::Lvasgn(asgn_loc, Id(loc, name), rhs),
        Node::Const(loc, scope, name) =>
            Node::Casgn(asgn_loc, scope, name, rhs),
        Node::Cvar(loc, name) =>
            Node::Cvasgn(asgn_loc, Id(loc, name), rhs),
        Node::Ivar(loc, name) =>
            Node::Ivasgn(asgn_loc, Id(loc, name), rhs),
        Node::Gvar(loc, name) =>
            Node::Gvasgn(asgn_loc, Id(loc, name), rhs),
        _ => {
            panic!("unimplemented lhs: {:?}", lhs);
        }
    }.to_raw()
}

unsafe extern "C" fn assignable(parser: *mut Parser, node: *mut Node) -> *mut Node {
    match *from_raw(node) {
        Node::Ident(loc, name) => {
            Parser::declare(parser, &name);
            Node::Lvassignable(loc, name)
        },
        lhs @ Node::Const(_, _, _) |
        lhs @ Node::Ivar(_, _) |
        lhs @ Node::Cvar(_, _) => lhs,
        lhs @ Node::Gvar(_, _) => lhs,
        lhs =>
            panic!("not assignable on lhs: {:?}", lhs),
    }.to_raw()
}

unsafe extern "C" fn associate(begin: *const Token, pairs: *mut NodeList, end: *const Token) -> *mut Node {
    let pairs = ffi::node_list_from_raw(pairs);
    Node::Hash(collection_map(begin, &pairs, end).unwrap(), pairs).to_raw()
}

unsafe extern "C" fn attr_asgn(receiver: *mut Node, dot: *const Token, selector: *const Token) -> *mut Node {
    let recv = from_raw(receiver);

    let selector = Id(Token::loc(selector), Token::string(selector) + "=");

    let loc = recv.loc().join(&selector.0);

    // this builds an incomplete AST node:
    match call_type_for_dot(dot) {
        CallType::CSend => Node::CSend(loc, Some(recv), selector, vec![]),
        CallType::Send => Node::Send(loc, Some(recv), selector, vec![]),
    }.to_raw()
}

unsafe extern "C" fn back_ref(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn begin(begin: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
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

    // TODO not exactly the logic from parser gem's begin
    // revisit when Node::Mlhs exists
    Node::Begin(loc, match body {
        // A nil expression: `()'.
        None => vec![],

        Some(boxed_body) => vec![boxed_body],
    }).to_raw()
}

unsafe extern "C" fn begin_body(body: *mut Node, rescue_bodies: *mut NodeList, else_tok: *const Token, else_: *mut Node, ensure_tok: *const Token, ensure: *mut Node) -> *mut Node {
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
                compound_stmt = Some(Box::new(Node::Rescue(loc, compound_stmt, rescue_bodies, Some(else_body))));
            },
            None => {
                let loc = {
                    let loc = rescue_bodies.last().unwrap().loc();
                    match compound_stmt {
                        Some(ref body) => body.loc().join(loc),
                        None => loc.clone(),
                    }
                };
                compound_stmt = Some(Box::new(Node::Rescue(loc, compound_stmt, rescue_bodies, None)));
            }
        }
    } else if let Some(else_body) = else_ {
        let mut stmts = match compound_stmt {
            Some(node) => match *node {
                Node::Begin(_, begin_stmts) => begin_stmts,
                _ => vec![node],
            },
            _ => vec![],
        };

        stmts.push(Box::new(
            Node::Begin(
                Token::loc(else_tok).join(else_body.loc()),
                vec![else_body])));

        compound_stmt = Some(Box::new(Node::Begin(join_exprs(stmts.as_slice()), stmts)));
    }

    if let Some(ensure_box) = ensure {
        let loc = {
            let ensure_loc = ensure_box.loc();

            match compound_stmt {
                Some(ref compound_stmt_box) => compound_stmt_box.loc().join(ensure_loc),
                None => Token::loc(ensure_tok).join(ensure_loc),
            }
        };

        compound_stmt = Some(Box::new(Node::Ensure(loc, compound_stmt, ensure_box)));
    }

    compound_stmt.to_raw()
}

unsafe extern "C" fn begin_keyword(begin: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    let body = from_raw(body);
    Node::KwBegin(join_tokens(begin, end), body).to_raw()
}

unsafe extern "C" fn binary_op(recv: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node {
    let recv = from_raw(recv);
    let arg = from_raw(arg);

    Node::Send(recv.loc().join(arg.loc()), Some(recv), token_id(oper), vec![arg]).to_raw()
}

unsafe extern "C" fn block(method_call: *mut Node, begin: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
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
            Node::Block(method_call.loc().join(&Token::loc(end)), method_call, args, body),
        _ => panic!("unknown method call node: {:?}", method_call),
    }.to_raw()
}

unsafe extern "C" fn block_pass(amper: *const Token, arg: *mut Node) -> *mut Node {
    let arg = from_raw(arg);
    Node::BlockPass(Token::loc(amper).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn blockarg(amper: *const Token, name: *const Token) -> *mut Node {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Blockarg(Token::loc(amper).join(&id.0), Some(id))
    } else {
        Node::Blockarg(Token::loc(amper), None)
    }.to_raw()
}

unsafe extern "C" fn call_lambda(lambda: *const Token) -> *mut Node {
    Node::Lambda(Token::loc(lambda)).to_raw()
}

unsafe extern "C" fn call_method(receiver: *mut Node, dot: *const Token, selector: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let recv = from_maybe_raw(receiver);
    let args = ffi::node_list_from_raw(args);

    let loc = {
        let selector_loc =
            if selector != ptr::null_mut() {
                Token::loc(selector)
            } else {
                // if there is no selector (in the case of the foo.() #call syntax)
                // syntactically there *must* be a dot:
                Token::loc(dot)
            };

        let loc_start =
            match recv {
                Some(ref node) => node.loc(),
                _ => &selector_loc,
            };

        if rparen != ptr::null_mut() {
            loc_start.join(&Token::loc(rparen))
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
            Id(Token::loc(dot), "call".to_owned())
        };

    match call_type_for_dot(dot) {
        CallType::CSend => Node::CSend(loc, recv, selector, args),
        CallType::Send => Node::Send(loc, recv, selector, args),
    }.to_raw()
}

unsafe extern "C" fn case_(case_: *const Token, expr: *mut Node, when_bodies: *mut NodeList, else_tok: *const Token, else_body: *mut Node, end: *const Token) -> *mut Node {
    let expr = from_maybe_raw(expr);
    let whens = ffi::node_list_from_raw(when_bodies);
    let else_ = from_maybe_raw(else_body);

    Node::Case(join_tokens(case_, end), expr, whens, else_).to_raw()
}

unsafe extern "C" fn character(char_: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn complex(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn compstmt(nodes: *mut NodeList) -> *mut Node {
    let mut nodes = ffi::node_list_from_raw(nodes);

    match nodes.len() {
        0 => None,
        1 => Some(nodes.remove(0)),
        _ => Some(Box::new(Node::Begin(join_exprs(nodes.as_slice()), nodes))),
    }.to_raw()
}

fn check_condition(cond: Node) -> Node {
    match cond {
        Node::Begin(loc, mut stmts) => {
            if stmts.len() == 1 {
                check_condition(*stmts.remove(0))
            } else {
                Node::Begin(loc, stmts)
            }
        },
        Node::And(loc, a, b) => Node::And(loc, Box::new(check_condition(*a)), Box::new(check_condition(*b))),
        Node::Or(loc, a, b) => Node::Or(loc, Box::new(check_condition(*a)), Box::new(check_condition(*b))),
        Node::IRange(loc, a, b) => Node::IFlipflop(loc, Box::new(check_condition(*a)), Box::new(check_condition(*b))),
        Node::ERange(loc, a, b) => Node::EFlipflop(loc, Box::new(check_condition(*a)), Box::new(check_condition(*b))),
        Node::Regexp(loc, parts, options) => Node::MatchCurLine(loc.clone(), Box::new(Node::Regexp(loc, parts, options))),
        other => other,
    }
}

unsafe extern "C" fn condition(cond_tok: *const Token, cond: *mut Node, then: *const Token, if_true: *mut Node, else_: *const Token, if_false: *mut Node, end: *const Token) -> *mut Node {
    let cond = from_raw(cond);
    let if_true = from_maybe_raw(if_true);
    let if_false = from_maybe_raw(if_false);

    let mut loc = Token::loc(cond_tok).join(cond.loc());

    if then != ptr::null() {
        loc = loc.join(&Token::loc(then));
    }

    if let Some(ref true_branch) = if_true {
        loc = loc.join(true_branch.loc());
    }

    if else_ != ptr::null() {
        loc = loc.join(&Token::loc(else_));
    }

    if let Some(ref false_branch) = if_false {
        loc = loc.join(false_branch.loc());
    }

    if end != ptr::null() {
        loc = loc.join(&Token::loc(end));
    }

    Node::If(loc, Box::new(check_condition(*cond)), if_true, if_false).to_raw()
}

unsafe extern "C" fn condition_mod(if_true: *mut Node, if_false: *mut Node, cond: *mut Node) -> *mut Node {
    let cond = from_raw(cond);
    let if_true = from_maybe_raw(if_true);
    let if_false = from_maybe_raw(if_false);

    let loc = cond.loc().join(if_true.as_ref().unwrap_or_else(|| if_false.as_ref().unwrap()).loc());

    Node::If(loc, Box::new(check_condition(*cond)), if_true, if_false).to_raw()
}

unsafe extern "C" fn const_(name: *const Token) -> *mut Node {
    Node::Const(Token::loc(name), None, token_id(name)).to_raw()
}

unsafe extern "C" fn const_fetch(scope: *mut Node, colon: *const Token, name: *const Token) -> *mut Node {
    let scope = from_raw(scope);

    let loc = scope.loc().join(&Token::loc(name));

    Node::Const(loc, Some(scope), token_id(name)).to_raw()
}

unsafe extern "C" fn const_global(colon: *const Token, name: *const Token) -> *mut Node {
    let loc = join_tokens(colon, name);

    Node::Const(loc, Some(Box::new(Node::Cbase(Token::loc(colon)))), token_id(name)).to_raw()
}

unsafe extern "C" fn const_op_assignable(node: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn cvar(tok: *const Token) -> *mut Node {
    Node::Cvar(Token::loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn dedent_string(node: *mut Node, dedent_level: size_t) -> *mut Node {
    if dedent_level != 0 {
        panic!("unimplemented dedent_string (dedent_level = {})", dedent_level); // TODO
    }

    return node;
}

unsafe extern "C" fn def_class(class_: *const Token, name: *mut Node, lt_: *const Token, superclass: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    Node::Class(join_tokens(class_, end_), from_raw(name), from_maybe_raw(superclass), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_method(def: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
    let loc = join_tokens(def, end);

    Node::Def(loc, token_id(name), from_maybe_raw(args), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_module(module: *const Token, name: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    Node::Module(join_tokens(module, end_), from_raw(name), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_sclass(class_: *const Token, lshft_: *const Token, expr: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    Node::SClass(join_tokens(class_, end_), from_raw(expr), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_singleton(def: *const Token, definee: *mut Node, dot: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
    let loc = join_tokens(def, end);

    Node::Defs(loc, from_raw(definee), token_id(name), from_maybe_raw(args), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn encoding_literal(tok: *const Token) -> *mut Node {
    Node::EncodingLiteral(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn false_(tok: *const Token) -> *mut Node {
    Node::False(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn file_literal(tok: *const Token) -> *mut Node {
    Node::FileLiteral(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn float_(tok: *const Token) -> *mut Node {
    Node::Float(Token::loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn float_complex(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn for_(for_: *const Token, iterator: *mut Node, in_: *const Token, iteratee: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn gvar(tok: *const Token) -> *mut Node {
    Node::Gvar(Token::loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn ident(tok: *const Token) -> *mut Node {
    Node::Ident(Token::loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn index(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node {
    let recv = from_raw(receiver);
    let indexes = ffi::node_list_from_raw(indexes);

    Node::Send(recv.loc().join(&Token::loc(rbrack)), Some(recv), Id(join_tokens(lbrack, rbrack), "[]".to_owned()), indexes).to_raw()
}

unsafe extern "C" fn index_asgn(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node {
    // Incomplete method call
    let recv = from_raw(receiver);
    let id = Id(join_tokens(lbrack, rbrack), "[]=".to_owned());
    let indexes = ffi::node_list_from_raw(indexes);
    Node::Send(recv.loc().clone(), Some(recv), id, indexes).to_raw()
}

unsafe extern "C" fn integer(tok: *const Token) -> *mut Node {
    Box::into_raw(Box::new(Node::Integer(Token::loc(tok), Token::string(tok))))
}

unsafe extern "C" fn ivar(tok: *const Token) -> *mut Node {
    Node::Ivar(Token::loc(tok), Token::string(tok)).to_raw()
}

unsafe extern "C" fn keyword_break(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    let mut loc = Token::loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Break(loc, args).to_raw()
}

unsafe extern "C" fn keyword_defined(keyword: *const Token, arg: *mut Node) -> *mut Node {
    let arg = from_raw(arg);
    Node::Defined(Token::loc(keyword).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn keyword_next(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    let mut loc = Token::loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Next(loc, args).to_raw()
}

unsafe extern "C" fn keyword_redo(keyword: *const Token) -> *mut Node {
    Node::Redo(Token::loc(keyword)).to_raw()
}

unsafe extern "C" fn keyword_retry(keyword: *const Token) -> *mut Node {
    Node::Retry(Token::loc(keyword)).to_raw()
}

unsafe extern "C" fn keyword_return(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    let mut loc = Token::loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Return(loc, args).to_raw()
}

unsafe extern "C" fn keyword_super(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    let mut loc = Token::loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Super(loc, args).to_raw()
}

unsafe extern "C" fn keyword_yield(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    let args = ffi::node_list_from_raw(args);

    let mut loc = Token::loc(keyword);

    if let Some(operand_loc) = collection_map(lparen, args.as_slice(), rparen) {
        loc = loc.join(&operand_loc);
    }

    Node::Yield(loc, args).to_raw()
}

unsafe extern "C" fn keyword_zsuper(keyword: *const Token) -> *mut Node {
    Node::ZSuper(Token::loc(keyword)).to_raw()
}

unsafe extern "C" fn kwarg(name: *const Token) -> *mut Node {
    Node::Kwarg(Token::loc(name), Token::string(name)).to_raw()
}

unsafe extern "C" fn kwoptarg(name: *const Token, value: *mut Node) -> *mut Node {
    let value = from_raw(value);
    let id = token_id(name);
    Node::Kwoptarg(id.0.join(value.loc()), id, value).to_raw()
}

unsafe extern "C" fn kwrestarg(dstar: *const Token, name: *const Token) -> *mut Node {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Kwrestarg(Token::loc(dstar).join(&id.0), Some(id))
    } else {
        Node::Kwrestarg(Token::loc(dstar), None)
    }.to_raw()
}

unsafe extern "C" fn kwsplat(dstar: *const Token, arg: *mut Node) -> *mut Node {
    let arg = from_raw(arg);
    Node::Kwsplat(Token::loc(dstar).join(arg.loc()), arg).to_raw()
}

unsafe extern "C" fn line_literal(tok: *const Token) -> *mut Node {
    Node::LineLiteral(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn logical_and(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);
    Node::And(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn logical_or(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);
    Node::Or(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn loop_until(keyword: *const Token, cond: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    let cond = from_raw(cond);
    let body = from_maybe_raw(body);
    Node::Until(join_tokens(keyword, end), cond, body).to_raw()
}

unsafe extern "C" fn loop_until_mod(body: *mut Node, cond: *mut Node) -> *mut Node {
    let cond = from_raw(cond);
    let body = from_raw(body);
    let loc = body.loc().join(cond.loc());

    match *body {
        Node::KwBegin(_, _) => Node::UntilPost(loc, cond, body),
        _ => Node::Until(loc, cond, Some(body))
    }.to_raw()
}

unsafe extern "C" fn loop_while(keyword: *const Token, cond: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    let cond = from_raw(cond);
    let body = from_maybe_raw(body);
    Node::While(join_tokens(keyword, end), cond, body).to_raw()
}

unsafe extern "C" fn loop_while_mod(body: *mut Node, cond: *mut Node) -> *mut Node {
    let cond = from_raw(cond);
    let body = from_raw(body);
    let loc = body.loc().join(cond.loc());

    match *body {
        Node::KwBegin(_, _) => Node::WhilePost(loc, cond, body),
        _ => Node::While(loc, cond, Some(body))
    }.to_raw()
}

unsafe extern "C" fn match_op(receiver: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node {
    let recv = from_raw(receiver);
    let arg = from_raw(arg);

    if let Node::Regexp(_, ref parts, _) = *recv {
        // TODO if parts are all static string literals, declare any named
        // captures as local variables and emit MatchWithLvasgn node
    }

    Node::Send(recv.loc().join(arg.loc()), Some(recv), token_id(oper), vec![arg]).to_raw()
}

unsafe extern "C" fn multi_assign(mlhs: *mut Node, rhs: *mut Node) -> *mut Node {
    let mlhs = from_raw(mlhs);
    let rhs = from_raw(rhs);

    Node::Masgn(mlhs.loc().join(rhs.loc()), mlhs, rhs).to_raw()
}

unsafe extern "C" fn multi_lhs(begin: *const Token, items: *mut NodeList, end: *const Token) -> *mut Node {
    let items = ffi::node_list_from_raw(items);

    Node::Mlhs(collection_map(begin, items.as_slice(), end).unwrap(), items).to_raw()
}

unsafe extern "C" fn negate(uminus: *const Token, numeric: *mut Node) -> *mut Node {
    let numeric = from_raw(numeric);
    let loc = Token::loc(uminus).join(numeric.loc());

    match *numeric {
        Node::Integer(_, value) => Node::Integer(loc, "-".to_owned() + value.as_str()),
        _ => panic!("unimplemented numeric type: {:?}", numeric),
    }.to_raw()
}

unsafe extern "C" fn nil(tok: *const Token) -> *mut Node {
    Node::Nil(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn not_op(not: *const Token, begin: *const Token, receiver: *mut Node, end: *const Token) -> *mut Node {
    let not_loc = Token::loc(not);
    let id = Id(Token::loc(not), "!".to_owned());

    match from_maybe_raw(receiver) {
        Some(expr) => {
            let loc = if end != ptr::null() {
                not_loc.join(&Token::loc(end))
            } else {
                not_loc.join(expr.loc())
            };
            Node::Send(loc, Some(expr), id, vec![])
        },
        None => {
            assert!(begin != ptr::null() && end != ptr::null());
            let nil_loc = join_tokens(begin, end);
            let loc = not_loc.join(&nil_loc);
            let recv = Box::new(Node::Begin(nil_loc.clone(), vec![Box::new(Node::Nil(nil_loc))]));
            Node::Send(loc, Some(recv), id, vec![])
        }
    }.to_raw()
}

unsafe extern "C" fn nth_ref(tok: *const Token) -> *mut Node {
    Node::NthRef(Token::loc(tok), Token::string(tok).parse().unwrap()).to_raw()
}

unsafe extern "C" fn op_assign(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node {
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

unsafe extern "C" fn optarg(name: *const Token, eql: *const Token, value: *mut Node) -> *mut Node {
    let id = token_id(name);
    let value = from_raw(value);
    Node::Optarg(id.0.join(value.loc()), id, value).to_raw()
}

unsafe extern "C" fn pair(key: *mut Node, assoc: *const Token, value: *mut Node) -> *mut Node {
    let key = from_raw(key);
    let value = from_raw(value);
    Node::Pair(key.loc().join(value.loc()), key, value).to_raw()
}

unsafe extern "C" fn pair_keyword(key: *const Token, value: *mut Node) -> *mut Node {
    let sym = Node::Symbol(Token::loc(key), Token::string(key));
    let value = from_raw(value);
    Node::Pair(sym.loc().join(value.loc()), Box::new(sym), value).to_raw()
}

unsafe extern "C" fn pair_quoted(begin: *const Token, parts: *mut NodeList, end: *const Token, value: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn postexe(body: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn preexe(node: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn procarg0(arg: *mut Node) -> *mut Node {
    let arg = from_raw(arg);
    Node::Procarg0(arg.loc().clone(), arg).to_raw()
}

unsafe extern "C" fn prototype(genargs: *mut Node, args: *mut Node, return_type: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn range_exclusive(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);

    Node::ERange(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn range_inclusive(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = from_raw(lhs);
    let rhs = from_raw(rhs);

    Node::IRange(lhs.loc().join(rhs.loc()), lhs, rhs).to_raw()
}

unsafe extern "C" fn rational(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn rational_complex(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn regexp_compose(begin: *const Token, parts: *mut NodeList, end: *const Token, options: *mut Node) -> *mut Node {
    let parts = ffi::node_list_from_raw(parts);
    let opts = from_maybe_raw(options);
    let begin_loc = Token::loc(begin);
    let loc = match opts {
        Some(ref opt_box) => begin_loc.join(opt_box.loc()),
        None => begin_loc.join(&Token::loc(end)),
    };
    Node::Regexp(loc, parts, opts).to_raw()
}

unsafe extern "C" fn regexp_options(regopt: *const Token) -> *mut Node {
    let mut options: Vec<char> = Token::string(regopt).chars().collect();
    options.sort();
    options.dedup();
    Node::Regopt(Token::loc(regopt), options).to_raw()
}

unsafe extern "C" fn rescue_body(rescue: *const Token, exc_list: *mut Node, assoc: *const Token, exc_var: *mut Node, then: *const Token, body: *mut Node) -> *mut Node {
    let exc_list = from_maybe_raw(exc_list);
    let exc_var = from_maybe_raw(exc_var);
    let body = from_maybe_raw(body);

    let mut loc = Token::loc(rescue);

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

unsafe extern "C" fn restarg(star: *const Token, name: *const Token) -> *mut Node {
    if name != ptr::null() {
        let id = token_id(name);
        Node::Restarg(Token::loc(star).join(&id.0), Some(id))
    } else {
        Node::Restarg(Token::loc(star), None)
    }.to_raw()
}

unsafe extern "C" fn self_(tok: *const Token) -> *mut Node {
    Node::Self_(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn shadowarg(name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn splat(star: *const Token, arg: *mut Node) -> *mut Node {
    let arg = from_maybe_raw(arg);
    let loc = match arg {
        Some(ref box_arg) => Token::loc(star).join(box_arg.loc()),
        None => Token::loc(star),
    };
    Node::Splat(loc, arg).to_raw()
}

unsafe extern "C" fn string(string_: *const Token) -> *mut Node {
    Node::String(Token::loc(string_), Token::string(string_)).to_raw()
}

unsafe extern "C" fn string_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    let mut parts = ffi::node_list_from_raw(parts);

    let loc = collection_map(begin, parts.as_slice(), end).unwrap();

    if parts.len() == 1 {
        let part = *parts.remove(0);

        match part {
            Node::String(loc, val) => Node::String(loc, val),
            node => Node::DString(loc, vec![Box::new(node)]),
        }
    } else {
        Node::DString(loc, parts)
    }.to_raw()
}

unsafe extern "C" fn string_internal(string: *const Token) -> *mut Node {
    Node::String(Token::loc(string), Token::string(string)).to_raw()
}

unsafe extern "C" fn symbol(symbol: *const Token) -> *mut Node {
    Node::Symbol(Token::loc(symbol), Token::string(symbol)).to_raw()
}

unsafe extern "C" fn symbol_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    let mut parts = ffi::node_list_from_raw(parts);

    let loc = collection_map(begin, parts.as_slice(), end).unwrap();

    if parts.len() == 1 {
        let part = *parts.remove(0);

        match part {
            Node::Symbol(loc, val) => Node::Symbol(loc, val),
            node => Node::DSymbol(loc, vec![Box::new(node)]),
        }
    } else {
        Node::DSymbol(loc, parts)
    }.to_raw()
}

unsafe extern "C" fn symbol_internal(symbol: *const Token) -> *mut Node {
    Node::Symbol(Token::loc(symbol), Token::string(symbol)).to_raw()
}

unsafe extern "C" fn symbols_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    let parts = ffi::node_list_from_raw(parts);

    let parts = parts.into_iter().map(|part| {
        let part = *part;
        Box::new(
            match part {
                Node::String(loc, val) => Node::Symbol(loc, val),
                Node::DString(loc, parts) => Node::DSymbol(loc, parts),
                node => node,
            }
        )
    }).collect::<Vec<_>>();

    Node::Array(collection_map(begin, parts.as_slice(), end).unwrap(), parts).to_raw()
}

unsafe extern "C" fn ternary(cond: *mut Node, question: *const Token, if_true: *mut Node, colon: *const Token, if_false: *mut Node) -> *mut Node {
    let cond = from_raw(cond);
    let if_true = from_raw(if_true);
    let if_false = from_raw(if_false);

    Node::If(cond.loc().join(if_false.loc()), Box::new(check_condition(*cond)), Some(if_true), Some(if_false)).to_raw()
}

unsafe extern "C" fn tr_array(begin: *const Token, type_: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_cast(begin: *const Token, expr: *mut Node, colon: *const Token, type_: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_cpath(cpath: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_genargs(begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_gendecl(cpath: *mut Node, begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_gendeclarg(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_geninst(cpath: *mut Node, begin: *const Token, genargs: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_hash(begin: *const Token, key_type: *mut Node, assoc: *const Token, value_type: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_ivardecl(name: *const Token, type_: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_nil(nil: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_nillable(tilde: *const Token, type_: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_or(a: *mut Node, b: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_proc(begin: *const Token, args: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_special(special: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn tr_tuple(begin: *const Token, types: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn true_(tok: *const Token) -> *mut Node {
    Node::True(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn typed_arg(type_: *mut Node, arg: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn unary_op(oper: *const Token, receiver: *mut Node) -> *mut Node {
    let id = token_id(oper);
    let recv = from_raw(receiver);

    let id = match id.1.as_str() {
        "+" => Id(id.0, "+@".to_owned()),
        "-" => Id(id.0, "-@".to_owned()),
        _   => id,
    };

    Node::Send(id.0.join(recv.loc()), Some(recv), id, vec![]).to_raw()
}

unsafe extern "C" fn undef_method(name_list: *mut NodeList) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn when(when: *const Token, patterns: *mut NodeList, then: *const Token, body: *mut Node) -> *mut Node {
    let patterns = ffi::node_list_from_raw(patterns);
    let body = from_maybe_raw(body);

    let when_loc = Token::loc(when);

    let loc = if let Some(ref body_box) = body {
        when_loc.join(body_box.loc())
    } else if then != ptr::null() {
        when_loc.join(&Token::loc(then))
    } else {
        when_loc.join(patterns.last().unwrap().loc())
    };

    Node::When(loc, patterns, body).to_raw()
}

unsafe extern "C" fn word(parts: *mut NodeList) -> *mut Node {
    let mut parts = ffi::node_list_from_raw(parts);

    if parts.len() == 1 {
        parts.remove(0)
    } else {
        assert!(!parts.is_empty());
        let loc = parts.first().unwrap().loc().join(parts.last().unwrap().loc());
        Box::new(Node::DString(loc, parts))
    }.to_raw()
}

unsafe extern "C" fn words_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    let words = ffi::node_list_from_raw(parts);
    Node::Array(collection_map(begin, words.as_slice(), end).unwrap(), words).to_raw()
}

unsafe extern "C" fn xstring_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
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
    tr_array: tr_array,
    tr_cast: tr_cast,
    tr_cpath: tr_cpath,
    tr_genargs: tr_genargs,
    tr_gendecl: tr_gendecl,
    tr_gendeclarg: tr_gendeclarg,
    tr_geninst: tr_geninst,
    tr_hash: tr_hash,
    tr_ivardecl: tr_ivardecl,
    tr_nil: tr_nil,
    tr_nillable: tr_nillable,
    tr_or: tr_or,
    tr_proc: tr_proc,
    tr_special: tr_special,
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

pub fn parse(filename: &str, source: &str) -> Ast {
    let parser = unsafe { Parser::new(source, &BUILDER) };
    let ast = unsafe { Parser::parse(parser) };
    let diagnostics = unsafe { Parser::diagnostics(parser) };
    unsafe { Parser::free(parser) };

    Ast {
        filename: filename.to_owned(),
        node: ast,
        diagnostics: diagnostics,
    }
}
