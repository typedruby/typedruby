extern crate gcc;

use std::process::Command;

fn main() {
    Command::new("ragel")
        .args(&["-C", "-o", "src/lexer.cc", "src/lexer.rl"])
        .status()
        .unwrap();

    gcc::compile_library("libruby-lexer.a", &["src/lexer.cc"]);
}
