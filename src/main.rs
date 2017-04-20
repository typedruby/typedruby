mod ast;
mod environment;
mod object;
mod top_level;

use environment::Environment;

use std::env;

fn main() {
    let env = Environment::new();

    for arg in env::args() {
        env.load_file(arg);
    }
}
