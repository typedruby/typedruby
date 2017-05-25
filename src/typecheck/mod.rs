mod call;
mod eval;
mod flow;
mod types;

use std::rc::Rc;
use environment::Environment;
use object::{MethodEntry, MethodImpl};
use typed_arena::Arena;
use self::types::TypeEnv;
use self::eval::Eval;

pub fn check<'env, 'object: 'env>(env: &'env Environment<'object>, method: Rc<MethodEntry<'object>>) {
    let arena = Arena::new();
    let types = TypeEnv::new(&arena, &env.object);

    match method.implementation {
        MethodImpl::Ruby { ref scope, ref node, ref owner, .. } =>
            Eval::new(env, types, scope.clone(), owner, node.clone()).process(),
        MethodImpl::AttrReader { .. } |
        MethodImpl::AttrWriter { .. } |
        MethodImpl::Untyped |
        MethodImpl::IntrinsicClassNew =>
            { /* pass */ }
    }
}
