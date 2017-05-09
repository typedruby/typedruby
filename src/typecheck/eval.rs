use std::rc::Rc;
use std::collections::HashMap;
use typecheck::flow::{Computation, Locals};
use typecheck::types::{Arg, TypeEnv, Type, Prototype};
use object::{Scope, RubyObject, MethodEntry};
use ast::{Node, Loc, Id};
use environment::Environment;
use errors::Detail;
use typed_arena::Arena;
use or::Or;

pub struct Eval<'ty, 'env, 'object: 'ty + 'env> {
    env: &'env Environment<'object>,
    tyenv: TypeEnv<'ty, 'env, 'object>,
    scope: Rc<Scope<'object>>,
    type_context: TypeContext<'ty, 'object>,
    node: Rc<Node>,
}

#[derive(Clone)]
pub struct TypeContext<'ty, 'object: 'ty> {
    class: &'object RubyObject<'object>,
    type_parameters: Vec<&'ty Type<'ty, 'object>>,
    type_names: HashMap<String, &'ty Type<'ty, 'object>>,
}

impl<'ty, 'object> TypeContext<'ty, 'object> {
    fn new(class: &'object RubyObject<'object>, type_parameters: Vec<&'ty Type<'ty, 'object>>) -> TypeContext<'ty, 'object> {
        let type_names =
            class.type_parameters().iter()
                .map(|&Id(_, ref name)| name.clone())
                .zip(type_parameters.iter().cloned())
                .collect();

        TypeContext {
            class: class,
            type_parameters: type_parameters,
            type_names: type_names,
        }
    }

    pub fn self_type<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>, loc: Loc) -> &'ty Type<'ty, 'object> {
        tyenv.instance(loc, self.class, self.type_parameters.clone())
    }
}

impl<'ty, 'env, 'object> Eval<'ty, 'env, 'object> {
    pub fn new(env: &'env Environment<'object>, tyenv: TypeEnv<'ty, 'env, 'object>, scope: Rc<Scope<'object>>, class: &'object RubyObject<'object>, node: Rc<Node>) -> Eval<'ty, 'env, 'object> {
        let type_parameters = class.type_parameters().iter().map(|&Id(ref loc, ref name)|
            tyenv.alloc(Type::TypeParameter {
                loc: loc.clone(),
                name: name.clone(),
            })
        ).collect();

        let type_context = TypeContext::new(class, type_parameters);

        Eval { env: env, tyenv: tyenv, scope: scope, type_context: type_context, node: node }
    }

    fn error(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn warning(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().warning(message, details)
    }

    pub fn process(&self) {
        let (prototype_node, body) = match *self.node {
            // just ignore method definitions that have no args or prototype:
            Node::Def(_, _, None, _) => return,
            Node::Defs(_, _, _, None, _) => return,

            Node::Def(_, _, Some(ref proto), ref body) =>
                (proto, body),
            Node::Defs(_, _, _, Some(ref proto), ref body) =>
                (proto, body),
            _ =>
                panic!("unknown node: {:?}", self.node),
        };

        let (prototype, locals) = self.resolve_prototype(prototype_node, Locals::new(), &self.type_context, self.scope.clone());

        // don't typecheck a method if it has no body
        if let Some(ref body_node) = *body {
            self.process_node(body_node, locals).terminate(&|ty|
                if let Some(retn) = prototype.retn {
                    self.compatible(retn, ty, None)
                }
            );
        }
    }

    fn create_instance_type(&self, loc: &Loc, class: &'object RubyObject<'object>, type_parameters: Vec<&'ty Type<'ty, 'object>>) -> &'ty Type<'ty, 'object> {
        let supplied_params = type_parameters.len();
        let expected_params = class.type_parameters().len();

        if supplied_params == expected_params {
            self.tyenv.instance(loc.clone(), class, type_parameters)
        } else {
            if supplied_params == 0 {
                self.error("Type referenced is generic but no type parameters were supplied", &[
                    Detail::Loc("here", loc),
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
                    Detail::Loc(&message, loc),
                ]);
            }

            self.tyenv.any(loc.clone())
        }
    }

    fn resolve_instance_type(&self, loc: &Loc, cpath: &Node, type_parameters: Vec<&'ty Type<'ty, 'object>>, context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        if let Node::Const(_, None, Id(ref name_loc, ref name)) = *cpath {
            if let Some(ty) = context.type_names.get(name) {
                if !type_parameters.is_empty() {
                    self.error("Type parameters were supplied but type mentioned does not take any", &[
                        Detail::Loc("here", name_loc),
                    ]);
                }

                return self.tyenv.update_loc(ty, name_loc.clone());
            }
        }

        match self.env.resolve_cpath(cpath, scope) {
            Ok(class) => match *class {
                RubyObject::Object { .. } => {
                    self.error("Constant mentioned in type name does not reference class/module", &[
                        Detail::Loc("here", cpath.loc()),
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
                    Detail::Loc("here", err_node.loc()),
                ]);

                self.tyenv.any(cpath.loc().clone())
            }
        }
    }

    fn create_array_type(&self, loc: &Loc, element_type: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        let array_class = self.env.object.get_const(self.env.object.Object, "Array").expect("expected Array to be defined");
        self.tyenv.instance(loc.clone(), array_class, vec![element_type])
    }

    fn resolve_type(&self, node: &Node, context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>) -> &'ty Type<'ty, 'object> {
        match *node {
            Node::TyCpath(ref loc, ref cpath) =>
                self.resolve_instance_type(loc, cpath, Vec::new(), context, scope),
            Node::TyGeninst(ref loc, ref cpath, ref args) => {
                let type_parameters = args.iter().map(|arg| self.resolve_type(arg, context, scope.clone())).collect();
                self.resolve_instance_type(loc, cpath, type_parameters, context, scope)
            },
            Node::TyNil(ref loc) => {
                self.create_instance_type(loc, self.env.object.NilClass, Vec::new())
            },
            Node::TyAny(ref loc) => {
                self.tyenv.any(loc.clone())
            },
            Node::TyArray(ref loc, ref element) => {
                self.create_array_type(loc, self.resolve_type(element, context, scope))
            },
            Node::TyProc(ref loc, ref prototype) => {
                self.tyenv.alloc(Type::Proc {
                    loc: loc.clone(),
                    proto: self.resolve_prototype(prototype, Locals::new(), context, scope).0,
                })
            },
            Node::TyClass(ref loc) => {
                // metaclasses never have type parameters:
                self.create_instance_type(loc, self.env.object.metaclass(context.class), Vec::new())
            },
            Node::TySelf(ref loc) => {
                context.self_type(&self.tyenv, loc.clone())
            },
            Node::TyInstance(ref loc) => {
                match *context.class {
                    RubyObject::Metaclass { of, .. } => {
                        // if the class we're trying to instantiate has type parameters just fill them with new
                        // type variables. TODO revisit this logic and see if there's something better we could do?
                        let type_parameters = of.type_parameters().iter().map(|_| self.tyenv.new_var(loc.clone())).collect();
                        self.create_instance_type(loc, of, type_parameters)
                    },
                    _ => {
                        // special case to allow the Class#allocate definition in the stdlib:
                        if context.class != self.env.object.Class {
                            self.error("Cannot instatiate instance type", &[
                                Detail::Loc(&format!("Self here is {}, which is not a Class", context.class.name()), loc),
                            ]);
                        }

                        self.tyenv.any(loc.clone())
                    },
                }
            },
            Node::TyNillable(ref loc, ref type_node) => {
                self.tyenv.union(loc,
                    self.tyenv.nil(loc.clone()),
                    self.resolve_type(type_node, context, scope))
            },
            Node::TyOr(ref loc, ref a, ref b) => {
                self.tyenv.union(loc,
                    self.resolve_type(a, context, scope.clone()),
                    self.resolve_type(b, context, scope))
            }
            _ => panic!("unknown type node: {:?}", node),
        }
    }

    fn resolve_arg(&self, arg_node: &Node, locals: Locals<'ty, 'object>, context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (Arg<'ty, 'object>, Locals<'ty, 'object>)
    {
        let (ty, arg_node) = match *arg_node {
            Node::TypedArg(_, ref type_node, ref arg) => {
                let ty = self.resolve_type(type_node, context, scope.clone());
                (Some(ty), &**arg)
            },
            _ => (None, arg_node),
        };

        let ty_or_any = ty.unwrap_or_else(|| self.tyenv.any(arg_node.loc().clone()));

        match *arg_node {
            Node::Arg(ref loc, ref name) =>
                (Arg::Required { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), ty_or_any)),
            Node::Blockarg(ref loc, None) =>
                (Arg::Block { loc: loc.clone(), ty: ty }, locals),
            Node::Blockarg(ref loc, Some(Id(_, ref name))) =>
                (Arg::Block { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), ty_or_any)),
            Node::Optarg(_, Id(ref loc, ref name), ref expr) =>
                (Arg::Optional { loc: loc.clone(), ty: ty, expr: expr.clone() }, locals.assign_shadow(name.to_owned(), ty_or_any)),
            Node::Restarg(ref loc, None) =>
                (Arg::Rest { loc: loc.clone(), ty: ty }, locals),
            Node::Restarg(ref loc, Some(Id(_, ref name))) =>
                (Arg::Rest { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), self.create_array_type(loc, ty_or_any))),
            Node::Procarg0(ref loc, ref inner_arg_node) => {
                let (inner_arg, locals) = self.resolve_arg(inner_arg_node, locals, context, scope);
                (Arg::Procarg0 { loc: loc.clone(), arg: Box::new(inner_arg) }, locals)
            },
            _ => panic!("arg_node: {:?}", arg_node),
        }
    }

    fn resolve_prototype(&self, node: &Node, locals: Locals<'ty, 'object>, context_: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (Rc<Prototype<'ty, 'object>>, Locals<'ty, 'object>)
    {
        let mut context = context_.clone();

        let (args_node, return_type) = match *node {
            Node::Prototype(_, ref genargs, ref args, ref ret) => {
                if let Some(ref genargs_) = *genargs {
                    if let Node::TyGenargs(_, ref gendeclargs) = **genargs_ {
                        for gendeclarg in gendeclargs {
                            if let Node::TyGendeclarg(ref loc, ref name) = **gendeclarg {
                                context.type_names.insert(name.clone(), self.tyenv.new_var(loc.clone()));
                            }
                        }
                    }
                }

                (&**args, ret.as_ref().map(|ret_node| self.resolve_type(ret_node, &context, scope.clone())))
            },
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
            let (arg, locals_) = self.resolve_arg(arg_node, locals, &context, scope.clone());
            args.push(arg);
            locals = locals_;
        }

        let prototype = Prototype { args: args, retn: return_type };

        (Rc::new(prototype), locals)
    }

    fn type_error(&self, a: &'ty Type<'ty, 'object>, b: &'ty Type<'ty, 'object>, err_a: &'ty Type<'ty, 'object>, err_b: &'ty Type<'ty, 'object>, loc: Option<&Loc>) {
        let strs = Arena::new();

        let mut details = vec![
            Detail::Loc(strs.alloc(self.tyenv.describe(err_a) + ", with:"), err_a.loc()),
            Detail::Loc(strs.alloc(self.tyenv.describe(err_b)), err_b.loc()),
        ];

        if !err_a.ref_eq(a) || !err_b.ref_eq(b) {
            details.push(Detail::Message("arising from an attempt to match:"));
            details.push(Detail::Loc(strs.alloc(self.tyenv.describe(a) + ", with:"), a.loc()));
            details.push(Detail::Loc(strs.alloc(self.tyenv.describe(b)), b.loc()));
        }

        if let Some(loc) = loc {
            details.push(Detail::Loc("in this expression", loc));
        }

        self.error("Could not match types:", &details);
    }

    fn unify(&self, a: &'ty Type<'ty, 'object>, b: &'ty Type<'ty, 'object>, loc: Option<&Loc>) {
        if let Err((err_a, err_b)) = self.tyenv.unify(a, b) {
            self.type_error(a, b, err_a, err_b, loc);
        }
    }

    fn compatible(&self, to: &'ty Type<'ty, 'object>, from: &'ty Type<'ty, 'object>, loc: Option<&Loc>) {
        if let Err((err_to, err_from)) = self.tyenv.compatible(to, from) {
            self.type_error(to, from, err_to, err_from, loc);
        }
    }

    fn process_array_tuple(&self, exprs: &[Rc<Node>], locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        let _ = exprs;
        let _ = locals;
        panic!("unimplemented")
    }

    fn prototype_from_method_entry(&self, loc: &Loc, method: &MethodEntry<'object>, type_context: TypeContext<'ty, 'object>) -> Rc<Prototype<'ty, 'object>> {
        match *method {
            MethodEntry::Ruby { ref node, ref scope, .. } => {
                let prototype_node = match **node {
                    Node::Def(_, _, None, _) => return self.tyenv.any_prototype(loc.clone()),
                    Node::Defs(_, _, _, None, _) => return self.tyenv.any_prototype(loc.clone()),
                    Node::Def(_, _, Some(ref proto), _) => proto,
                    Node::Defs(_, _, _, Some(ref proto), _) => proto,
                    _ => panic!("unexpected node in MethodEntry::Ruby: {:?}", node),
                };

                self.resolve_prototype(&prototype_node, Locals::new(), &type_context, scope.clone()).0
            }
            MethodEntry::Untyped => self.tyenv.any_prototype(loc.clone()),
        }
    }

    fn prototypes_for_invocation(&self, recv_loc: &Loc, recv_type: &'ty Type<'ty, 'object>, id: &Id) -> Vec<Rc<Prototype<'ty, 'object>>> {
        let degraded_recv_type = self.tyenv.degrade_to_instance(recv_type);

        match *degraded_recv_type {
            Type::Instance { class, type_parameters: ref tp, .. } => {
                match self.env.object.lookup_method(class, &id.1) {
                    Some(method) => vec![self.prototype_from_method_entry(&id.0, &method, TypeContext::new(class, tp.clone()))],
                    None => Vec::new(),
                }
            }
            Type::Proc { .. } => {
                match self.env.object.lookup_method(self.env.object.Proc, &id.1) {
                    Some(method) => vec![self.prototype_from_method_entry(&id.0, &method, TypeContext::new(&self.env.object.Proc, Vec::new()))],
                    None => Vec::new(),
                }
            }
            Type::Union { ref types, .. } => {
                types.iter().flat_map(|ty| {
                    let prototypes = self.prototypes_for_invocation(recv_loc, ty, id);

                    if prototypes.is_empty() {
                        let message = format!("Union member {} does not respond to #{}", self.tyenv.describe(ty), &id.1);
                        self.error(&message, &[
                            Detail::Loc(&self.tyenv.describe(recv_type), recv_loc),
                        ]);
                    }

                    prototypes
                }).collect()
            }
            Type::Any { .. } => vec![self.tyenv.any_prototype(id.0.clone())],
            Type::TypeParameter { ref name, .. } => {
                self.error(&format!("Type parameter {} is of unknown type", name), &[
                    Detail::Loc("in invocation", recv_loc),
                ]);

                vec![]
            }
            Type::Var { id, .. } => {
                self.error(&format!("Type of receiver is not known at this point"), &[
                    Detail::Loc(&format!("t{}", id), recv_loc),
                ]);

                vec![]
            }
            Type::KeywordHash { .. } => panic!("should have degraded to instance"),
            Type::Tuple { .. } => panic!("should have degraded to instance"),
        }
    }

    fn process_send(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, args: &[Rc<Node>], block: Option<(&Node, &Option<Rc<Node>>)>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        fn merge_maybe_comps<'ty, 'object: 'ty>(a: Option<Computation<'ty, 'object>>, b: Option<Computation<'ty, 'object>>) -> Option<Computation<'ty, 'object>> {
            match (a, b) {
                (Some(a), Some(b)) => Some(Computation::divergent(a, b)),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        }

        let (recv_type, locals, non_result_comp) = match *recv {
            Some(ref recv_node) => match self.process_node(recv_node, locals).extract_results(recv_node.loc(), &self.tyenv) {
                Or::Left((recv_ty, locals)) => (recv_ty, locals, None),
                Or::Both((recv_ty, locals), comp) => (recv_ty, locals, Some(comp)),
                Or::Right(comp) => {
                    self.warning("Useless method call", &[
                        Detail::Loc("here", &id.0),
                        Detail::Loc("receiver never evaluates to a result", recv_node.loc()),
                    ]);
                    return comp;
                }
            },
            None => (self.type_context.self_type(&self.tyenv, id.0.clone()), locals, None),
        };

        let mut arg_types = Vec::new();
        let mut locals = locals;
        let mut non_result_comp = non_result_comp;

        for arg_node in args {
            match self.process_node(arg_node, locals).extract_results(arg_node.loc(), &self.tyenv) {
                Or::Left((arg_ty, l)) => {
                    arg_types.push(arg_ty);
                    locals = l;
                }
                Or::Both((arg_ty, l), comp) => {
                    arg_types.push(arg_ty);
                    locals = l;
                    non_result_comp = merge_maybe_comps(non_result_comp, Some(comp));
                }
                Or::Right(comp) => {
                    self.warning("Useless method call", &[
                        Detail::Loc("here", &id.0),
                        Detail::Loc("argument never evaluates to a result", arg_node.loc()),
                    ]);
                    return merge_maybe_comps(non_result_comp, Some(comp)).expect("never None");
                }
            }
        }

        let prototypes = self.prototypes_for_invocation(recv_type.loc(), recv_type, id);

        panic!("unimplemented")
    }

    fn seq_process(&self, comp: Computation<'ty, 'object>, node: &Node) -> Computation<'ty, 'object> {
        comp.seq(&|_, locals| self.process_node(node, locals))
    }

    fn process_node(&self, node: &Node, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        match *node {
            Node::Array(ref loc, ref elements) => {
                let element_ty = self.tyenv.new_var(loc.clone());
                let array_ty = self.create_array_type(loc, element_ty);
                let comp = Computation::result(array_ty, locals);

                elements.iter().fold(comp, |comp, element_node|
                    self.seq_process(comp, element_node).seq(&|ty, l| {
                        self.unify(element_ty, ty, Some(loc));
                        Computation::result(array_ty, l)
                    })
                )
            }
            Node::Begin(ref loc, ref nodes) => {
                let comp = Computation::result(self.tyenv.nil(loc.clone()), locals);

                nodes.iter().fold(comp, |comp, node|
                    self.seq_process(comp, node).converge_results(node.loc(), &self.tyenv)
                )
            }
            Node::Kwbegin(ref loc, ref node) => {
                match *node {
                    Some(ref n) => self.process_node(n, locals),
                    None => Computation::result(self.tyenv.nil(loc.clone()), locals),
                }
            }
            Node::Lvasgn(ref asgn_loc, Id(_, ref lvar_name), ref expr) => {
                self.process_node(expr, locals).seq(&|expr_ty, l| {
                    let l = match l.assign(lvar_name.to_owned(), expr_ty) {
                        // in the none case, the assignment happened
                        // successfully and the local variable entry is now set
                        // to the type we passed in:
                        (None, l) => l,
                        // in the some case, the local variable is already
                        // pinned to a type and we must check type compatibility:
                        (Some(lvar_ty), l) => {
                            self.compatible(lvar_ty, expr_ty, Some(asgn_loc));
                            l
                        }
                    };

                    Computation::result(expr_ty, l)
                })
            }
            Node::Lvar(ref loc, ref name) => {
                let (ty, locals) = locals.lookup(name);

                let ty = match ty {
                    Some(ty) => self.tyenv.update_loc(ty, loc.clone()),
                    None => {
                        self.error("Use of uninitialised local variable", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.any(loc.clone())
                    },
                };

                Computation::result(ty, locals)
            }
            Node::Ivar(ref loc, ref name) => {
                let ty = match self.env.object.lookup_ivar(self.type_context.class, name) {
                    Some(ivar) => {
                        self.resolve_type(&ivar.type_node, &self.type_context, ivar.scope.clone())
                    },
                    None => {
                        self.error("Use of undeclared instance variable", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.any(loc.clone())
                    },
                };

                Computation::result(ty, locals)
            }
            Node::Ivasgn(ref loc, Id(_, ref iv_name), ref expr) => {
                let ivar_ty = match self.env.object.lookup_ivar(self.type_context.class, iv_name) {
                    Some(ivar) => {
                        self.resolve_type(&ivar.type_node, &self.type_context, ivar.scope.clone())
                    },
                    None => {
                        self.error("Use of undeclared instance variable", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.any(loc.clone())
                    },
                };

                self.process_node(expr, locals).seq(&|ty, l| {
                    self.compatible(ivar_ty, ty, Some(loc));
                    Computation::result(ty, l)
                })
            }
            Node::Integer(ref loc, _) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.Integer), locals)
            }
            Node::String(ref loc, _) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.String), locals)
            }
            Node::Nil(ref loc) => {
                Computation::result(self.tyenv.nil(loc.clone()), locals)
            }
            Node::True(ref loc) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.TrueClass), locals)
            }
            Node::False(ref loc) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.FalseClass), locals)
            }
            Node::Self_(ref loc) => {
                Computation::result(self.tyenv.update_loc(self.type_context.self_type(&self.tyenv, loc.clone()), loc.clone()), locals)
            }
            Node::Symbol(ref loc, _) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.Symbol), locals)
            }
            Node::Float(ref loc, _) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.Float), locals)
            }
            Node::Return(ref loc, ref exprs) => {
                let comp = match exprs.len() {
                    0 => Computation::result(self.tyenv.nil(loc.clone()), locals),
                    1 => self.process_node(exprs.first().unwrap(), locals),
                    _ => self.process_array_tuple(exprs, locals),
                };

                comp.seq(&|ty, _| Computation::return_(ty))
            }
            Node::TyCast(ref loc, ref expr, ref type_node) => {
                self.process_node(expr, locals).seq(&|_, l| {
                    let ty = self.resolve_type(type_node, &self.type_context, self.scope.clone());
                    Computation::result(self.tyenv.update_loc(ty, loc.clone()), l)
                })
            }
            Node::Redo(_) => {
                // TODO this needs to ensure soundness of assignments when the block is repeated
                // for example in:
                //   x = 123; tap { x; x = "foo"; redo }
                // x should be (Integer|String) at the beginning of the tap block
                Computation::redo()
            }
            Node::Retry(_) => {
                // TODO also needs to ensure soundness of locals (see above)
                Computation::retry()
            }
            Node::Send(ref loc, ref recv, ref mid, ref args) => {
                self.process_send(loc, recv, mid, args, None, locals)
            }
            Node::Block(_, ref send, ref block_args, ref block_body) => {
                if let Node::Send(ref send_loc, ref recv, ref mid, ref args) = **send {
                    self.process_send(send_loc, recv, mid, args, Some((block_args, block_body)), locals)
                } else {
                    panic!("expected Node::Send inside Node::Block")
                }
            }
            _ => panic!("node: {:?}", node),
        }
    }
}
