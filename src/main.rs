mod lexer;

fn main() {
    let mut l = lexer::new(lexer::RubyVersion::Ruby24, "1 + 2");
    l.advance();
}
