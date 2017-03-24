mod lexer;

#[link(name="c++")]
#[link(name="ruby-lexer")]
extern {
    fn ruby_lexer_init(version: i32);
}

fn main() {
    let mut l = lexer::new(lexer::RubyVersion::Ruby24, "1 + 2");
    println!("{:?}", l.advance());

    unsafe { ruby_lexer_init(24); }
}
