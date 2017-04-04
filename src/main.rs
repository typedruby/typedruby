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
        writeln!(&mut std::io::stderr(), "usage: {} <source file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let mut source = String::new();

    File::open(filename).unwrap()
        .read_to_string(&mut source).unwrap();

    println!("{:#?}", parser::parse(filename, source.as_str()));
}
