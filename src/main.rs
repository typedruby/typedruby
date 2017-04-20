mod ast;
mod environment;
mod errors;
mod object;
mod top_level;

use environment::Environment;
use errors::ErrorReporter;

use std::env;
use std::io;

fn main() {
    let mut errors = ErrorReporter::new(io::stderr());
    let env = Environment::new(&mut errors);

    for arg in env::args() {
        env.load_file(arg);
    }
}
