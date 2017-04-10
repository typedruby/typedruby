mod ffi;
mod ast;
mod parser;

use std::env;
use std::process;
use std::io;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;

struct SourceFile {
    filename: String,
    source: String,
    line_map: Vec<usize>,
}

impl SourceFile {
    fn open(filename: &str) -> io::Result<SourceFile> {
        let mut file = File::open(filename)?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        let mut line_map = vec![];
        for (index, c) in source.char_indices() {
            if c == '\n' {
                line_map.push(index);
            }
        }

        Ok(SourceFile {
            filename: filename.to_owned(),
            source: source,
            line_map: line_map,
        })
    }

    fn parse(&self) -> ast::Ast {
        parser::parse(self.filename.as_str(), self.source.as_str())
    }

    fn line_for_pos(&self, byte_pos: usize) -> usize {
        match self.line_map.binary_search(&byte_pos) {
            Ok(idx) => idx + 1,
            Err(idx) => idx + 1,
        }
    }
}

fn show_diagnostic<'r>(diagnostic: &'r &ast::Diagnostic) -> bool {
    match diagnostic.level {
        ast::DiagnosticLevel::Error |
        ast::DiagnosticLevel::Fatal => true,
        _ => false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        writeln!(&mut std::io::stderr(), "usage: {} <source files...>", args[0]).unwrap();
        process::exit(1);
    }

    let mut rc = 0;

    for filename in args.iter().skip(1) {
        println!("\x1b[34;1m{}\x1b[0m", filename);

        let file = SourceFile::open(filename).unwrap();

        let result = file.parse();

        for diagnostic in result.diagnostics.iter().filter(show_diagnostic) {
            println!("\x1b[31;1m{}:\x1b[0m {:?}", file.line_for_pos(diagnostic.loc.begin_pos), diagnostic.message);
            rc = 1;
        }

        // println!("{:#?}", result.ast);
    }

    process::exit(rc);
}
