#![allow(improper_ctypes)]

extern crate libc;

use ::ast::{Node, DiagnosticLevel};
use self::libc::{size_t, c_int};
use std::vec::Vec;
use std::rc::Rc;
use std::ptr;
use std::slice;
use std::str;

include!("ffi_builder.rsinc");

pub enum Parser {}
pub enum Token {}
pub enum NodeList {}

#[link(name="rubyparser")]
#[cfg_attr(target_os="linux", link(name="stdc++"))]
#[cfg_attr(target_os="macos", link(name="c++"))]
extern "C" {
    fn ruby_parser_typedruby24_new(source: *const u8, source_length: size_t, builder: *const Builder) -> *mut Parser;
    fn ruby_parser_typedruby24_free(parser: *mut Parser);
    fn ruby_parser_parse(parser: *mut Parser) -> *mut Rc<Node>;
    fn ruby_parser_static_env_is_declared(p: *const Parser, name: *const u8, len: size_t) -> bool;
    fn ruby_parser_static_env_declare(p: *mut Parser, name: *const u8, len: size_t);
    fn ruby_parser_token_get_start(token: *const Token) -> size_t;
    fn ruby_parser_token_get_end(token: *const Token) -> size_t;
    fn ruby_parser_token_get_string(token: *const Token, ptr: *mut *const u8) -> size_t;
    fn ruby_parser_node_list_get_length(list: *mut NodeList) -> size_t;
    fn ruby_parser_node_list_index(list: *mut NodeList, index: size_t) -> *mut Rc<Node>;
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

    pub unsafe fn loc(ptr: *const Token) -> (usize, usize) {
        (Token::start(ptr), Token::end(ptr))
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

    pub unsafe fn parse(parser: *mut Parser) -> Option<Box<Rc<Node>>> {
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

    pub unsafe fn diagnostics(parser: *mut Parser) -> Vec<(DiagnosticLevel, String, usize, usize)> {
        let mut vec = Vec::new();

        for index in 0..ruby_parser_diagnostics_get_length(parser) {
            let mut message_ptr: *const u8 = ptr::null();
            let message_len = ruby_parser_diagnostic_get_message(parser, index, &mut message_ptr);
            let message = String::from(str::from_utf8_unchecked(slice::from_raw_parts(message_ptr, message_len)));

            let level = match ruby_parser_diagnostic_get_level(parser, index) {
                1 => DiagnosticLevel::Note,
                2 => DiagnosticLevel::Warning,
                3 => DiagnosticLevel::Error,
                4 => DiagnosticLevel::Fatal,
                _ => panic!("bad diagnostic level"),
            };

            let begin = ruby_parser_diagnostic_get_begin(parser, index);

            let end = ruby_parser_diagnostic_get_end(parser, index);

            vec.push((level, message, begin, end));
        }

        vec
    }
}

pub unsafe fn node_list_from_raw(list: *mut NodeList) -> Vec<Rc<Node>> {
    let mut vec = Vec::new();

    if list == ptr::null_mut() {
        return vec;
    }

    for index in 0..ruby_parser_node_list_get_length(list) {
        let node_ptr = ruby_parser_node_list_index(list, index);

        assert!(node_ptr != ptr::null_mut());

        vec.push(*Box::from_raw(node_ptr));
    }

    vec
}
