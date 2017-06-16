extern crate gcc;

fn main() {
    gcc::Config::new()
		.cpp(true)
		.flag("-std=c++14")
		.flag("-Wall")
		.flag("-Wextra")
		.flag("-Wpedantic")
		.include("include")
		.file("cc/capi.cc")
		.file("cc/lexer.cc")
		.file("cc/literal.cc")
		.file("cc/driver.cc")
		.file("cc/state_stack.cc")
		.file("cc/token.cc")
		.file("cc/grammars/typedruby24.cc")
		.compile("librubyparser.a")
}
