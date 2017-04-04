mod ffi;
mod ast;
mod parser;

fn main() {
    println!("{:#?}", parser::parse("foo.rb", "123 + 456"));
}
