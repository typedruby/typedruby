use std::rc::Rc;
use typecheck::types::{Arg, TypeEnv, Type};
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

fn assign<'ty, 'object: 'ty>(locals: Rc<Locals<'ty, 'object>>, name: String, ty: &'ty Type<'ty, 'object>) -> Rc<Locals<'ty, 'object>> {
    Rc::new(Locals::Var {
        parent: locals,
        name: name,
        ty: ty,
    })
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

        let (prototype, locals) = self.resolve_prototype(prototype_node, Rc::new(Locals::None), self_type, self.scope.clone());

        // don't typecheck a method if it has no body
        if let Some(ref body_node) = *body {
            println!("body: {:?}", body_node);
        }
    }

    fn create_instance_type(&self, loc: &Loc, class: &'object RubyObject<'object>, type_parameters: Vec<&'ty Type<'ty, 'object>>) -> &'ty Type<'ty, 'object> {
        let supplied_params = type_parameters.len();
        let expected_params = class.type_parameters().len();

        if supplied_params == expected_params {
            self.tyenv.alloc(Type::Instance { loc: loc.clone(), class: class, type_parameters: type_parameters })
        } else {
            if supplied_params == 0 {
                self.error("Type referenced is generic but no type parameters were supplied", &[
                    ("here", loc),
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
                    (&message, loc),
                ]);
            }

            self.tyenv.any(loc.clone())
        }
    }

    fn resolve_instance_type(&self, loc: &Loc, cpath: &Node, type_parameters: Vec<&'ty Type<'ty, 'object>>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match self.env.resolve_cpath(cpath, scope) {
            Ok(class) => match *class {
                RubyObject::Object { .. } => {
                    self.error("Constant mentioned in type name does not reference class/module", &[
                        ("here", cpath.loc()),
                    ]);

                    self.tyenv.any(cpath.loc().clone())
                },
                RubyObject::Module { .. } |
                RubyObject::Metaclass { .. } |
                RubyObject::Class { .. } =>
                    self.create_instance_type(loc, class, type_parameters),
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

    fn create_array_type(&self, loc: &Loc, element_type: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        let array_class = self.env.object.get_const(self.env.object.Object, "Array").expect("expected Array to be defined");
        self.create_instance_type(loc, array_class, vec![element_type])
    }

    fn resolve_type(&self, node: &Node, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match *node {
            Node::TyCpath(ref loc, ref cpath) =>
                self.resolve_instance_type(loc, cpath, Vec::new(), scope),
            Node::TyGeninst(ref loc, ref cpath, ref args) => {
                let type_parameters = args.iter().map(|arg| self.resolve_type(arg, self_type, scope.clone())).collect();
                self.resolve_instance_type(loc, cpath, type_parameters, scope)
            },
            Node::TyNil(ref loc) => {
                self.create_instance_type(loc, self.env.object.NilClass, Vec::new())
            },
            Node::TyAny(ref loc) => {
                self.tyenv.any(loc.clone())
            },
            Node::TyArray(ref loc, ref element) => {
                self.create_array_type(loc, self.resolve_type(element, self_type, scope))
            },
            Node::TyProc(ref loc, ref prototype) => {
                self.resolve_prototype(prototype, Rc::new(Locals::None), self_type, scope).0
            },
            Node::TyClass(ref loc) => {
                let self_class = match *self_type {
                    Type::Instance { ref class, .. } => class,
                    _ => panic!("unknown self_type in TyClass resolution: {:?}", self_type),
                };

                // metaclasses never have type parameters:
                self.create_instance_type(loc, self.env.object.metaclass(self_class), Vec::new())
            },
            Node::TySelf(ref loc) => {
                self.tyenv.update_loc(self_type, loc.clone())
            },
            Node::TyInstance(ref loc) => {
                let self_class = match *self_type {
                    Type::Instance { class, .. } => class,
                    _ => panic!("unknown self_type in TyInstance resolution: {:?}", self_type),
                };

                match *self_class {
                    RubyObject::Metaclass { of, .. } => {
                        // if the class we're trying to instantiate has type parameters just fill them with new
                        // type variables. TODO revisit this logic and see if there's something better we could do?
                        let type_parameters = of.type_parameters().iter().map(|_| self.tyenv.new_var(loc.clone())).collect();
                        self.create_instance_type(loc, of, type_parameters)
                    },
                    _ => {
                        // special case to allow the Class#allocate definition in the stdlib:
                        if self_class != self.env.object.Class {
                            self.error("Cannot instatiate instance type", &[
                                (&format!("Self here is {}, which is not a Class", self_class.name()), loc),
                            ]);
                        }

                        self.tyenv.any(loc.clone())
                    },
                }
            },
            Node::TyNillable(ref loc, ref type_node) => {
                self.tyenv.alloc(Type::Union {
                    loc: loc.clone(),
                    types: vec![
                        self.create_instance_type(loc, self.env.object.NilClass, Vec::new()),
                        self.resolve_type(type_node, self_type, scope),
                    ],
                })
            },
            Node::TyOr(ref loc, ref a, ref b) => {
                self.tyenv.alloc(Type::Union {
                    loc: loc.clone(),
                    types: vec![
                        self.resolve_type(a, self_type, scope.clone()),
                        self.resolve_type(b, self_type, scope),
                    ],
                })
            }
            _ => panic!("unknown type node: {:?}", node),
        }
    }

    fn resolve_arg(&self, arg_node: &Node, locals: Rc<Locals<'ty, 'object>>, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (Arg<'ty, 'object>, Rc<Locals<'ty, 'object>>)
    {
        let (ty, arg_node) = match *arg_node {
            Node::TypedArg(_, ref type_node, ref arg) => {
                let ty = self.resolve_type(type_node, self_type, scope.clone());
                (Some(ty), &**arg)
            },
            _ => (None, arg_node),
        };

        let ty_or_any = ty.unwrap_or_else(|| self.tyenv.any(arg_node.loc().clone()));

        match *arg_node {
            Node::Arg(ref loc, ref name) =>
                (Arg::Required { loc: loc.clone(), ty: ty }, assign(locals, name.to_owned(), ty_or_any)),
            Node::Blockarg(ref loc, None) =>
                (Arg::Block { loc: loc.clone(), ty: ty }, locals),
            Node::Blockarg(ref loc, Some(Id(_, ref name))) =>
                (Arg::Block { loc: loc.clone(), ty: ty }, assign(locals, name.to_owned(), ty_or_any)),
            Node::Optarg(_, Id(ref loc, ref name), ref expr) =>
                (Arg::Optional { loc: loc.clone(), ty: ty, expr: expr.clone() }, assign(locals, name.to_owned(), ty_or_any)),
            Node::Restarg(ref loc, None) =>
                (Arg::Rest { loc: loc.clone(), ty: ty }, locals),
            Node::Restarg(ref loc, Some(Id(_, ref name))) =>
                (Arg::Rest { loc: loc.clone(), ty: ty }, assign(locals, name.to_owned(), self.create_array_type(loc, ty_or_any))),
            Node::Procarg0(ref loc, ref inner_arg_node) => {
                let (inner_arg, locals) = self.resolve_arg(inner_arg_node, locals, self_type, scope);
                (Arg::Procarg0 { loc: loc.clone(), arg: Box::new(inner_arg) }, locals)
            },
            _ => panic!("arg_node: {:?}", arg_node),
        }
    }

    fn resolve_prototype(&self, node: &Node, locals: Rc<Locals<'ty, 'object>>, self_type: &'ty Type<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>)
    {
        let (args_node, return_type) = match *node {
            Node::Prototype(_, ref gendecl, ref args, ref ret) =>
                (&**args, ret.as_ref().map(|ret_node| self.resolve_type(ret_node, self_type, scope.clone()))),
            Node::Args(..) => (node, None),
            _ => panic!("unexpected {:?}", node),
        };

        let arg_nodes = if let Node::Args(_, ref arg_nodes) = *args_node {
            arg_nodes
        } else {
            panic!("expected args_node to be Node::Args")
        };

        let mut args = Vec::new();
        let mut locals = locals;

        for arg_node in arg_nodes {
            let (arg, locals_) = self.resolve_arg(arg_node, locals, self_type, scope.clone());
            locals = locals_;
        }

        let proc_type = self.tyenv.alloc(Type::Proc {
            loc: node.loc().clone(),
            args: args,
            retn: return_type,
        });

        (proc_type, locals)
    }
}
