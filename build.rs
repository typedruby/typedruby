extern crate gcc;

// use std::fmt::Write;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::iter::FromIterator;
use std::process::Command;
use std::vec::Vec;

struct Token {
    id: i32,
    name: String,
}

fn read_tokens_def() -> Vec<Token> {
    let reader = BufReader::new(File::open("tokens.def").unwrap());

    Vec::from_iter(reader.lines().enumerate().map(|(index, line)| {
        let name = line.unwrap();

        Token {
            id: index as i32,
            name: name,
        }
    }))
}

fn write_tokens_header(tokens: &Vec<Token>) {
    let mut file = File::create("src/tokens_gen.h").unwrap();

    write!(&mut file, "enum ruby_token_type_t {{\n").unwrap();

    for token in tokens {
        write!(&mut file, "    {} = {},\n", token.name, token.id).unwrap();
    }

    write!(&mut file, "}};\n").unwrap();
}

fn write_tokens_rust(tokens: &Vec<Token>) {
    let mut file = File::create("src/tokens_gen.rs").unwrap();

    for token in tokens {
        write!(&mut file, "pub const {}: i32 = {};\n", token.name, token.id).unwrap();
    }
}

fn build_lexer() {
    Command::new("ragel")
        .args(&["-C", "-o", "src/lexer.cc", "src/lexer.rl"])
        .status()
        .unwrap();

    gcc::compile_library("libruby-lexer.o", &["src/lexer.cc"]);
}

fn main() {
    println!("cargo:rustc-link-lib=static=ruby-lexer");

    let tokens = read_tokens_def();

    // write_tokens_header(&tokens);
    // write_tokens_rust(&tokens);

    build_lexer();
}
