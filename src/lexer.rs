extern crate libc;
use self::libc::{c_int, c_void, size_t};

#[link(name = "c++")]
#[link(name = "ruby-lexer")]
extern {
    fn ruby_lexer_init(version: c_int, source: *const u8, len: size_t) -> *mut c_void;
    fn ruby_lexer_free(state: *mut c_void);
    fn ruby_lexer_env_extend_static(state: *mut c_void);
    fn ruby_lexer_env_extend_dynamic(state: *mut c_void);
    fn ruby_lexer_env_unextend(state: *mut c_void);
    fn ruby_lexer_env_declare(state: *mut c_void, name: *const u8, len: size_t);
    fn ruby_lexer_advance(state: *mut c_void);
}

pub struct Lexer {
    state: *mut c_void,
}

impl Drop for Lexer {
    fn drop(&mut self) {
        unsafe {
            ruby_lexer_free(self.state);
        }
    }
}

pub enum RubyVersion {
    Ruby18 = 18,
    Ruby19 = 19,
    Ruby20 = 20,
    Ruby21 = 21,
    Ruby22 = 22,
    Ruby23 = 23,
    Ruby24 = 24,
}

pub fn new(version: RubyVersion, source: &str) -> Lexer {
    let state = unsafe {
        ruby_lexer_init(version as i32, source.as_bytes().as_ptr(), source.len())
    };
    Lexer { state: state }
}

impl Lexer {
    pub fn extend_static(&mut self) {
        unsafe {
            ruby_lexer_env_extend_static(self.state);
        }
    }

    pub fn extend_dynamic(&mut self) {
        unsafe {
            ruby_lexer_env_extend_dynamic(self.state);
        }
    }

    pub fn unextend(&mut self) {
        unsafe {
            ruby_lexer_env_unextend(self.state);
        }
    }

    pub fn declare(&mut self, name: &str) {
        unsafe {
            ruby_lexer_env_declare(self.state, name.as_bytes().as_ptr(), name.len());
        }
    }

    pub fn advance(&mut self) {
        unsafe {
            ruby_lexer_advance(self.state);
        }
    }
}
