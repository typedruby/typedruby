mod object;

use object::ObjectGraph;

fn main() {
    let mut env = ObjectGraph::new();

    let obj_meta = env.metaclass(&env.Object);
    println!("{}", env.name(&obj_meta));
}
