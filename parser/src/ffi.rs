#![allow(improper_ctypes)]

extern crate libc;

use ::ast::{Node, Loc, SourceRef, Diagnostic, Level, Error, Comment};
use ::builder::Builder;
use ::parser::{ParserOptions, ParserMode};
use self::libc::{size_t, c_char, c_int};
use std::ffi::{CStr, CString};
use std::vec::Vec;
use std::rc::Rc;
use std::ptr;
use std::slice;
use std::str;
use std::mem;
use id_arena::IdArena;

type NodeId = usize;

trait ToRaw {
    fn to_raw(self, builder: &mut Builder) -> NodeId;
}

impl ToRaw for Option<Rc<Node>> {
    fn to_raw(self, builder: &mut Builder) -> NodeId {
        builder.nodes.insert(self)
    }
}

impl ToRaw for Rc<Node> {
    fn to_raw(self, builder: &mut Builder) -> NodeId {
        builder.nodes.insert(Some(self))
    }
}

impl ToRaw for Option<Node> {
    fn to_raw(self, builder: &mut Builder) -> NodeId {
        builder.nodes.insert(self.map(Rc::new))
    }
}

impl ToRaw for Node {
    fn to_raw(self, builder: &mut Builder) -> NodeId {
        builder.nodes.insert(Some(Rc::new(self)))
    }
}

#[inline(always)]
unsafe fn node_from_c(builder: &Builder, p: NodeId) -> Option<Rc<Node>> {
    builder.nodes.get(p).cloned()
}

#[inline(always)]
unsafe fn token_from_c(t: *const TokenPtr) -> Option<Token> {
    if t.is_null() {
        None
    } else {
        Some(Token {token: t})
    }
}

#[inline(always)]
unsafe fn node_list_from_c(builder: &Builder, list: *mut NodeListPtr) -> Vec<Rc<Node>> {
    if list.is_null() {
        Vec::new()
    } else {
        let len = rblist_get_length(list);
        let mut vec = Vec::with_capacity(len);

        for index in 0..len {
            let node_ptr = rblist_index(list, index);
            let node = builder.nodes.get(node_ptr).cloned()
                .expect("node list should not contain None node");
            vec.push(node);
        }

        vec
    }
}

pub enum DriverPtr {}
pub enum TokenPtr {}
pub enum NodeListPtr {}

#[repr(C)]
struct CDiagnostic {
    level: Level,
    class: Error,
    data: *const c_char,
    begin_pos: size_t,
    end_pos: size_t,
}

include!(concat!(env!("OUT_DIR"), "/ffi_builder.rs"));

extern "C" {
    fn rbdriver_typedruby24_new(mode: c_int, source: *const u8, source_length: size_t, builder: *const BuilderInterface) -> *mut DriverPtr;
    fn rbdriver_typedruby24_free(driver: *mut DriverPtr);
    fn rbdriver_parse(driver: *mut DriverPtr, builder: *mut Builder) -> NodeId;
    fn rbdriver_in_definition(driver: *const DriverPtr) -> bool;
    fn rbdriver_env_is_declared(driver: *const DriverPtr, name: *const u8, len: size_t) -> bool;
    fn rbdriver_env_declare(driver: *mut DriverPtr, name: *const u8, len: size_t);
    fn rbtoken_get_start(token: *const TokenPtr) -> size_t;
    fn rbtoken_get_end(token: *const TokenPtr) -> size_t;
    fn rbtoken_get_string(token: *const TokenPtr, ptr: *mut *const u8) -> size_t;
    fn rblist_get_length(list: *mut NodeListPtr) -> size_t;
    fn rblist_index(list: *mut NodeListPtr, index: size_t) -> NodeId;
    fn rbdriver_diag_get_length(driver: *const DriverPtr) -> size_t;
    fn rbdriver_diag_get(driver: *const DriverPtr, index: size_t, diag: *mut CDiagnostic);
    fn rbdriver_diag_report(driver: *const DriverPtr, diag: *const CDiagnostic);
    fn rbdriver_comment_get_length(driver: *const DriverPtr) -> size_t;
    fn rbdriver_comment_get_begin(driver: *const DriverPtr, index: size_t) -> size_t;
    fn rbdriver_comment_get_end(driver: *const DriverPtr, index: size_t) -> size_t;
    fn rbdriver_comment_get_string(driver: *const DriverPtr, index: size_t, ptr: *mut *const u8) -> size_t;
}

pub struct Token {
    token: *const TokenPtr,
}

impl Token {
    pub fn begin_pos(&self) -> usize {
        unsafe { rbtoken_get_start(self.token) }
    }

    pub fn end_pos(&self) -> usize {
        unsafe { rbtoken_get_end(self.token) }
    }

    pub fn string(&self) -> String {
        unsafe {
            let mut string: *const u8 = ptr::null();
            let string_length = rbtoken_get_string(self.token, &mut string);
            String::from(str::from_utf8_unchecked(slice::from_raw_parts(string, string_length)))
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        unsafe {
            let mut string: *const u8 = ptr::null();
            let mut bytes: Vec<u8> = Vec::new();
            let string_length = rbtoken_get_string(self.token, &mut string);
            bytes.extend_from_slice(slice::from_raw_parts(string, string_length));
            bytes
        }
    }
}

pub struct Comments<'d, 'a: 'd> {
    driver: &'d Driver<'a>,
}

impl<'d, 'a> Comments<'d, 'a> {
    pub fn len(&self) -> usize {
        unsafe { rbdriver_comment_get_length(self.driver.ptr) }
    }

    pub fn at(&self, idx: usize) -> Comment {
        let begin_pos = unsafe { rbdriver_comment_get_begin(self.driver.ptr, idx) };
        let end_pos = unsafe { rbdriver_comment_get_end(self.driver.ptr, idx) };

        let mut string: *const u8 = ptr::null();
        let string_length = unsafe { rbdriver_comment_get_string(self.driver.ptr, idx, &mut string) };

        let contents = unsafe {
            String::from(str::from_utf8_unchecked(slice::from_raw_parts(string, string_length)))
        };

        Comment {
            loc: self.driver.source_ref.make_loc(begin_pos, end_pos),
            contents: contents,
        }
    }

    pub fn before(&self, pos: usize) -> Option<Comment> {
        let mut i = self.len();

        while i > 0 {
            i = i - 1;

            let comment = self.at(i);

            if comment.loc.end_pos <= pos {
                return Some(comment);
            }
        }

        None
    }
}

pub struct Driver<'a> {
    ptr: *mut DriverPtr,
    pub(crate) opt: ParserOptions<'a>,
    pub source_ref: SourceRef,
}

impl<'a> Drop for Driver<'a> {
    fn drop(&mut self) {
        unsafe { rbdriver_typedruby24_free(self.ptr); }
    }
}

impl<'a> Driver<'a> {
    pub fn new(opt: ParserOptions<'a>, file: SourceRef) -> Self {
        let mode = match opt.mode {
            ParserMode::Program => 1,
            ParserMode::Prototype => 2,
        };

        let ptr = {
            let source = file.source();
            unsafe { rbdriver_typedruby24_new(mode, source.as_ptr(), source.len(), &CALLBACKS) }
        };

        Driver { ptr: ptr, opt: opt, source_ref: file }
    }

    pub fn parse(&mut self) -> Option<Rc<Node>> {
        for var in self.opt.declare_env.iter() {
            self.declare(var);
        }

        let driver = self.ptr;

        let mut builder = Builder {
            cookie: 12345678,
            magic_literals: self.opt.emit_file_vars_as_literals,
            emit_lambda: self.opt.emit_lambda,
            emit_procarg0: self.opt.emit_procarg0,
            nodes: IdArena::new(),
            driver: self,
        };

        let ast = unsafe { rbdriver_parse(driver, &mut builder) };

        builder.nodes.get(ast).cloned()
    }

    pub fn diagnostic(&mut self, level: Level, err: Error, loc: Loc, data: Option<&str>) {
        let data = data.map(|data| CString::new(data.to_owned()).unwrap());

        let ptr = data.as_ref().map(|cstr| cstr.as_ptr()).unwrap_or(ptr::null());

        let diag = CDiagnostic {
            level: level,
            class: err,
            data: ptr,
            begin_pos: loc.begin_pos,
            end_pos: loc.end_pos,
        };

        unsafe {
            rbdriver_diag_report(self.ptr, &diag);
        }
    }

    pub fn error(&mut self, err: Error, loc: Loc) {
        self.diagnostic(Level::Error, err, loc, None)
    }

    pub fn is_in_definition(&self) -> bool {
        unsafe { rbdriver_in_definition(self.ptr) }
    }

    pub fn is_declared(&self, id: &str) -> bool {
        unsafe { rbdriver_env_is_declared(self.ptr, id.as_ptr(), id.len()) }
    }

    pub fn declare(&mut self, id: &str) {
        unsafe { rbdriver_env_declare(self.ptr, id.as_ptr(), id.len()); }
    }

    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let len = unsafe { rbdriver_diag_get_length(self.ptr) };
        let mut vec = Vec::with_capacity(len);

        for index in 0..len {
            let cdiag = unsafe {
                let mut diag: CDiagnostic = mem::uninitialized();
                rbdriver_diag_get(self.ptr, index, &mut diag);
                diag
            };

            let loc = self.source_ref.make_loc(cdiag.begin_pos, cdiag.end_pos);
            let cstr = unsafe { CStr::from_ptr(cdiag.data) }.to_str();
            let data = match cstr {
                Ok(msg) => if msg.len() > 0 { Some(msg.to_owned()) } else { println!("None because zero length"); None },
                Err(_) => { println!("None because error"); None },
            };

            vec.push(Diagnostic {
                error: cdiag.class,
                level: cdiag.level,
                loc: loc,
                data: data,
            });
        }

        vec
    }

    pub fn comments(&self) -> Comments {
        Comments { driver: self }
    }
}
