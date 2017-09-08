mod call;
mod control;
mod eval;
mod locals;
mod materialize;
mod types;

use std::rc::Rc;
use environment::Environment;
use object::{MethodEntry, MethodImpl};
use typed_arena::Arena;
use self::types::TypeEnv;
use self::eval::Eval;

pub fn check<'object>(env: &Environment<'object>, method: Rc<MethodEntry<'object>>) {
    let arena = Arena::new();
    let types = TypeEnv::new(&arena, env);

    match *method.implementation {
        MethodImpl::TypedRuby { ref scope, ref body, ref proto, .. } =>
            Eval::process(env, types, scope.clone(), method.owner, body.clone(), proto),
        MethodImpl::Ruby { .. } |
        MethodImpl::AttrReader { .. } |
        MethodImpl::AttrWriter { .. } |
        MethodImpl::Untyped |
        MethodImpl::IntrinsicClassNew |
        MethodImpl::IntrinsicProcCall |
        MethodImpl::IntrinsicKernelRaise |
        MethodImpl::IntrinsicKernelIsA =>
            { /* pass */ }
    }
}
