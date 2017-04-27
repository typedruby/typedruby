mod evaluator;
mod types;

use std::rc::Rc;
use environment::Environment;
use object::MethodEntry;
use typed_arena::Arena;
use self::types::Types;

pub fn check<'object, 'env: 'object>(env: &'env Environment<'object>, method: Rc<MethodEntry<'object>>) {
    let arena = Arena::new();
    let types = Types::new(&arena, &env.object);
}
