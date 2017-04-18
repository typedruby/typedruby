mod object;

use object::ObjectGraph;

fn main() {
    let mut object = ObjectGraph::new();

    let mut c = object.metaclass(&object.Class);

    loop {
        println!("{}", object.name(&c));

        match object.superclass(&c) {
            Some(s) => c = s,
            None => break,
        };
    }
}
