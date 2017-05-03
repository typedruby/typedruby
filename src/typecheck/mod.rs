mod eval;
mod flow;
mod types;

use std::rc::Rc;
use environment::Environment;
use object::MethodEntry;
use typed_arena::Arena;
use self::types::TypeEnv;
use self::eval::Eval;

pub fn check<'env, 'object: 'env>(env: &'env Environment<'object>, method: Rc<MethodEntry<'object>>) {
    let arena = Arena::new();
    let types = TypeEnv::new(&arena, &env.object);

    match *method {
        MethodEntry::Ruby { ref scope, ref node, ref owner, .. } =>
            Eval::new(env, types, scope.clone(), owner, node.clone()).process(),
        MethodEntry::Untyped => {
            // pass
        }
    }
}
