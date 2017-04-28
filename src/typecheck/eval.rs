use std::rc::Rc;
use typecheck::types::{TypeEnv, Type};
use object::{Scope, RubyObject};
use ast::{Node, Loc, Id};
use environment::Environment;

pub struct Eval<'ty, 'env, 'object: 'ty + 'env> {
    env: &'env Environment<'object>,
    tyenv: TypeEnv<'ty, 'env, 'object>,
    scope: Rc<Scope<'object>>,
    class: &'object RubyObject<'object>,
}

enum Locals<'ty, 'object: 'ty> {
    None,
    Var {
        parent: Rc<Locals<'ty, 'object>>,
        name: String,
        ty: &'ty Type<'ty, 'object>,
    },
}

enum Computation<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
    Return(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
    Divergent(Rc<Computation<'ty, 'object>>, Rc<Computation<'ty, 'object>>),
}

impl<'ty, 'env, 'object> Eval<'ty, 'env, 'object> {
    pub fn new(env: &'env Environment<'object>, tyenv: TypeEnv<'ty, 'env, 'object>, scope: Rc<Scope<'object>>, class: &'object RubyObject<'object>) -> Eval<'ty, 'env, 'object> {
        Eval { env: env, tyenv: tyenv, scope: scope, class: class }
    }

    fn error(&self, message: &str, details: &[(&str, &Loc)]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn warning(&self, message: &str, details: &[(&str, &Loc)]) {
        self.env.error_sink.borrow_mut().warning(message, details)
    }

    pub fn process_def(&self, node: &Node) {
        let (prototype_node, body) = match *node {
            // just ignore method definitions that have no args or prototype:
            Node::Def(_, _, None, _) => return,
            Node::Defs(_, _, _, None, _) => return,

            Node::Def(_, _, Some(ref proto), ref body) =>
                (proto, body),
            Node::Defs(_, _, _, Some(ref proto), ref body) =>
                (proto, body),
            _ =>
                panic!("unknown node: {:?}", node),
        };

        let self_type = self.tyenv.alloc(Type::Instance {
            // TODO - this just takes the entire method as the location of the self type. make this a bit less bruteforce.
            loc: node.loc().clone(),
            class: self.class,
            type_parameters: self.class.type_parameters().iter().map(|&Id(ref loc, ref name)|
                self.tyenv.alloc(Type::TypeParameter {
                    loc: loc.clone(),
                    name: name.clone(),
                })
            ).collect(),
        });

        let (prototype, locals) = self.parse_prototype(prototype_node, Rc::new(Locals::None), self_type, self.scope.clone());

        // don't typecheck a method if it has no body
        if let Some(ref body_node) = *body {
            println!("body: {:?}", body_node);
        }
    }

    fn parse_type(&self, node: &Node, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match *node {
            Node::TyCpath(_, ref cpath) =>
                match self.env.resolve_cpath(node, scope) {
                    Ok(class) =>
                        match *class {
                            RubyObject::Object { .. } => {
                                self.error("Constant mentioned in type name does not reference class/module", &[
                                    ("here", cpath.loc()),
                                ]);

                                self.tyenv.any(node.loc().clone())
                            },
                            RubyObject::Module { .. } |
                            RubyObject::Metaclass { .. } =>
                                self.tyenv.alloc(Type::Instance { loc: node.loc().clone(), class: class, type_parameters: Vec::new() }),
                            RubyObject::Class { ref type_parameters, .. } =>
                                if type_parameters.is_empty() {
                                    self.tyenv.alloc(Type::Instance { loc: node.loc().clone(), class: class, type_parameters: Vec::new() })
                                } else {
                                    self.error("Type referenced is generic but no type parameters were supplied", &[
                                        ("here", cpath.loc()),
                                    ]);

                                    self.tyenv.any(node.loc().clone())
                                },
                            RubyObject::IClass { .. } => panic!("unexpected iclass"),
                        },
                    Err((node, message)) => {
                        self.error(message, &[
                            ("here", node.loc()),
                        ]);

                        self.tyenv.any(node.loc().clone())
                    }
                },
            _ => panic!("unknown type node: {:?}", node),
        }
    }

    fn parse_prototype(&self, node: &Node, locals: Rc<Locals<'ty, 'object>>, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>)
    {
        let (args_node, return_type) = match *node {
            Node::Prototype(_, ref gendecl, ref args, ref ret) =>
                (args, match *ret {
                    Some(ref ret_node) => self.parse_type(ret_node, self_type, scope.clone()),
                    None => self.tyenv.any(node.loc().clone()),
                }),
            Node::Args(..) => panic!(),
            _ => panic!("expected Args or Prototype node"),
        };

        panic!();
    }
}
