mod ffi;
mod ast;
mod parser;

use std::env;
use std::process;
use std::io::Write;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        writeln!(&mut std::io::stderr(), "usage: {} <source files...>", args[0]).unwrap();
        process::exit(1);
    }

    let mut rc = 0;

    for filename in args.iter().skip(1) {
        let result = File::open(filename).and_then(|mut file| {
            let mut source = String::new();
            file.read_to_string(&mut source).map(|_| source)
        });

        match result {
            Ok(source) => { parser::parse(filename.as_str(), source.as_str()); }
            Err(e) => {
                rc = 1;
                writeln!(&mut std::io::stderr(), "{}: {}", filename, e).unwrap();
            }
        };
    }

    process::exit(rc);
}
