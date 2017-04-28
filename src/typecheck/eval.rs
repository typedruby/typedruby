use std::rc::Rc;
use typecheck::types::TypeEnv;
use object::{Scope, RubyObject};
use ast::Node;

pub struct Eval<'ty, 'env, 'object: 'ty + 'env> {
    tyenv: TypeEnv<'ty, 'env, 'object>,
    scope: Rc<Scope<'object>>,
    class: &'object RubyObject<'object>
}

impl<'ty, 'env, 'object> Eval<'ty, 'env, 'object> {
    pub fn new(tyenv: TypeEnv<'ty, 'env, 'object>, scope: Rc<Scope<'object>>, class: &'object RubyObject<'object>) -> Eval<'ty, 'env, 'object> {
        Eval { tyenv: tyenv, scope: scope, class: class }
    }

    pub fn eval(&self, node: &Node) {
        match *node {
            _ => panic!("unknown node: {:?}", node),
        }
    }
}
