mod ffi;
mod ast;
mod parser;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct SourceFile {
    filename: String,
    source: String,
    line_map: Vec<usize>,
}

impl SourceFile {
    pub fn new(filename: String, source: String) -> SourceFile {
        let mut line_map = vec![];

        for (index, c) in source.char_indices() {
            if c == '\n' {
                line_map.push(index);
            }
        }

        SourceFile {
            filename: filename.to_owned(),
            source: source.to_owned(),
            line_map: line_map,
        }
    }

    pub fn open(filename: String) -> io::Result<SourceFile> {
        let mut file = File::open(&filename)?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        Ok(SourceFile::new(filename, source))
    }

    pub fn parse(&self) -> ast::Ast {
        parser::parse(self.filename.as_str(), self.source.as_str())
    }

    pub fn line_for_pos(&self, byte_pos: usize) -> usize {
        match self.line_map.binary_search(&byte_pos) {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        }
    }
}
