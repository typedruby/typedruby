pub mod ast;
mod ffi;
mod parser;

use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct SourceFile {
    filename: String,
    source: String,
    line_map: Vec<usize>,
}

pub struct SourceLine {
    pub number: usize,
    pub begin_pos: usize,
    pub end_pos: usize,
}

fn line_map_from_source(source: &str) -> Vec<usize> {
    let mut line_map = vec![];

    let mut previous_index = 0;

    for (index, c) in source.char_indices() {
        if c == '\n' {
            line_map.push(previous_index);
            previous_index = index + 1;
        }
    }

    line_map.push(previous_index);

    line_map
}

impl SourceFile {
    pub fn new(filename: String, source: String) -> SourceFile {
        let line_map = line_map_from_source(&source);

        SourceFile {
            filename: filename,
            source: source,
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

    pub fn line_for_pos(&self, byte_pos: usize) -> SourceLine {
        let idx = match self.line_map.binary_search(&byte_pos) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        };

        SourceLine {
            number: idx + 1,
            begin_pos: self.line_map[idx],
            end_pos: self.line_map[idx + 1],
        }
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
