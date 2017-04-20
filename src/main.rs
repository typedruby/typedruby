extern crate ruby_parser;

mod environment;
mod object;
mod top_level;

use environment::Environment;

fn main() {
    Environment::new();
}
