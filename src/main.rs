mod ffi;
mod ast;
mod parser;

fn main() {
    println!("{:?}", parser::parse("123 + 456"));
}
