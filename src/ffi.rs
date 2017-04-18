#![allow(improper_ctypes)]

extern crate libc;

use ::ast::{Node, Loc, Diagnostic, DiagnosticLevel};
use self::libc::{size_t, c_int};
use std::vec::Vec;
use std::ptr;
use std::slice;
use std::str;

#[repr(C)]
pub struct Builder {
    pub accessible: unsafe extern "C" fn(p: *mut Parser, node: *mut Node) -> *mut Node,
    pub alias: unsafe extern "C" fn(alias: *const Token, to: *mut Node, from: *mut Node) -> *mut Node,
    pub arg: unsafe extern "C" fn(name: *const Token) -> *mut Node,
    pub args: unsafe extern "C" fn(begin: *const Token, args: *mut NodeList, end: *const Token, check_args: bool) -> *mut Node,
    pub array: unsafe extern "C" fn(begin: *const Token, elements: *mut NodeList, end: *const Token) -> *mut Node,
    pub assign: unsafe extern "C" fn(lhs: *mut Node, eql: *const Token, rhs: *mut Node) -> *mut Node,
    pub assignable: unsafe extern "C" fn(p: *mut Parser, node: *mut Node) -> *mut Node,
    pub associate: unsafe extern "C" fn(begin: *const Token, pairs: *mut NodeList, end: *const Token) -> *mut Node,
    pub attr_asgn: unsafe extern "C" fn(receiver: *mut Node, dot: *const Token, selector: *const Token) -> *mut Node,
    pub back_ref: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub begin: unsafe extern "C" fn(begin: *const Token, body: *mut Node, end: *const Token) -> *mut Node,
    pub begin_body: unsafe extern "C" fn(body: *mut Node, rescue_bodies: *mut NodeList, else_tok: *const Token, else_: *mut Node, ensure_tok: *const Token, ensure: *mut Node) -> *mut Node,
    pub begin_keyword: unsafe extern "C" fn(begin: *const Token, body: *mut Node, end: *const Token) -> *mut Node,
    pub binary_op: unsafe extern "C" fn(receiver: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node,
    pub block: unsafe extern "C" fn(method_call: *mut Node, begin: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node,
    pub block_pass: unsafe extern "C" fn(amper: *const Token, arg: *mut Node) -> *mut Node,
    pub blockarg: unsafe extern "C" fn(amper: *const Token, name: *const Token) -> *mut Node,
    pub call_lambda: unsafe extern "C" fn(lambda: *const Token) -> *mut Node,
    pub call_method: unsafe extern "C" fn(receiver: *mut Node, dot: *const Token, selector: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub case_: unsafe extern "C" fn(case_: *const Token, expr: *mut Node, when_bodies: *mut NodeList, else_tok: *const Token, else_body: *mut Node, end: *const Token) -> *mut Node,
    pub character: unsafe extern "C" fn(char_: *const Token) -> *mut Node,
    pub complex: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub compstmt: unsafe extern "C" fn(nodes: *mut NodeList) -> *mut Node,
    pub condition: unsafe extern "C" fn(cond_tok: *const Token, cond: *mut Node, then: *const Token, if_true: *mut Node, else_: *const Token, if_false: *mut Node, end: *const Token) -> *mut Node,
    pub condition_mod: unsafe extern "C" fn(if_true: *mut Node, if_false: *mut Node, cond: *mut Node) -> *mut Node,
    pub const_: unsafe extern "C" fn(name: *const Token) -> *mut Node,
    pub const_fetch: unsafe extern "C" fn(scope: *mut Node, colon: *const Token, name: *const Token) -> *mut Node,
    pub const_global: unsafe extern "C" fn(colon: *const Token, name: *const Token) -> *mut Node,
    pub const_op_assignable: unsafe extern "C" fn(node: *mut Node) -> *mut Node,
    pub cvar: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub dedent_string: unsafe extern "C" fn(node: *mut Node, dedent_level: size_t) -> *mut Node,
    pub def_class: unsafe extern "C" fn(class_: *const Token, name: *mut Node, lt_: *const Token, superclass: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node,
    pub def_method: unsafe extern "C" fn(def: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node,
    pub def_module: unsafe extern "C" fn(module: *const Token, name: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node,
    pub def_sclass: unsafe extern "C" fn(class_: *const Token, lshft_: *const Token, expr: *mut Node, body: *mut Node, end_: *const Token) -> *mut Node,
    pub def_singleton: unsafe extern "C" fn(def: *const Token, definee: *mut Node, dot: *const Token, name: *const Token, args: *mut Node, body: *mut Node, end: *const Token) -> *mut Node,
    pub encoding_literal: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub false_: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub file_literal: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub float_: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub float_complex: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub for_: unsafe extern "C" fn(for_: *const Token, iterator: *mut Node, in_: *const Token, iteratee: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node,
    pub gvar: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub ident: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub index: unsafe extern "C" fn(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node,
    pub index_asgn: unsafe extern "C" fn(receiver: *mut Node, lbrack: *const Token, indexes: *mut NodeList, rbrack: *const Token) -> *mut Node,
    pub integer: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub ivar: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub keyword_break: unsafe extern "C" fn(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub keyword_defined: unsafe extern "C" fn(keyword: *const Token, arg: *mut Node) -> *mut Node,
    pub keyword_next: unsafe extern "C" fn(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub keyword_redo: unsafe extern "C" fn(keyword: *const Token) -> *mut Node,
    pub keyword_retry: unsafe extern "C" fn(keyword: *const Token) -> *mut Node,
    pub keyword_return: unsafe extern "C" fn(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub keyword_super: unsafe extern "C" fn(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub keyword_yield: unsafe extern "C" fn(keyword: *const Token, lparen: *const Token, args: *mut NodeList, rparen: *const Token) -> *mut Node,
    pub keyword_zsuper: unsafe extern "C" fn(keyword: *const Token) -> *mut Node,
    pub kwarg: unsafe extern "C" fn(name: *const Token) -> *mut Node,
    pub kwoptarg: unsafe extern "C" fn(name: *const Token, value: *mut Node) -> *mut Node,
    pub kwrestarg: unsafe extern "C" fn(dstar: *const Token, name: *const Token) -> *mut Node,
    pub kwsplat: unsafe extern "C" fn(dstar: *const Token, arg: *mut Node) -> *mut Node,
    pub line_literal: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub logical_and: unsafe extern "C" fn(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node,
    pub logical_or: unsafe extern "C" fn(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node,
    pub loop_until: unsafe extern "C" fn(keyword: *const Token, cond: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node,
    pub loop_until_mod: unsafe extern "C" fn(body: *mut Node, cond: *mut Node) -> *mut Node,
    pub loop_while: unsafe extern "C" fn(keyword: *const Token, cond: *mut Node, do_: *const Token, body: *mut Node, end: *const Token) -> *mut Node,
    pub loop_while_mod: unsafe extern "C" fn(body: *mut Node, cond: *mut Node) -> *mut Node,
    pub match_op: unsafe extern "C" fn(receiver: *mut Node, oper: *const Token, arg: *mut Node) -> *mut Node,
    pub multi_assign: unsafe extern "C" fn(mlhs: *mut Node, rhs: *mut Node) -> *mut Node,
    pub multi_lhs: unsafe extern "C" fn(begin: *const Token, items: *mut NodeList, end: *const Token) -> *mut Node,
    pub negate: unsafe extern "C" fn(uminus: *const Token, numeric: *mut Node) -> *mut Node,
    pub nil: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub not_op: unsafe extern "C" fn(not_: *const Token, begin: *const Token, receiver: *mut Node, end: *const Token) -> *mut Node,
    pub nth_ref: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub op_assign: unsafe extern "C" fn(lhs: *mut Node, op: *const Token, rhs: *mut Node) -> *mut Node,
    pub optarg: unsafe extern "C" fn(name: *const Token, eql: *const Token, value: *mut Node) -> *mut Node,
    pub pair: unsafe extern "C" fn(key: *mut Node, assoc: *const Token, value: *mut Node) -> *mut Node,
    pub pair_keyword: unsafe extern "C" fn(key: *const Token, value: *mut Node) -> *mut Node,
    pub pair_quoted: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token, value: *mut Node) -> *mut Node,
    pub postexe: unsafe extern "C" fn(begin: *const Token, node: *mut Node, rbrace: *const Token) -> *mut Node,
    pub preexe: unsafe extern "C" fn(begin: *const Token, node: *mut Node, rbrace: *const Token) -> *mut Node,
    pub procarg0: unsafe extern "C" fn(arg: *mut Node) -> *mut Node,
    pub prototype: unsafe extern "C" fn(genargs: *mut Node, args: *mut Node, return_type: *mut Node) -> *mut Node,
    pub range_exclusive: unsafe extern "C" fn(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node,
    pub range_inclusive: unsafe extern "C" fn(lhs: *mut Node, oper: *const Token, rhs: *mut Node) -> *mut Node,
    pub rational: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub rational_complex: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub regexp_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token, options: *mut Node) -> *mut Node,
    pub regexp_options: unsafe extern "C" fn(regopt: *const Token) -> *mut Node,
    pub rescue_body: unsafe extern "C" fn(rescue: *const Token, exc_list: *mut Node, assoc: *const Token, exc_var: *mut Node, then: *const Token, body: *mut Node) -> *mut Node,
    pub restarg: unsafe extern "C" fn(star: *const Token, name: *const Token) -> *mut Node,
    pub self_: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub shadowarg: unsafe extern "C" fn(name: *const Token) -> *mut Node,
    pub splat: unsafe extern "C" fn(star: *const Token, arg: *mut Node) -> *mut Node,
    pub string: unsafe extern "C" fn(string_: *const Token) -> *mut Node,
    pub string_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node,
    pub string_internal: unsafe extern "C" fn(string_: *const Token) -> *mut Node,
    pub symbol: unsafe extern "C" fn(symbol: *const Token) -> *mut Node,
    pub symbol_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node,
    pub symbol_internal: unsafe extern "C" fn(symbol: *const Token) -> *mut Node,
    pub symbols_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node,
    pub ternary: unsafe extern "C" fn(cond: *mut Node, question: *const Token, if_true: *mut Node, colon: *const Token, if_false: *mut Node) -> *mut Node,
    pub tr_array: unsafe extern "C" fn(begin: *const Token, type_: *mut Node, end: *const Token) -> *mut Node,
    pub tr_cast: unsafe extern "C" fn(begin: *const Token, expr: *mut Node, colon: *const Token, type_: *mut Node, end: *const Token) -> *mut Node,
    pub tr_cpath: unsafe extern "C" fn(cpath: *mut Node) -> *mut Node,
    pub tr_genargs: unsafe extern "C" fn(begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Node,
    pub tr_gendecl: unsafe extern "C" fn(cpath: *mut Node, begin: *const Token, genargs: *mut NodeList, end: *const Token) -> *mut Node,
    pub tr_gendeclarg: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub tr_geninst: unsafe extern "C" fn(cpath: *mut Node, begin: *const Token, genargs: *mut Node, end: *const Token) -> *mut Node,
    pub tr_hash: unsafe extern "C" fn(begin: *const Token, key_type: *mut Node, assoc: *const Token, value_type: *mut Node, end: *const Token) -> *mut Node,
    pub tr_ivardecl: unsafe extern "C" fn(name: *const Token, type_: *mut Node) -> *mut Node,
    pub tr_nil: unsafe extern "C" fn(nil: *const Token) -> *mut Node,
    pub tr_nillable: unsafe extern "C" fn(tilde: *const Token, type_: *mut Node) -> *mut Node,
    pub tr_or: unsafe extern "C" fn(a: *mut Node, b: *mut Node) -> *mut Node,
    pub tr_proc: unsafe extern "C" fn(begin: *const Token, args: *mut Node, end: *const Token) -> *mut Node,
    pub tr_special: unsafe extern "C" fn(special: *const Token) -> *mut Node,
    pub tr_tuple: unsafe extern "C" fn(begin: *const Token, types: *mut NodeList, end: *const Token) -> *mut Node,
    pub true_: unsafe extern "C" fn(tok: *const Token) -> *mut Node,
    pub typed_arg: unsafe extern "C" fn(type_: *mut Node, arg: *mut Node) -> *mut Node,
    pub unary_op: unsafe extern "C" fn(oper: *const Token, receiver: *mut Node) -> *mut Node,
    pub undef_method: unsafe extern "C" fn(undef: *const Token, name_list: *mut NodeList) -> *mut Node,
    pub when: unsafe extern "C" fn(when: *const Token, patterns: *mut NodeList, then: *const Token, body: *mut Node) -> *mut Node,
    pub word: unsafe extern "C" fn(parts: *mut NodeList) -> *mut Node,
    pub words_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node,
    pub xstring_compose: unsafe extern "C" fn(begin: *const Token, parts: *mut NodeList, end: *const Token) -> *mut Node,
}

pub enum Parser {}
pub enum Token {}
pub enum NodeList {}

#[link(name="rubyparser")]
#[link(name="c++")]
extern "C" {
    fn ruby_parser_typedruby24_new(source: *const u8, source_length: size_t, builder: *const Builder) -> *mut Parser;
    fn ruby_parser_typedruby24_free(parser: *mut Parser);
    fn ruby_parser_parse(parser: *mut Parser) -> *mut Node;
    fn ruby_parser_static_env_is_declared(p: *const Parser, name: *const u8, len: size_t) -> bool;
    fn ruby_parser_static_env_declare(p: *mut Parser, name: *const u8, len: size_t);
    fn ruby_parser_token_get_start(token: *const Token) -> size_t;
    fn ruby_parser_token_get_end(token: *const Token) -> size_t;
    fn ruby_parser_token_get_string(token: *const Token, ptr: *mut *const u8) -> size_t;
    fn ruby_parser_node_list_get_length(list: *mut NodeList) -> size_t;
    fn ruby_parser_node_list_index(list: *mut NodeList, index: size_t) -> *mut Node;
    fn ruby_parser_diagnostics_get_length(parser: *const Parser) -> size_t;
    fn ruby_parser_diagnostic_get_level(parser: *const Parser, index: size_t) -> c_int;
    fn ruby_parser_diagnostic_get_message(parser: *const Parser, index: size_t, ptr: *mut *const u8) -> size_t;
    fn ruby_parser_diagnostic_get_begin(parser: *const Parser, index: size_t) -> size_t;
    fn ruby_parser_diagnostic_get_end(parser: *const Parser, index: size_t) -> size_t;
}

impl Token {
    pub unsafe fn start(ptr: *const Token) -> usize {
        ruby_parser_token_get_start(ptr)
    }

    pub unsafe fn end(ptr: *const Token) -> usize {
        ruby_parser_token_get_end(ptr)
    }

    pub unsafe fn loc(ptr: *const Token) -> Loc {
        Loc {
            begin_pos: Token::start(ptr),
            end_pos: Token::end(ptr),
        }
    }

    pub unsafe fn string(ptr: *const Token) -> String {
        let mut string: *const u8 = ptr::null();
        let string_length = ruby_parser_token_get_string(ptr, &mut string);
        String::from(str::from_utf8_unchecked(slice::from_raw_parts(string, string_length)))
    }
}

impl Parser {
    pub unsafe fn new(source: &str, builder: &'static Builder) -> *mut Parser {
        ruby_parser_typedruby24_new(source.as_ptr(), source.len(), builder)
    }

    pub unsafe fn free(parser: *mut Parser) {
        ruby_parser_typedruby24_free(parser);
    }

    pub unsafe fn parse(parser: *mut Parser) -> Option<Box<Node>> {
        let ptr = ruby_parser_parse(parser);

        if ptr == ptr::null_mut() {
            None
        } else {
            Some(Box::from_raw(ptr))
        }
    }

    pub unsafe fn is_declared(parser: *const Parser, id: &str) -> bool {
        ruby_parser_static_env_is_declared(parser, id.as_ptr(), id.len())
    }

    pub unsafe fn declare(parser: *mut Parser, id: &str) {
        ruby_parser_static_env_declare(parser, id.as_ptr(), id.len());
    }

    pub unsafe fn diagnostics(parser: *mut Parser) -> Vec<Diagnostic> {
        let mut vec = Vec::new();

        for index in 0..ruby_parser_diagnostics_get_length(parser) {
            let mut message_ptr: *const u8 = ptr::null();
            let message_len = ruby_parser_diagnostic_get_message(parser, index, &mut message_ptr);
            let message = String::from(str::from_utf8_unchecked(slice::from_raw_parts(message_ptr, message_len)));

            vec.push(Diagnostic {
                level: match ruby_parser_diagnostic_get_level(parser, index) {
                    1 => DiagnosticLevel::Note,
                    2 => DiagnosticLevel::Warning,
                    3 => DiagnosticLevel::Error,
                    4 => DiagnosticLevel::Fatal,
                    _ => panic!("bad diagnostic level"),
                },
                message: message,
                loc: Loc {
                    begin_pos: ruby_parser_diagnostic_get_begin(parser, index),
                    end_pos: ruby_parser_diagnostic_get_end(parser, index),
                },
            })
        }

        vec
    }
}

pub unsafe fn node_list_from_raw(list: *mut NodeList) -> Vec<Box<Node>> {
    let mut vec = Vec::new();

    if list == ptr::null_mut() {
        return vec;
    }

    for index in 0..ruby_parser_node_list_get_length(list) {
        let node_ptr = ruby_parser_node_list_index(list, index);

        assert!(node_ptr != ptr::null_mut());

        vec.push(Box::from_raw(node_ptr));
    }

    vec
}
