extern crate typed_arena;

mod ast;
mod environment;
mod errors;
mod object;
mod top_level;

use typed_arena::Arena;

use environment::Environment;
use errors::ErrorReporter;

use std::io;

fn main() {
    let mut errors = ErrorReporter::new(io::stderr());
    let arena = Arena::new();
    let env = Environment::new(&arena, Box::new(errors));

    for arg in std::env::args().skip(1) {
        match env.load_file(arg) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
