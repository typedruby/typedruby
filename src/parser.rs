#![allow(unused_variables)]

extern crate libc;

use std::ptr;

use ast::*;
use ffi;
use ffi::{Builder, NodeList, Token};
use self::libc::{size_t, c_int};

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
    Box::from_raw(p)
}

unsafe extern "C" fn accessible(node: *mut Node) -> *mut Node {
    node
}

unsafe extern "C" fn alias(alias: *const Token, to: *mut Node, from: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn arg(name: *const Token) -> *mut Node {
    Node::Arg(Token::loc(name), Token::string(name)).to_raw()
}

fn check_duplicate_args<'a>(args: &'a [Box<Node>]) {
    let mut names = ::std::collections::HashSet::new();

    fn arg_loc_and_name<'a>(node: &'a Node) -> (&'a Loc, &'a str) {
        match node {
            &Node::Arg(ref loc, ref name) => (&loc, &name),
            _ => panic!("not an arg node"),
        }
    }

    let mut check_inner = |args: &'a [Box<Node>]| {
        for arg in args {
            let (range, name) = arg_loc_and_name(&*arg);

            if name.starts_with("_") {
                continue
            }

            if names.contains(name) {
                // TODO error
            }

            names.insert(name);
        }
    };

    check_inner(args)
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

    collection_map(begin, args.as_slice(), end).map(|loc|
        Node::Args(loc, args)
    ).to_raw()
}

unsafe extern "C" fn array(begin: *const Token, elements: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn assign(lhs: *mut Node, eql: *const Token, rhs: *mut Node) -> *mut Node {
    let lhs = *from_raw(lhs);
    let rhs = from_raw(rhs);

    match lhs {
        Node::Send(loc, recv, mid, mut args) => {
            let loc = loc.join((&rhs).loc());
            args.push(rhs);
            Node::Send(loc, recv, mid, args)
        },
        Node::CSend(loc, recv, mid, mut args) => {
            let loc = loc.join((&rhs).loc());
            args.push(rhs);
            Node::CSend(loc, recv, mid, args)
        },
        _ => {
            panic!("unimplemented lhs: {:?}", lhs);
        }
    }.to_raw()
}

unsafe extern "C" fn assignable(node: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn associate(begin: *const Token, pairs: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
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
    panic!("unimplemented");
}

unsafe extern "C" fn binary_op(recv: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node {
    let recv = from_raw(recv);
    let arg = from_raw(arg);

    Node::Send(recv.loc().join(arg.loc()), Some(recv), token_id(oper), vec![arg]).to_raw()
}

unsafe extern "C" fn block(method_call: *mut Node, begin: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn block_pass(amper: *const Token, arg: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn blockarg(amper: *const Token, name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn call_lambda(lambda: *const Token) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
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

unsafe extern "C" fn condition(cond_tok: *const Token, cond: *mut Node, then: *const Token, if_true: *mut Node, else_: *const Token, if_false: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn condition_mod(if_true: *mut Node, if_false: *mut Node, cond: *mut Node) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
}

unsafe extern "C" fn dedent_string(node: *mut Node, dedent_level: size_t) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn def_class(class_: *const Token, name: *mut Node, lt_: *const Token, superclass: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn def_method(def: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
    let loc = join_tokens(def, end);

    Node::Def(loc, token_id(name), from_maybe_raw(args), from_maybe_raw(body)).to_raw()
}

unsafe extern "C" fn def_module(module: *const Token, name: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn def_sclass(class_: *const Token, lshft_: *const Token, expr: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn def_singleton(def: *const Token, definee: *mut Node, dot: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
}

unsafe extern "C" fn float_complex(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn for_(for_: *const Token, iterator: *mut Node, in_: *const Token, iteratee: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn gvar(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn ident(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn index(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn index_asgn(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn integer(tok: *const Token) -> *mut Node {
    Box::into_raw(Box::new(Node::Integer(Token::loc(tok), Token::string(tok))))
}

unsafe extern "C" fn ivar(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn keyword_cmd(type_: c_int, keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn kwarg(name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn kwoptarg(name: *const Token, value: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn kwrestarg(dstar: *const Token, name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn kwsplat(dstar: *const Token, arg: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn line_literal(tok: *const Token) -> *mut Node {
    Node::LineLiteral(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn logical_op(type_: c_int, lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn loop_(type_: c_int, keyword: *const Token, cond: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn loop_mod(type_: c_int, body: *mut Node, cond: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn match_op(receiver: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn multi_assign(mlhs: *mut Node, rhs: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn multi_lhs(begin: *const Token, items: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn negate(uminus: *const Token, numeric: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn nil(tok: *const Token) -> *mut Node {
    Node::Nil(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn not_op(not_: *const Token, begin: *const Token, receiver: *mut Node, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn nth_ref(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn op_assign(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn optarg(name: *const Token, eql: *const Token, value: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn pair(key: *mut Node, assoc: *const Token, value: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn pair_keyword(key: *const Token, value: *mut Node) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
}

unsafe extern "C" fn prototype(genargs: *mut Node, args: *mut Node, return_type: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn range_exclusive(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn range_inclusive(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn rational(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn rational_complex(tok: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn regexp_compose(begin: *const Token, parts: *mut NodeList, end: *const Token, options: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn regexp_options(regopt: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn rescue_body(rescue: *const Token, exc_list: *mut Node, assoc: *const Token, exc_var: *mut Node, then: *const Token, body: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn restarg(star: *const Token, name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn self_(tok: *const Token) -> *mut Node {
    Node::Self_(Token::loc(tok)).to_raw()
}

unsafe extern "C" fn shadowarg(name: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn splat(star: *const Token, arg: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn string(string_: *const Token) -> *mut Node {
    Node::String(Token::loc(string_), Token::string(string_)).to_raw()
}

unsafe extern "C" fn string_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn string_internal(string_: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn symbol(symbol: *const Token) -> *mut Node {
    Node::Symbol(Token::loc(symbol), Token::string(symbol)).to_raw()
}

unsafe extern "C" fn symbol_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn symbol_internal(symbol: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn symbols_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn ternary(cond: *mut Node, question: *const Token, if_true: *mut Node, colon: *const Token, if_false: *mut Node) -> *mut Node {
    panic!("unimplemented");
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
    panic!("unimplemented");
}

unsafe extern "C" fn undef_method(name_list: *mut NodeList) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn when(when: *const Token, patterns: *mut NodeList, then: *const Token, body: *mut Node) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn word(parts: *mut NodeList) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn words_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

unsafe extern "C" fn xstring_compose(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node {
    panic!("unimplemented");
}

const BUILDER: Builder = Builder {
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
    keyword_cmd: keyword_cmd,
    kwarg: kwarg,
    kwoptarg: kwoptarg,
    kwrestarg: kwrestarg,
    kwsplat: kwsplat,
    line_literal: line_literal,
    logical_op: logical_op,
    loop_: loop_,
    loop_mod: loop_mod,
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
    Ast {
        filename: filename.to_owned(),
        node: unsafe {
            from_maybe_raw(ffi::ruby_parser_typedruby24_parse(source.as_ptr(), source.len(), &BUILDER))
        },
    }
}
