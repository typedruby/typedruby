#[macro_use]
extern crate gc_derive;
extern crate gc;

mod environment;

use environment::Env;

fn main() {
    let env = Env::new();

    let obj_meta = env.metaclass(env.object.clone());
    println!("{}", obj_meta.borrow().name());
}
