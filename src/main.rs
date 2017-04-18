mod environment;

use environment::Env;

fn main() {
    let mut env = Env::new();

    let obj_meta = env.metaclass(&env.Object);
    println!("{}", env.name(&obj_meta));
}
