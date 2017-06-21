#![allow(improper_ctypes)]

extern crate libc;

use ::ast::{Node, Loc, SourceFile, Diagnostic, Level};
use ::builder::Builder;
use ::parser::ParserOptions;
use ::diagnostics::Error;
use self::libc::{size_t, c_char};
use std::ffi::{CStr, CString};
use std::vec::Vec;
use std::rc::Rc;
use std::ptr;
use std::slice;
use std::str;
use std::mem;

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

#[inline(always)]
unsafe fn node_from_c(p: *mut Rc<Node>) -> Option<Rc<Node>> {
    if p.is_null() {
        None
    } else {
        Some(*Box::from_raw(p))
    }
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
unsafe fn node_list_from_c(list: *mut NodeListPtr) -> Vec<Rc<Node>> {
    if list.is_null() {
        Vec::new()
    } else {
        let len = rblist_get_length(list);
        let mut vec = Vec::with_capacity(len);

        for index in 0..len {
            let node_ptr = rblist_index(list, index);
            assert!(node_ptr != ptr::null_mut());
            vec.push(*Box::from_raw(node_ptr));
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

include!("ffi_builder.rs");

extern "C" {
    fn rbdriver_typedruby24_new(source: *const u8, source_length: size_t, builder: *const BuilderInterface) -> *mut DriverPtr;
    fn rbdriver_typedruby24_free(driver: *mut DriverPtr);
    fn rbdriver_parse(driver: *mut DriverPtr, builder: *mut Builder) -> *mut Rc<Node>;
    fn rbdriver_in_definition(driver: *const DriverPtr) -> bool;
    fn rbdriver_env_is_declared(driver: *const DriverPtr, name: *const u8, len: size_t) -> bool;
    fn rbdriver_env_declare(driver: *mut DriverPtr, name: *const u8, len: size_t);
    fn rbtoken_get_start(token: *const TokenPtr) -> size_t;
    fn rbtoken_get_end(token: *const TokenPtr) -> size_t;
    fn rbtoken_get_string(token: *const TokenPtr, ptr: *mut *const u8) -> size_t;
    fn rblist_get_length(list: *mut NodeListPtr) -> size_t;
    fn rblist_index(list: *mut NodeListPtr, index: size_t) -> *mut Rc<Node>;
    fn rbdriver_diag_get_length(driver: *const DriverPtr) -> size_t;
    fn rbdriver_diag_get(driver: *const DriverPtr, index: size_t, diag: *mut CDiagnostic);
    fn rbdriver_diag_report(driver: *const DriverPtr, diag: *const CDiagnostic);
}

pub struct Token {
    token: *const TokenPtr,
}

impl Token {
    pub fn location(&self, file: Rc<SourceFile>) -> Loc {
        let begin = unsafe { rbtoken_get_start(self.token) };
        let end = unsafe { rbtoken_get_end(self.token) };

        Loc {
            file: file,
            begin_pos: begin,
            end_pos: end,
        }
    }

    pub fn string(&self) -> String {
        unsafe {
            let mut string: *const u8 = ptr::null();
            let string_length = rbtoken_get_string(self.token, &mut string);
            String::from(str::from_utf8_unchecked(slice::from_raw_parts(string, string_length)))
        }
    }
}

pub struct Driver {
    ptr: *mut DriverPtr,
    pub current_file: Rc<SourceFile>,
}

impl Drop for Driver {
    fn drop(&mut self) {
        unsafe { rbdriver_typedruby24_free(self.ptr); }
    }
}

impl Driver {
    pub fn new(file: Rc<SourceFile>) -> Self {
        let source = file.source();
        let ptr = unsafe { rbdriver_typedruby24_new(source.as_ptr(), source.len(), &CALLBACKS) };
        Driver { ptr: ptr, current_file: file.clone() }
    }

    pub fn parse(&mut self, opt: &ParserOptions) -> Option<Box<Rc<Node>>> {
        for var in opt.declare_env.iter() {
            self.declare(var);
        }

        let driver = self.ptr;
        let mut builder = Box::new(Builder {
            driver: self,
            cookie: 12345678,
            magic_literals: opt.emit_file_vars_as_literals,
            emit_lambda: opt.emit_lambda,
            emit_procarg0: opt.emit_procarg0,
        });
        let ast = unsafe { rbdriver_parse(driver, &mut *builder) };

        if ast.is_null() {
            None
        } else {
            Some(unsafe { Box::from_raw(ast) })
        }
    }

    fn diagnostic(&mut self, level: Level, err: Error, loc: Loc, data: *const c_char) {
        let diag = CDiagnostic {
            level: level,
            class: err,
            data: data,
            begin_pos: loc.begin_pos,
            end_pos: loc.end_pos,
        };

        unsafe {
            rbdriver_diag_report(self.ptr, &diag);
        }
    }

    pub fn error(&mut self, err: Error, loc: Loc) {
        self.diagnostic(Level::Error, err, loc, ptr::null())
    }

    #[allow(dead_code)]
    pub fn error_with_data(&mut self, err: Error, loc: Loc, data: &str) {
        let data = CString::new(data.to_owned()).unwrap();
        self.diagnostic(Level::Error, err, loc, data.as_ptr())
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

            let loc = Loc {
                file: self.current_file.clone(),
                begin_pos: cdiag.begin_pos,
                end_pos: cdiag.end_pos
            };

            let cstr = unsafe { CStr::from_ptr(cdiag.data) }.to_str();
            let data = match cstr {
                Ok(msg) => if msg.len() > 0 { Some(msg.to_owned()) } else { None },
                Err(_) => None,
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
}
