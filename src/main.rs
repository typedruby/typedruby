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
        println!("\x1b[34;1m{}\x1b[0m", filename);

        let result = File::open(filename).and_then(|mut file| {
            let mut source = String::new();
            file.read_to_string(&mut source).map(|_| source)
        });

        match result {
            Ok(source) => {
                let result = parser::parse(filename.as_str(), source.as_str());
                println!("{:?}", result);
            }
            Err(e) => {
                rc = 1;
                println!("{:?}", e);
            }
        };
    }

    process::exit(rc);
}
