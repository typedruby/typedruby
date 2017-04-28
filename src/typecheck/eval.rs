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

    fn create_instance_type(&self, loc: &Loc, cpath: &Node, type_parameters: Vec<&'ty Type<'ty, 'object>>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match self.env.resolve_cpath(cpath, scope) {
            Ok(class) =>
                match *class {
                    RubyObject::Object { .. } => {
                        self.error("Constant mentioned in type name does not reference class/module", &[
                            ("here", cpath.loc()),
                        ]);

                        self.tyenv.any(cpath.loc().clone())
                    },
                    RubyObject::Module { .. } |
                    RubyObject::Metaclass { .. } |
                    RubyObject::Class { .. } => {
                        let supplied_params = type_parameters.len();
                        let expected_params = class.type_parameters().len();

                        if supplied_params == expected_params {
                            self.tyenv.alloc(Type::Instance { loc: loc.clone(), class: class, type_parameters: type_parameters })
                        } else {
                            if supplied_params == 0 {
                                self.error("Type referenced is generic but no type parameters were supplied", &[
                                    ("here", cpath.loc()),
                                ]);
                            } else if supplied_params < expected_params {
                                let mut message = format!("{} also expects ", class.name());

                                for (i, &Id(_, ref name)) in class.type_parameters().iter().skip(supplied_params).enumerate() {
                                    if i > 0 {
                                        message += ", ";
                                    }

                                    message += name;
                                }

                                self.error("Too few type parameters supplied in instantiation of generic type", &[
                                    (&message, cpath.loc()),
                                ]);
                            }

                            self.tyenv.any(cpath.loc().clone())
                        }
                    },
                    RubyObject::IClass { .. } => panic!("unexpected iclass"),
                },
            Err((err_node, message)) => {
                self.error(message, &[
                    ("here", err_node.loc()),
                ]);

                self.tyenv.any(cpath.loc().clone())
            }
        }
    }

    fn parse_type(&self, node: &Node, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match *node {
            Node::TyCpath(_, ref cpath) =>
                self.create_instance_type(node.loc(), cpath, Vec::new(), scope),
            Node::TyGeninst(_, ref cpath, ref args) => {
                let type_parameters = args.iter().map(|arg| self.parse_type(arg, self_type, scope.clone())).collect();
                self.create_instance_type(node.loc(), cpath, type_parameters, scope)
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
