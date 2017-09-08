use std::rc::Rc;
use std::collections::HashMap;
use typecheck::control::{Computation, ComputationPredicate, EvalResult};
use typecheck::locals::{Locals, LocalEntry, LocalEntryMerge};
use typecheck::types::{Arg, TypeEnv, Type, TypeRef, Prototype, KwsplatResult, TupleElement};
use object::{Scope, RubyObject, MethodImpl, ConstantEntry};
use ast::{Node, Loc, Id};
use environment::Environment;
use errors::Detail;
use typed_arena::Arena;
use typecheck::call;
use typecheck::call::{CallArg, ArgError};
use itertools::Itertools;
use deferred_cell::DeferredCell;

pub struct Eval<'ty, 'object: 'ty> {
    env: &'ty Environment<'object>,
    tyenv: TypeEnv<'ty, 'object>,
    scope: Rc<Scope<'object>>,
    type_context: TypeContext<'ty, 'object>,
    proto: DeferredCell<Rc<Prototype<'ty, 'object>>>,
}

#[derive(Clone)]
pub struct TypeContext<'ty, 'object: 'ty> {
    class: &'object RubyObject<'object>,
    type_parameters: Vec<TypeRef<'ty, 'object>>,
    type_names: HashMap<String, TypeRef<'ty, 'object>>,
}

impl<'ty, 'object> TypeContext<'ty, 'object> {
    fn new(class: &'object RubyObject<'object>, type_parameters: Vec<TypeRef<'ty, 'object>>) -> TypeContext<'ty, 'object> {
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

    pub fn self_type(&self, tyenv: &TypeEnv<'ty, 'object>, loc: Loc) -> TypeRef<'ty, 'object> {
        tyenv.instance(loc, self.class, self.type_parameters.clone())
    }
}

enum HashEntry<'ty, 'object: 'ty> {
    Symbol(Id, TypeRef<'ty, 'object>),
    Pair(TypeRef<'ty, 'object>, TypeRef<'ty, 'object>),
    Kwsplat(TypeRef<'ty, 'object>),
}

#[derive(Clone,Copy)]
enum AnnotationStatus {
    Empty,
    Typed,
    Partial,
    Untyped,
}

impl AnnotationStatus {
    pub fn empty() -> AnnotationStatus {
        AnnotationStatus::Empty
    }

    pub fn append(self, other: AnnotationStatus) -> AnnotationStatus {
        match (self, other) {
            (AnnotationStatus::Typed, AnnotationStatus::Typed) => AnnotationStatus::Typed,
            (AnnotationStatus::Untyped, AnnotationStatus::Untyped) => AnnotationStatus::Untyped,
            (AnnotationStatus::Empty, _) => other,
            _ => AnnotationStatus::Partial,
        }
    }

    pub fn append_into(&mut self, other: AnnotationStatus) {
        *self = self.append(other);
    }

    pub fn or(self, other: AnnotationStatus) -> AnnotationStatus {
        match (self, other) {
            (AnnotationStatus::Empty, _) => other,
            (_, AnnotationStatus::Empty) => self,
            (AnnotationStatus::Partial, _) => AnnotationStatus::Partial,
            (_, AnnotationStatus::Partial) => AnnotationStatus::Partial,
            (AnnotationStatus::Typed, _) => AnnotationStatus::Typed,
            (AnnotationStatus::Untyped, _) => other,
        }
    }
}

enum BlockArg {
    Pass { loc: Loc, node: Rc<Node> },
    Literal { loc: Loc, args: Option<Rc<Node>>, body: Option<Rc<Node>> },
}

impl BlockArg {
    pub fn loc(&self) -> &Loc {
        match *self {
            BlockArg::Pass { ref loc, .. } => loc,
            BlockArg::Literal { ref loc, .. } => loc,
        }
    }
}

struct Invokee<'ty, 'object: 'ty> {
    recv_ty: TypeRef<'ty, 'object>,
    method: Rc<MethodImpl<'object>>,
    prototype: Rc<Prototype<'ty, 'object>>,
}

#[derive(Debug)]
enum Lhs<'ty, 'object: 'ty> {
    Lvar(Loc, String),
    Simple(Loc, TypeRef<'ty, 'object>),
    Send(Loc, TypeRef<'ty, 'object>, Id, Vec<CallArg<'ty, 'object>>),
}

impl<'ty, 'object> Eval<'ty, 'object> {
    pub fn process(env: &'ty Environment<'object>, tyenv: TypeEnv<'ty, 'object>, scope: Rc<Scope<'object>>, class: &'object RubyObject<'object>, node: Rc<Node>) {
        let class_type_parameters = class.type_parameters().iter().map(|&Id(ref loc, _)|
            tyenv.new_var(loc.clone())
        ).collect();

        let mut type_context = TypeContext::new(class, class_type_parameters);

        let mut eval = Eval {
            env: env,
            tyenv: tyenv,
            scope: scope.clone(),
            type_context: type_context.clone(),
            proto: DeferredCell::new()
        };

        let (id, prototype_node, body) = match *node {
            Node::Def(_, ref id, ref proto, ref body) =>
                (id, proto, body),
            Node::Defs(_, _, ref id, ref proto, ref body) =>
                (id, proto, body),
            _ =>
                panic!("unknown node: {:?}", node),
        };

        let (annotation_status, prototype, locals) = {
            let proto_node = prototype_node.as_ref().map(Rc::as_ref);
            let proto_loc = proto_node.map(|n| n.loc().join(&id.0)).unwrap_or_else(|| id.0.clone());
            eval.resolve_prototype(
                &proto_loc,
                proto_node,
                Locals::new(),
                &mut type_context,
                scope.clone())
        };

        match annotation_status {
            AnnotationStatus::Empty |
            AnnotationStatus::Untyped =>
                return,
            AnnotationStatus::Partial => {
                let loc = prototype_node.as_ref().expect("prototype node must exist when annotation status is partial").loc();

                eval.error("Partial type signatures are not permitted in method definitions", &[
                    Detail::Loc("all arguments and return value must be annotated", loc),
                ]);
                return;
            },
            AnnotationStatus::Typed => {},
        };

        // type parameters are initially inserted into the type context
        // unresolved to that they can be constrained. unify any unresolved
        // type variables with their named parameters:
        for (name, ty) in &type_context.type_names {
            if eval.tyenv.is_unresolved_var(*ty) {
                eval.tyenv.unify(*ty, eval.tyenv.alloc(Type::TypeParameter {
                    name: name.clone(),
                    loc: ty.loc().clone(),
                })).expect("unifying unresolved typevar should succeed");
            }
        }

        eval.type_context = type_context;

        DeferredCell::set(&mut eval.proto, prototype.clone());

        // don't typecheck a method if it has no body
        if let Some(ref body_node) = *body {
            eval.process_node(body_node, locals).terminate(&|ty|
                eval.compatible(prototype.retn, ty, None)
            );
        }
    }

    fn error(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn warning(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().warning(message, details)
    }

    fn create_instance_type(&self, loc: &Loc, class: &'object RubyObject<'object>, mut type_parameters: Vec<TypeRef<'ty, 'object>>) -> TypeRef<'ty, 'object> {
        let supplied_params = type_parameters.len();
        let expected_params = class.type_parameters().len();

        if supplied_params == 0 && expected_params > 0 {
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

            for _ in 0..(expected_params - supplied_params) {
                type_parameters.push(self.tyenv.new_var(loc.clone()))
            }
        } else if supplied_params > expected_params {
            self.error("Too many type parameters supplied in instantiation of generic type", &[
                Detail::Loc("from here", type_parameters[expected_params].loc()),
            ]);

            for _ in 0..(supplied_params - expected_params) {
                type_parameters.pop();
            }
        }

        self.tyenv.instance(loc.clone(), class, type_parameters)
    }

    fn resolve_class_instance_type(&self, loc: &Loc, type_parameters: &[Rc<Node>], context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>) -> TypeRef<'ty, 'object> {
        if type_parameters.len() == 0 {
            return self.tyenv.instance0(loc.clone(), self.env.object.Class);
        }

        if type_parameters.len() > 1 {
            self.error("Too many type parameters supplied in instantiation of metaclass", &[
                Detail::Loc("from here", type_parameters[1].loc()),
            ]);
        }

        let cpath = if let Node::TyCpath(_, ref cpath) = *type_parameters[0] {
            cpath
        } else {
            self.error("Type parameter in metaclass must be constant path", &[
                Detail::Loc("here", type_parameters[0].loc()),
            ]);

            return self.tyenv.new_var(loc.clone());
        };

        let class = match **cpath {
            Node::Const(_, None, Id(_, ref name)) => {
                if let Some(&Type::Instance { class, .. }) = context.type_names.get(name).map(TypeRef::deref) {
                    Ok(class)
                } else {
                    self.resolve_type_name(cpath, scope)
                }
            }
            _ => self.resolve_type_name(cpath, scope),
        };

        match class {
            Ok(class@&RubyObject::Module { .. }) |
            Ok(class@&RubyObject::Class { .. }) |
            Ok(class@&RubyObject::Metaclass { .. }) => {
                let metaclass = self.env.object.metaclass(class);
                self.tyenv.instance0(loc.clone(), metaclass)
            },
            Ok(&RubyObject::IClass { .. }) => panic!(),
            Err((err_node, message)) => {
                self.error(message, &[
                    Detail::Loc("here", err_node.loc()),
                ]);
                self.tyenv.new_var(loc.clone())
            }
        }
    }

    fn resolve_type_name<'node>(&self, cpath: &'node Node, scope: Rc<Scope<'object>>)
        -> Result<&'object RubyObject<'object>, (&'node Node, &'static str)>
    {
        self.env.resolve_cpath(cpath, scope).and_then(|constant| {
            match *constant {
                ConstantEntry::Expression { .. } =>
                    Err((cpath, "Constant mentioned in type name does not reference static class/module")),
                ConstantEntry::Module { value, .. } =>
                    Ok(value),
            }
        })
    }

    fn resolve_instance_type(&self, loc: &Loc, cpath: &Node, type_parameters: &[Rc<Node>], context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>) -> TypeRef<'ty, 'object> {
        if let Node::Const(_, None, Id(ref name_loc, ref name)) = *cpath {
            if let Some(&ty) = context.type_names.get(name) {
                if !type_parameters.is_empty() {
                    self.error("Type parameters were supplied but type mentioned does not take any", &[
                        Detail::Loc("here", name_loc),
                    ]);
                }

                return self.tyenv.update_loc(ty, name_loc.clone());
            }
        }

        match self.resolve_type_name(cpath, scope.clone()) {
            Ok(class) if class == self.env.object.Class =>
                self.resolve_class_instance_type(loc, type_parameters, context, scope),
            Ok(class) => {
                let type_parameters = type_parameters.iter().map(|arg|
                    self.resolve_type(arg, context, scope.clone())
                ).collect();

                self.create_instance_type(loc, class, type_parameters)
            }
            Err((err_node, message)) => {
                self.error(message, &[
                    Detail::Loc("here", err_node.loc()),
                ]);

                self.tyenv.new_var(cpath.loc().clone())
            }
        }
    }

    fn create_array_type(&self, loc: &Loc, element_type: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.tyenv.instance(loc.clone(), self.env.object.array_class(), vec![element_type])
    }

    fn create_hash_type(&self, loc: &Loc, key_type: TypeRef<'ty, 'object>, value_type: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.tyenv.instance(loc.clone(), self.env.object.hash_class(), vec![key_type, value_type])
    }

    fn resolve_type(&self, node: &Node, context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>) -> TypeRef<'ty, 'object> {
        match *node {
            Node::TyCpath(ref loc, ref cpath) =>
                self.resolve_instance_type(loc, cpath, &[], context, scope),
            Node::TyGeninst(ref loc, ref cpath, ref args) =>
                self.resolve_instance_type(loc, cpath, args, context, scope),
            Node::TyNil(ref loc) => {
                self.create_instance_type(loc, self.env.object.NilClass, Vec::new())
            },
            Node::TyAny(ref loc) => {
                self.tyenv.any(loc.clone())
            },
            Node::TyArray(ref loc, ref element) => {
                self.create_array_type(loc, self.resolve_type(element, context, scope))
            },
            Node::TyHash(ref loc, ref key, ref value) => {
                self.create_hash_type(loc,
                    self.resolve_type(key, context, scope.clone()),
                    self.resolve_type(value, context, scope))
            },
            Node::TyProc(ref loc, ref prototype) => {
                let mut context = context.clone();

                self.tyenv.alloc(Type::Proc {
                    loc: loc.clone(),
                    proto: self.resolve_prototype(loc, Some(prototype), Locals::new(), &mut context, scope).1,
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

                        self.tyenv.new_var(loc.clone())
                    },
                }
            },
            Node::TyNillable(ref loc, ref type_node) => {
                self.tyenv.nillable(loc, self.resolve_type(type_node, context, scope))
            },
            Node::TyOr(ref loc, ref a, ref b) => {
                self.tyenv.union(loc,
                    self.resolve_type(a, context, scope.clone()),
                    self.resolve_type(b, context, scope))
            }
            Node::TyTuple(ref loc, ref ty_nodes) => {
                let tys = ty_nodes.iter().map(|ty_node| self.resolve_type(ty_node, context, scope.clone())).collect();

                self.tyenv.tuple(loc.clone(), tys, None, vec![])
            }
            _ => panic!("unknown type node: {:?}", node),
        }
    }

    fn resolve_arg(&self, arg_node: &Node, locals: Locals<'ty, 'object>, context: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (AnnotationStatus, Arg<'ty, 'object>, Locals<'ty, 'object>)
    {
        let (status, ty, arg_node) = match *arg_node {
            Node::TypedArg(_, ref type_node, ref arg) => {
                let ty = self.resolve_type(type_node, context, scope.clone());
                (AnnotationStatus::Typed, ty, &**arg)
            },
            _ => {
                (AnnotationStatus::Untyped, self.tyenv.new_var(arg_node.loc().clone()), arg_node)
            },
        };

        match *arg_node {
            Node::Arg(ref loc, ref name) =>
                (status, Arg::Required { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), ty)),
            Node::Blockarg(ref loc, None) =>
                (status, Arg::Block { loc: loc.clone(), ty: ty }, locals),
            Node::Blockarg(ref loc, Some(Id(_, ref name))) =>
                (status, Arg::Block { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), ty)),
            Node::Kwarg(ref loc, ref name) =>
                (status, Arg::Kwarg { loc: loc.clone(), name: name.to_owned(), ty: ty }, locals.assign_shadow(name.to_owned(), ty)),
            Node::Kwoptarg(ref loc, Id(_, ref name), ref expr) =>
                (status, Arg::Kwoptarg { loc: loc.clone(), name: name.to_owned(), ty: ty, expr: expr.clone() }, locals.assign_shadow(name.to_owned(), ty)),
            Node::Mlhs(ref loc, ref nodes) => {
                let mut mlhs_status = AnnotationStatus::empty();
                let mut mlhs_types = Vec::new();
                let mut locals = locals;

                for node in nodes {
                    let (st, arg, l) = self.resolve_arg(node, locals.clone(), context, scope.clone());
                    let arg_ty = if let Arg::Required { ty, .. } = arg {
                        ty
                    } else {
                        self.error("Only required arguments are currently supported in destructuring arguments", &[
                            Detail::Loc("here", arg.loc()),
                        ]);
                        break;
                    };
                    mlhs_status.append_into(st);
                    mlhs_types.push(arg_ty);
                    locals = l;
                }

                let tuple_ty = self.tyenv.tuple(loc.clone(), mlhs_types, None, vec![]);

                let arg = Arg::Required { loc: loc.clone(), ty: tuple_ty };

                (status.or(mlhs_status), arg, locals)
            }
            Node::Optarg(_, Id(ref loc, ref name), ref expr) =>
                (status, Arg::Optional { loc: loc.clone(), ty: ty, expr: expr.clone() }, locals.assign_shadow(name.to_owned(), ty)),
            Node::Restarg(ref loc, None) =>
                (status, Arg::Rest { loc: loc.clone(), ty: ty }, locals),
            Node::Restarg(ref loc, Some(Id(_, ref name))) =>
                (status, Arg::Rest { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), self.create_array_type(loc, ty))),
            Node::Procarg0(ref loc, ref inner_arg_node) => {
                let (status, inner_arg, locals) = self.resolve_arg(inner_arg_node, locals, context, scope);
                (status, Arg::Procarg0 { loc: loc.clone(), arg: Box::new(inner_arg) }, locals)
            }
            Node::Kwrestarg(ref loc, None) =>
                (status, Arg::Kwrest { loc: loc.clone(), ty: ty }, locals),
            Node::Kwrestarg(ref loc, Some(Id(_, ref name))) => {
                let hash_ty = self.create_hash_type(loc, self.tyenv.instance0(loc.clone(), self.env.object.Symbol), ty);
                (status, Arg::Kwrest { loc: loc.clone(), ty: ty }, locals.assign_shadow(name.to_owned(), hash_ty))
            }
            _ => panic!("arg_node: {:?}", arg_node),
        }
    }

    fn resolve_prototype(&self, proto_loc: &Loc, node: Option<&Node>, locals: Locals<'ty, 'object>, context: &mut TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (AnnotationStatus, Rc<Prototype<'ty, 'object>>, Locals<'ty, 'object>)
    {
        let (mut status, args_node, return_type) = match node {
            Some(&Node::Prototype(_, ref genargs, ref args, ref ret)) => {
                let mut status = AnnotationStatus::empty();

                if let Some(ref genargs_) = *genargs {
                    if let Node::TyGenargs(_, ref gendeclargs) = **genargs_ {
                        for gendeclarg in gendeclargs {
                            if let Node::TyGendeclarg(ref loc, ref name, ref constraint) = **gendeclarg {
                                let tyvar = self.tyenv.new_var(loc.clone());
                                context.type_names.insert(name.clone(), tyvar);

                                match constraint.as_ref().map(Rc::as_ref) {
                                    Some(&Node::TyConUnify(ref loc, ref a, ref b)) => {
                                        let a = self.resolve_type(a, &context, scope.clone());
                                        let b = self.resolve_type(b, &context, scope.clone());
                                        self.unify(a, b, Some(loc));
                                    }
                                    Some(&Node::TyConSubtype(ref loc, ref sub, ref super_)) => {
                                        let sub = self.resolve_type(sub, &context, scope.clone());
                                        let super_ = self.resolve_type(super_, &context, scope.clone());
                                        self.compatible(super_, sub, Some(loc));
                                    }
                                    Some(_) => panic!(),
                                    None => {}
                                }
                            }
                        }
                    }

                    status.append_into(AnnotationStatus::Typed);
                }

                let args = args.as_ref().map(Rc::as_ref);

                match *ret {
                    Some(ref type_node) =>
                        (status.append(AnnotationStatus::Typed), args, self.resolve_type(type_node, &context, scope.clone())),
                    None =>
                        (status.append(AnnotationStatus::Untyped), args, self.tyenv.new_var(proto_loc.clone())),
                }
            },
            Some(&Node::Args(..)) | None => {
                (AnnotationStatus::Untyped, node, self.tyenv.new_var(proto_loc.clone()))
            },
            _ => panic!("unexpected {:?}", node),
        };

        let mut args = Vec::new();
        let mut locals = locals;

        match args_node {
            Some(&Node::Args(_, ref arg_nodes)) => {
                for arg_node in arg_nodes {
                    let (arg_status, arg, locals_) = self.resolve_arg(arg_node, locals, &context, scope.clone());
                    status.append_into(arg_status);
                    args.push(arg);
                    locals = locals_;
                }
            }
            Some(_) =>
                panic!("expected args_node to be Node::Args"),
            None => {},
        };

        (status, Rc::new(Prototype { loc: proto_loc.clone(), args: args, retn: return_type }), locals)
    }

    fn type_error(&self, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>, err_a: TypeRef<'ty, 'object>, err_b: TypeRef<'ty, 'object>, loc: Option<&Loc>) {
        let strs = Arena::new();

        let mut details = vec![
            Detail::Loc(strs.alloc(self.tyenv.describe(err_a) + ", with:"), err_a.loc()),
            Detail::Loc(strs.alloc(self.tyenv.describe(err_b)), err_b.loc()),
        ];

        if !err_a.ref_eq(&a) || !err_b.ref_eq(&b) {
            details.push(Detail::Message("arising from an attempt to match:"));
            details.push(Detail::Loc(strs.alloc(self.tyenv.describe(a) + ", with:"), a.loc()));
            details.push(Detail::Loc(strs.alloc(self.tyenv.describe(b)), b.loc()));
        }

        if let Some(loc) = loc {
            details.push(Detail::Loc("in this expression", loc));
        }

        self.error("Could not match types:", &details);
    }

    fn unify(&self, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>, loc: Option<&Loc>) {
        if let Err((err_a, err_b)) = self.tyenv.unify(a, b) {
            self.type_error(a, b, err_a, err_b, loc);
        }
    }

    fn compatible(&self, to: TypeRef<'ty, 'object>, from: TypeRef<'ty, 'object>, loc: Option<&Loc>) {
        if let Err((err_to, err_from)) = self.tyenv.compatible(to, from) {
            self.type_error(to, from, err_to, err_from, loc);
        }
    }

    fn process_array_tuple(&self, loc: &Loc, exprs: &[Rc<Node>], locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        let mut elements = Vec::new();
        let mut result = EvalResult::Ok((), locals);

        for expr in exprs {
            let (splat, node) = match **expr {
                Node::Splat(_, Some(ref node)) => (true, node),
                _ => (false, expr),
            };

            result = result.and_then(|(), locals| {
                self.eval_node(node, locals)
            }).map(|ty| {
                if splat {
                    match *ty {
                        Type::Tuple { ref lead, ref splat, ref post, .. } => {
                            for lead_ty in lead {
                                elements.push(TupleElement::Value(*lead_ty));
                            }

                            if let Some(splat_ty) = *splat {
                                elements.push(TupleElement::Splat(splat_ty));
                            }

                            for post_ty in post {
                                elements.push(TupleElement::Value(*post_ty));
                            }
                        }
                        Type::Instance { class, ref type_parameters, .. }
                            if class == self.env.object.array_class()
                        => {
                            elements.push(TupleElement::Splat(type_parameters[0]));
                        }
                        _ => {
                            self.error("Cannot splat non-array", &[
                                Detail::Loc(&self.tyenv.describe(ty), node.loc()),
                            ]);
                        }
                    }
                } else {
                    elements.push(TupleElement::Value(ty));
                }
            });
        }

        result.map(|()| {
            self.tuple_from_elements(loc.clone(), &elements)
        }).into_computation()
    }

    fn tuple_from_elements(&self, loc: Loc, elements: &[TupleElement<'ty, 'object>]) -> TypeRef<'ty, 'object> {
        use slice_util::View;

        assert!(!elements.is_empty());

        let mut v = View(elements);

        let mut lead_types = Vec::new();
        let mut post_types = Vec::new();

        while let Some(&TupleElement::Value(ty)) = v.first() {
            lead_types.push(ty);
            v.consume_front();
        }

        while let Some(&TupleElement::Value(ty)) = v.last() {
            post_types.push(ty);
            v.consume_back();
        }

        post_types.reverse();

        let splat_type = if !v.is_empty() {
            // first tuple remaining at this point must be a splat:
            panic!("splats unsupported for now");
        } else {
            None
        };

        self.tyenv.tuple(loc, lead_types, splat_type, post_types)
    }

    fn prototype_from_method_impl(&self, loc: &Loc, impl_: &MethodImpl<'object>, mut type_context: TypeContext<'ty, 'object>) -> Rc<Prototype<'ty, 'object>> {
        match *impl_ {
            MethodImpl::Ruby { ref node, ref scope, .. } => {
                let (id, prototype_node) = match **node {
                    Node::Def(_, ref id, ref proto, _) => (id, proto),
                    Node::Defs(_, _, ref id, ref proto, _) => (id, proto),
                    _ => panic!("unexpected node in MethodEntry::Ruby: {:?}", node),
                };

                let prototype_node = prototype_node.as_ref().map(Rc::as_ref);

                let prototype_loc = prototype_node.map(|n| n.loc().join(&id.0)).unwrap_or_else(|| id.0.clone());

                let (anno_status, prototype, _) = self.resolve_prototype(&prototype_loc, prototype_node, Locals::new(), &mut type_context, scope.clone());

                if let AnnotationStatus::Untyped = anno_status {
                    self.tyenv.unify(prototype.retn, self.tyenv.any(prototype.retn.loc().clone()))
                        .expect("retn is unresolved type var");
                }

                prototype
            }
            MethodImpl::AttrReader { ref ivar, ref loc } => {
                let ivar_type = self.lookup_ivar(ivar, &type_context)
                    .unwrap_or_else(|| self.tyenv.any(loc.clone()));

                Rc::new(Prototype {
                    loc: loc.clone(),
                    args: vec![],
                    retn: ivar_type,
                })
            }
            MethodImpl::AttrWriter { ref ivar, ref loc } => {
                let ivar_type = self.lookup_ivar(ivar, &type_context)
                    .unwrap_or_else(|| self.tyenv.any(loc.clone()));

                Rc::new(Prototype {
                    loc: loc.clone(),
                    args: vec![Arg::Required { ty: ivar_type, loc: loc.clone() }],
                    retn: ivar_type,
                })
            }
            MethodImpl::Untyped => self.tyenv.any_prototype(loc.clone()),
            MethodImpl::IntrinsicClassNew => {
                match *type_context.class {
                    RubyObject::Metaclass { of, .. } => {
                        let initialize_method = match self.env.object.lookup_method(of, "initialize") {
                            Some(method) => method,
                            None => {
                                self.error("Can't call #new on class with undefined #initialize method", &[
                                    Detail::Loc("here", loc),
                                ]);

                                return self.tyenv.any_prototype(loc.clone());
                            }
                        };

                        let initialize_type_context = TypeContext::new(of,
                            of.type_parameters().iter().map(|_|
                                self.tyenv.new_var(loc.clone())
                            ).collect()
                        );

                        let instance_type = initialize_type_context.self_type(&self.tyenv, loc.clone());

                        let proto = self.prototype_from_method_impl(loc, &initialize_method.implementation, initialize_type_context);

                        Rc::new(Prototype {
                            loc: proto.loc.clone(),
                            args: proto.args.clone(),
                            retn: instance_type,
                        })
                    },
                    RubyObject::Class { .. } => {
                        // the only way this case can be triggered is calling
                        // #new on an unknown instance of Class, such as:
                        // def foo(Class x); x.new; end
                        // TODO - consider disallowing use of Class without type parameters
                        self.error("Unknown class instance in call to #new, can't determine #initialize signature", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.any_prototype(loc.clone())
                    },
                    _ => panic!("should never happen"),
                }
            }
            MethodImpl::IntrinsicKernelRaise => {
                let any_ty = self.tyenv.any(loc.clone());
                // TODO give Kernel#raise a proper prototype
                Rc::new(Prototype {
                    loc: loc.clone(),
                    args: vec![Arg::Rest { loc: loc.clone(), ty: any_ty }],
                    retn: any_ty,
                })
            }
            MethodImpl::IntrinsicKernelIsA => {
                Rc::new(Prototype {
                    loc: loc.clone(),
                    args: vec![Arg::Required { loc: loc.clone(), ty: self.tyenv.instance0(loc.clone(), self.env.object.Kernel)}],
                    retn: self.tyenv.instance0(loc.clone(), self.env.object.Boolean),
                })
            }
            MethodImpl::IntrinsicProcCall => panic!("should never happen"),
        }
    }

    fn resolve_invocation(&self, recv_type: TypeRef<'ty, 'object>, id: &Id) -> Vec<Invokee<'ty, 'object>>
    {
        let degraded_recv_type = self.tyenv.degrade_to_instance(recv_type);

        match *degraded_recv_type {
            Type::Instance { class, type_parameters: ref tp, .. } => {
                match self.env.object.lookup_method(class, &id.1) {
                    Some(method) => vec![Invokee {
                        recv_ty: recv_type,
                        method: method.implementation.clone(),
                        prototype: self.prototype_from_method_impl(&id.0, &method.implementation, TypeContext::new(class, tp.clone())),
                    }],
                    None => vec![],
                }
            }
            Type::Proc { ref proto, .. } => {
                match self.env.object.lookup_method(self.env.object.Proc, &id.1) {
                    Some(method) => match *method.implementation {
                        MethodImpl::IntrinsicProcCall => vec![Invokee {
                            recv_ty: recv_type,
                            method: method.implementation.clone(),
                            prototype: proto.clone(),
                        }],
                        _ => vec![Invokee {
                            recv_ty: recv_type,
                            method: method.implementation.clone(),
                            prototype: self.prototype_from_method_impl(&id.0, &method.implementation, TypeContext::new(&self.env.object.Proc, Vec::new())),
                        }],
                    },
                    None => vec![],
                }
            }
            Type::Union { ref types, .. } => {
                types.iter().flat_map(|&ty| {
                    // XXX - this is a hack. instead of narrowing local variables we need to narrow types in the tyenv:
                    let ty = if let Type::LocalVariable { ref name, ref loc, .. } = *recv_type {
                        self.tyenv.local_variable(loc.clone(), name.clone(), ty)
                    } else {
                        ty
                    };

                    let invokees = self.resolve_invocation(ty, id);

                    if invokees.is_empty() {
                        let message = format!("Union member {} does not respond to #{}", self.tyenv.describe(ty), &id.1);
                        self.error(&message, &[
                            Detail::Loc(&self.tyenv.describe(recv_type), recv_type.loc()),
                        ]);
                    }

                    invokees
                }).collect()
            }
            Type::Any { .. } => vec![Invokee {
                recv_ty: recv_type,
                method: Rc::new(MethodImpl::Untyped),
                prototype: self.tyenv.any_prototype(id.0.clone()),
            }],
            Type::TypeParameter { ref name, .. } => {
                self.error(&format!("Type parameter {} is of unknown type", name), &[
                    Detail::Loc("in receiver", recv_type.loc()),
                    Detail::Loc("of this invocation", &id.0),
                ]);

                vec![]
            }
            Type::Var { id: tyid, .. } => {
                self.error(&format!("Type of receiver is not known at this point"), &[
                    Detail::Loc(&format!("t{}", tyid), recv_type.loc()),
                    Detail::Loc("in this invocation", &id.0),
                ]);

                vec![]
            }
            Type::KeywordHash { .. } => panic!("should have degraded to instance"),
            Type::Tuple { .. } => panic!("should have degraded to instance"),
            Type::LocalVariable { .. } => panic!("should never remain after prune"),
        }
    }

    fn merge_locals(&self, a: Locals<'ty, 'object>, b: Locals<'ty, 'object>) -> Locals<'ty, 'object> {
        let mut merges = Vec::new();
        let merged_locals = a.merge(b, &self.tyenv, &mut merges);
        self.process_local_merges(merges);
        merged_locals
    }

    fn process_local_merges(&self, merges: Vec<LocalEntryMerge<'ty, 'object>>) {
        for merge in merges {
            match merge {
                LocalEntryMerge::Ok(_) => {},
                LocalEntryMerge::MustMatch(_, to, from) => self.compatible(to, from, None),
            }
        }
    }

    fn extract_results(&self, comp: Computation<'ty, 'object>, loc: &Loc) -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>> {
        let mut merges = Vec::new();
        let result = comp.extract_results(loc, &self.tyenv, &mut merges);
        self.process_local_merges(merges);
        result
    }

    fn converge_results(&self, comp: Computation<'ty, 'object>, loc: &Loc) -> Computation<'ty, 'object> {
        let mut merges = Vec::new();
        let comp = comp.converge_results(loc, &self.tyenv, &mut merges);
        self.process_local_merges(merges);
        comp
    }

    fn process_call_arg(&self, node: &Node, locals: Locals<'ty, 'object>) -> EvalResult<'ty, 'object, CallArg<'ty, 'object>> {
        match *node {
            Node::Splat(_, ref n) => {
                let splat_node = n.as_ref().expect("splat in call arg must have node");

                self.eval_node(splat_node, locals).map(|ty|
                    match *self.tyenv.prune(ty) {
                        Type::Instance { class, ref type_parameters, .. } if class.is_a(self.env.object.array_class()) => {
                            CallArg::Splat(node.loc().clone(), type_parameters[0])
                        }
                        _ => {
                            self.error("Cannot splat non-array", &[
                                Detail::Loc(&self.tyenv.describe(ty), splat_node.loc()),
                            ]);
                            CallArg::Splat(node.loc().clone(), self.tyenv.new_var(node.loc().clone()))
                        }
                    }
                )
            }
            _ =>
                self.eval_node(node, locals).map(|ty|
                    CallArg::Pass(node.loc().clone(), ty)
                ),
        }
    }

    fn prototype_from_procish_type(&self, procish_ty: TypeRef<'ty, 'object>)
        -> Result<Rc<Prototype<'ty, 'object>>, Option<&'static str>>
    {
        match *procish_ty {
            Type::Union { ref types, .. } => {
                let non_nil_types = types.iter().filter(|ty|
                    if let Type::Instance { class, .. } = ***ty {
                        !class.is_a(self.env.object.NilClass)
                    } else {
                        true
                    }
                ).collect::<Vec<_>>();

                if non_nil_types.len() == 1 {
                    self.prototype_from_procish_type(*non_nil_types[0])
                } else {
                    return Err(Some("because the block type defined in the method prototype is too complex"));
                }
            }
            Type::Proc { ref proto, .. } => Ok(proto.clone()),
            Type::Any { .. } => Err(None),
            _ => {
                return Err(Some("because the block type defined in the method prototype is not a proc type"));
            },
        }
    }

    fn infer_symbol_as_proc_type(&self, proto_block_ty: TypeRef<'ty, 'object>, mid: &str, loc: &Loc)
        -> TypeRef<'ty, 'object>
    {
        let proto = match self.prototype_from_procish_type(proto_block_ty) {
            Ok(proto) => proto,
            Err(None) => return self.tyenv.any(loc.clone()),
            Err(Some(msg)) => {
                self.error("Can't infer type for symbol-as-proc", &[
                    Detail::Loc("passed here", loc),
                    Detail::Loc(msg, proto_block_ty.loc())
                ]);

                return self.tyenv.new_var(loc.clone());
            }
        };

        fn recv_ty_from_arg<'ty, 'object: 'ty>(arg: Option<&Arg<'ty, 'object>>) -> Option<TypeRef<'ty, 'object>> {
            match arg {
                Some(&Arg::Required { ty, .. }) => Some(ty),
                Some(&Arg::Procarg0 { ref arg, .. }) => recv_ty_from_arg(Some(arg)),
                _ => None,
            }
        }

        if let Some(ty) = recv_ty_from_arg(proto.args.first()) {
            let invokee_proc_ty = self.tyenv.alloc(Type::Proc {
                loc: proto_block_ty.loc().clone(),
                proto: Rc::new(Prototype {
                    loc: proto.loc.clone(),
                    args: proto.args[1..].iter().cloned().collect(),
                    retn: proto.retn,
                })
            });

            let invokees = self.resolve_invocation(ty, &Id(loc.clone(), mid.to_owned()));

            if invokees.is_empty() {
                self.error(&format!("Could not resolve method #{}", mid), &[
                    Detail::Loc(&format!("on {}", &self.tyenv.describe(ty)), ty.loc()),
                    Detail::Loc("in symbol-as-proc", loc),
                ]);

                return self.tyenv.any(loc.clone());
            }

            if invokees.len() > 0 {
                for invokee in invokees {
                    let prototype_ty = self.tyenv.alloc(Type::Proc {
                        loc: invokee.prototype.loc().clone(),
                        proto: invokee.prototype.clone(),
                    });

                    self.compatible(invokee_proc_ty, prototype_ty, Some(loc));
                }
            }

            self.tyenv.alloc(Type::Proc {
                loc: loc.clone(),
                proto: proto.clone(),
            })
        } else {
            self.error("Can't infer type for symbol-as-proc", &[
                Detail::Loc("passed here", loc),
                Detail::Loc("because the block type defined in the method prototype has no required arguments", proto_block_ty.loc()),
            ]);

            return self.tyenv.new_var(loc.clone());
        }
    }

    fn block_type_from_block_pass(&self, proto_block_ty: TypeRef<'ty, 'object>, node: &Node, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        self.process_node(node, locals).seq(&|ty, l| {
            match *ty {
                Type::Instance { class, .. } if class == self.env.object.Symbol => {
                    if let Node::Symbol(_, ref sym) = *node {
                        let proc_ty = self.infer_symbol_as_proc_type(proto_block_ty, sym, node.loc());
                        Computation::result(proc_ty, l)
                    } else {
                        self.error("Expected symbol literal in block pass", &[
                            Detail::Loc("but an expression evaluating to a Symbol instance was passed instead", node.loc()),
                        ]);
                        Computation::result(self.tyenv.new_var(node.loc().clone()), l)
                    }
                }
                _ => Computation::result(ty, l)
            }
        })
    }

    fn process_block(&self, send_loc: &Loc, block: Option<&BlockArg>, locals: Locals<'ty, 'object>, proto_loc: &Loc, prototype_block: Option<TypeRef<'ty, 'object>>)
        -> EvalResult<'ty, 'object, ()>
    {
        match (prototype_block, block) {
            (None, None) => {
                EvalResult::Ok((), locals)
            }
            (None, Some(block)) => {
                self.error("Block passed in method invocation", &[
                    Detail::Loc("here", block.loc()),
                    Detail::Loc("but this method does not take a block", send_loc),
                    Detail::Loc("as defined here", proto_loc),
                ]);

                EvalResult::Ok((), locals)
            }
            (Some(proto_block_ty), Some(&BlockArg::Pass { ref loc, ref node, .. })) => {
                self.extract_results(self.block_type_from_block_pass(proto_block_ty, node, locals), loc).map(|ty| {
                    self.compatible(proto_block_ty, ty, Some(loc));
                })
            }
            (Some(proto_block_ty), Some(&BlockArg::Literal { ref loc, ref args, ref body })) => {
                let block_locals = locals.extend();

                let mut block_type_context = self.type_context.clone();

                let (_, block_prototype, block_locals) = self.resolve_prototype(loc, args.as_ref().map(Rc::as_ref), block_locals, &mut block_type_context, self.scope.clone());

                let block_return_type = block_prototype.retn;

                let block_proc_type = self.tyenv.alloc(Type::Proc {
                    loc: loc.clone(),
                    proto: block_prototype,
                });

                self.compatible(proto_block_ty, block_proc_type, None);

                let comp = match *body {
                    None => Computation::result(self.tyenv.nil(loc.clone()), block_locals),
                    Some(ref body_node) => self.process_node(body_node, block_locals),
                };

                let comp = comp.terminate_next_scope()
                    .map_locals(&|locals| locals.unextend());

                self.extract_results(comp, loc)
                    .map(|ty| self.compatible(block_return_type, ty, None))
            }
            (Some(proto_block_ty), None) => {
                // intentionally calling tyenv.compatible so this
                // does not emit an error if types are incompatible:
                let nil_block = self.tyenv.nil(send_loc.clone() /* just need a dummy location */);

                if let Err(..) = self.tyenv.compatible(proto_block_ty, nil_block) {
                    self.error("Expected block of type", &[
                        Detail::Loc(&self.tyenv.describe(proto_block_ty), proto_block_ty.loc()),
                        Detail::Loc("in this method invocation", send_loc),
                    ]);
                }

                EvalResult::Ok((), locals)
            }
        }
    }

    fn process_send_receiver(&self, recv: &Option<Rc<Node>>, id: &Id, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>>
    {
        match *recv {
            Some(ref recv_node) => {
                let ev = self.eval_node(recv_node, locals);

                if let EvalResult::NonResult(_) = ev {
                    self.warning("Useless method call", &[
                        Detail::Loc("here", &id.0),
                        Detail::Loc("receiver never evaluates to a result", recv_node.loc()),
                    ]);
                }

                ev
            },
            None => EvalResult::Ok(self.type_context.self_type(&self.tyenv, id.0.clone()), locals),
        }
    }

    fn process_send_args(&self, invoc_loc: &Loc, arg_nodes: &[Rc<Node>], locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, Vec<CallArg<'ty, 'object>>>
    {
        arg_nodes.iter().fold(EvalResult::Ok(Vec::new(), locals), |result, arg_node|
            result.and_then(|mut args, locals| {
                self.process_call_arg(arg_node, locals).and_then(|call_arg, locals| {
                    args.push(call_arg);
                    EvalResult::Ok(args, locals)
                }).if_not(|| {
                    self.warning("Useless invocation", &[
                        Detail::Loc("here", invoc_loc),
                        Detail::Loc("argument never evaluates to a result", arg_node.loc()),
                    ]);
                })
            })
        )
    }

    fn match_prototype_with_invocation(&self, expr_loc: &Loc, invoc_loc: &Loc, proto_loc: &Loc, proto_args: &[Arg<'ty, 'object>], args: &[CallArg<'ty, 'object>]) {
        let args = match proto_args.first() {
            Some(&Arg::Procarg0 { .. }) if args.len() > 1 => {
                let tuple_elements = args.iter().map(|call_arg| match *call_arg {
                    CallArg::Pass(_, ty) => TupleElement::Value(ty),
                    CallArg::Splat(_, ty) => TupleElement::Splat(ty),
                }).collect::<Vec<_>>();

                let args_loc = args[0].loc().join(args.last().unwrap().loc());

                let arg_ty = self.tuple_from_elements(args_loc.clone(), &tuple_elements);

                vec![CallArg::Pass(args_loc, arg_ty)]
            }
            _ => args.to_vec(),
        };

        let match_result = call::match_prototype_with_invocation(&self.tyenv, proto_args, &args);

        for match_error in match_result.errors {
            match match_error {
                ArgError::TooFewArguments => {
                    self.error("Too few arguments supplied", &[
                        Detail::Loc("in this invocation", invoc_loc),
                        Detail::Loc("for this prototype", proto_loc),
                    ])
                }
                ArgError::TooManyArguments(ref loc) => {
                    self.error("Too many arguments supplied", &[
                        Detail::Loc("from here", loc),
                        Detail::Loc("in this invocation", invoc_loc),
                        Detail::Loc("for this prototype", proto_loc),
                    ])
                }
                ArgError::MissingKeyword(ref name) => {
                    self.error(&format!("Missing keyword argument :{}", name), &[
                        Detail::Loc("in this invocation", invoc_loc),
                        Detail::Loc("for this prototype", proto_loc),
                    ])
                }
                ArgError::UnknownKeyword(ref name) => {
                    self.error(&format!("Unknown keyword argument :{}", name), &[
                        Detail::Loc("in this invocation", invoc_loc),
                        Detail::Loc("for this prototype", proto_loc),
                    ])
                }
                ArgError::UnexpectedSplat(ref loc) => {
                    self.error("Unexpected splat in keyword arguments", &[
                        Detail::Loc("here", loc),
                        Detail::Loc("in this invocation", invoc_loc),
                        Detail::Loc("for this prototype", proto_loc),
                    ])
                }
            }
        }

        for (proto_ty, pass_ty) in match_result.matches {
            self.compatible(proto_ty, pass_ty, Some(expr_loc));
        }
    }

    fn process_intrinsic_kernel_is_a(&self, expr_loc: &Loc, invokee: &Invokee<'ty, 'object>, args: &[CallArg<'ty, 'object>], locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        let no_intrinsic = |locals| {
            let boolean_ty = self.tyenv.instance0(expr_loc.clone(), self.env.object.Boolean);
            Computation::result(boolean_ty, locals)
        };

        let (arg_loc, arg_ty) = if let Some(&CallArg::Pass(ref arg_loc, arg_ty)) = args.first() {
            (arg_loc, self.tyenv.prune(arg_ty))
        } else {
            return no_intrinsic(locals);
        };

        if let Type::Instance { class: &RubyObject::Metaclass { of: instance_class, .. }, .. } = *arg_ty {
            let refine = |retn_class, refine_ty| {
                let retn_ty = self.tyenv.instance0(expr_loc.clone(), retn_class);

                let locals = if let Type::LocalVariable { ref name, .. } = *invokee.recv_ty {
                    locals.refine(name, refine_ty)
                } else {
                    locals.clone()
                };

                Computation::result(retn_ty, locals)
            };

            self.tyenv.partition_by_class(invokee.recv_ty, instance_class, arg_loc)
                .map_left(|refine_ty| refine(self.env.object.TrueClass, refine_ty))
                .map_right(|refine_ty| refine(self.env.object.FalseClass, refine_ty))
                .flatten(Computation::divergent)
        } else {
            return no_intrinsic(locals);
        }
    }

    fn process_invocation(&self, expr_loc: &Loc, invoc_loc: &Loc, invokee: &Invokee<'ty, 'object>, args: &[CallArg<'ty, 'object>], block: Option<&BlockArg>, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        let ref proto_loc = invokee.prototype.loc;
        let ref proto_args = invokee.prototype.args;
        let retn = invokee.prototype.retn;

        let (proto_block, proto_args) = match proto_args.last() {
            Some(&Arg::Block { ty, .. }) => (Some(ty), &proto_args[..proto_args.len() - 1]),
            Some(_) | None => (None, proto_args.as_slice()),
        };

        self.match_prototype_with_invocation(expr_loc, invoc_loc, proto_loc, proto_args, args);

        let comp = self.process_block(invoc_loc, block, locals, invokee.prototype.loc(), proto_block).and_then_comp(|(), locals| {
            match *invokee.method {
                MethodImpl::IntrinsicKernelRaise => {
                    Computation::raise(locals)
                }
                MethodImpl::IntrinsicKernelIsA => {
                    self.process_intrinsic_kernel_is_a(expr_loc, invokee, args, locals)
                }
                _ => {
                    let ty = self.tyenv.update_loc(retn, expr_loc.clone());
                    Computation::result(ty, locals)
                }

            }
        });

        comp.terminate_break_scope()
    }

    fn process_send_dispatch(&self, loc: &Loc, recv_type: TypeRef<'ty, 'object>, id: &Id, args: Vec<CallArg<'ty, 'object>>, block: Option<BlockArg>, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        self.resolve_invocation(recv_type, id)
            .iter()
            .map(|invokee| self.process_invocation(loc, &id.0, invokee, &args, block.as_ref(), locals.clone()))
            .fold1(Computation::divergent)
            .unwrap_or_else(|| {
                self.error(&format!("Could not resolve method #{}", &id.1), &[
                    Detail::Loc(&format!("on {}", &self.tyenv.describe(recv_type)), recv_type.loc()),
                    Detail::Loc("in this invocation", &id.0),
                ]);

                Computation::result(self.tyenv.any(loc.clone()), locals)
            })
    }

    fn process_send(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, arg_nodes: &[Rc<Node>], block: Option<BlockArg>, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        self.process_send_receiver(recv, id, locals).and_then_comp(|recv_type, locals| {
            self.process_send_args(&id.0, arg_nodes, locals).and_then_comp(|args, locals| {
                self.process_send_dispatch(loc, recv_type, id, args, block, locals)
            })
        })
    }

    fn process_yield(&self, loc: &Loc, invoc_loc: &Loc, arg_nodes: &[Rc<Node>], locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        self.process_send_args(invoc_loc, arg_nodes, locals).and_then_comp(|args, locals| {
            if let Some(&Arg::Block { ty: block_ty, .. }) = self.proto.args.last() {
                if let Type::Proc { ref proto, .. } = *self.tyenv.prune(block_ty) {
                    self.match_prototype_with_invocation(loc, invoc_loc, &proto.loc, &proto.args, &args);

                    let retn_ty = self.tyenv.update_loc(proto.retn, loc.clone());

                    Computation::result(retn_ty, locals)
                } else {
                    self.error("Cannot yield to block of type", &[
                        Detail::Loc(&self.tyenv.describe(block_ty), block_ty.loc())
                    ]);

                    Computation::result(self.tyenv.any(loc.clone()), locals)
                }
            } else {
                self.error("Cannot yield in method without formal block argument", &[
                    Detail::Loc("here", invoc_loc),
                ]);

                Computation::result(self.tyenv.any(loc.clone()), locals)
            }
        })
    }

    fn seq_process(&self, comp: Computation<'ty, 'object>, node: &Node) -> Computation<'ty, 'object> {
        comp.seq(&|_, locals| self.process_node(node, locals))
    }

    fn seq_process_option(&self, comp: Computation<'ty, 'object>, node: &Option<Rc<Node>>, loc: &Loc) -> Computation<'ty, 'object> {
        comp.seq(&|_, locals|
            match *node {
                Some(ref node) => self.process_node(node, locals),
                None => Computation::result(self.tyenv.nil(loc.clone()), locals),
            }
        )
    }

    fn lookup_ivar(&self, name: &str, type_context: &TypeContext<'ty, 'object>) -> Option<TypeRef<'ty, 'object>> {
        self.env.object.lookup_ivar(type_context.class, name).map(|ivar|
            self.resolve_type(&ivar.type_node, type_context, ivar.scope.clone()))
    }

    fn lookup_ivar_or_error(&self, id: &Id, type_context: &TypeContext<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.lookup_ivar(&id.1, type_context).unwrap_or_else(|| {
            self.error("Use of undeclared instance variable", &[
                Detail::Loc("here", &id.0),
            ]);

            self.tyenv.any(id.0.clone())
        })
    }

    fn lookup_lvar(&self, loc: &Loc, name: &str, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, Option<TypeRef<'ty, 'object>>>
    {
        let (ty, locals) = locals.lookup(name);

        let ty = match ty {
            LocalEntry::Bound(ty) |
            LocalEntry::Pinned(ty) => {
                let lv_ty = self.tyenv.local_variable(loc.clone(), name.to_owned(), ty);
                Some(lv_ty)
            }
            LocalEntry::ConditionallyPinned(ty) => {
                let lv_ty = self.tyenv.nillable(loc, self.tyenv.local_variable(loc.clone(), name.to_owned(), ty));
                Some(lv_ty)
            }
            LocalEntry::Unbound => None,
        };

        EvalResult::Ok(ty, locals)
    }

    fn lookup_lvar_or_error(&self, loc: &Loc, name: &str, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>>
    {
        self.lookup_lvar(loc, name, locals).map(|ty| {
            ty.unwrap_or_else(|| {
                self.error("Use of uninitialised local variable", &[
                    Detail::Loc("here", loc),
                ]);

                self.tyenv.nil(loc.clone())
            })
        })
    }

    fn assign_lvar(&self, name: &str, ty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>, loc: &Loc)
        -> Locals<'ty, 'object>
    {
        match locals.assign(name.to_owned(), ty) {
            // in the none case, the assignment happened
            // successfully and the local variable entry is now set
            // to the type we passed in:
            (None, l) => l,
            // in the some case, the local variable is already
            // pinned to a type and we must check type compatibility:
            (Some(lvar_ty), l) => {
                self.compatible(lvar_ty, ty, Some(loc));
                l
            }
        }
    }

    fn type_for_lhs(&self, lhs: &Lhs<'ty, 'object>, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>>
    {
        match *lhs {
            Lhs::Lvar(ref loc, ref name) => {
                let lv_ty = self.tyenv.new_var(loc.clone());
                match locals.assign(name.clone(), lv_ty) {
                    (Some(ty), locals) => EvalResult::Ok(ty, locals),
                    (None, locals) => EvalResult::Ok(lv_ty, locals),
                }
            }
            Lhs::Simple(_, ty) => EvalResult::Ok(ty, locals),
            Lhs::Send(ref loc, recv_ty, ref id, ref args) => {
                let id = Id(id.0.clone(), id.1.clone() + "=");

                let rhs_ty = self.tyenv.new_var(loc.clone());

                let mut args = args.clone();
                args.push(CallArg::Pass(loc.clone(), rhs_ty));

                let comp = self.process_send_dispatch(loc, recv_ty,  &id, args, None, locals);

                self.extract_results(comp, loc).map(|_| rhs_ty)
            }
        }
    }

    fn process_lhs(&self, lhs: &Node, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, Lhs<'ty, 'object>>
    {
        match *lhs {
            Node::LvarLhs(ref loc, Id(_, ref name)) => {
                EvalResult::Ok(Lhs::Lvar(loc.clone(), name.clone()), locals)
            }
            Node::IvarLhs(ref loc, Id(_, ref name)) => {
                let iv_ty = self.lookup_ivar_or_error(&Id(loc.clone(), name.clone()), &self.type_context);

                EvalResult::Ok(Lhs::Simple(loc.clone(), iv_ty), locals)
            }
            Node::Send(ref loc, ref recv, ref id, ref arg_nodes) => {
                self.process_send_receiver(recv, id, locals).and_then(|recv_ty, locals| {
                    self.process_send_args(&id.0, arg_nodes, locals).map(|args| {
                        Lhs::Send(loc.clone(), recv_ty, id.clone(), args)
                    })
                })
            }
            Node::Mlhs(ref loc, ref nodes) => {
                let mut locals = locals;
                let mut lead_types = Vec::new();
                let mut post_types = Vec::new();
                let mut splat_type = None;
                let mut non_result_comp = None;

                for node in nodes {
                    let (splat, node) = match **node {
                        Node::Splat(_, Some(ref node)) => (true, node),
                        _ => (false, node),
                    };

                    let ty_result = self.process_lhs(node, locals).and_then(|lhs, locals| {
                        self.type_for_lhs(&lhs, locals)
                    });

                    let ty = match ty_result {
                        EvalResult::Ok(ty, l) => {
                            locals = l;
                            ty
                        }
                        EvalResult::Both(ty, l, comp) => {
                            locals = l;
                            non_result_comp = Computation::divergent_option(non_result_comp, Some(comp));
                            ty
                        }
                        EvalResult::NonResult(comp) => {
                            return EvalResult::NonResult(Computation::divergent_option(non_result_comp, Some(comp)).unwrap());
                        }
                    };

                    if splat {
                        splat_type = Some(ty);
                    } else {
                        if let Some(_) = splat_type {
                            post_types.push(ty);
                        } else {
                            lead_types.push(ty);
                        }
                    }
                }

                let tuple = self.tyenv.tuple(loc.clone(), lead_types, splat_type, post_types);

                let lhs = Lhs::Simple(loc.clone(), tuple);

                match non_result_comp {
                    Some(comp) => EvalResult::Both(lhs, locals, comp),
                    None => EvalResult::Ok(lhs, locals),
                }
            }
            _ => panic!("unknown node type in lhs: {:?}", lhs),
        }
    }

    fn process_asgn(&self, lhs: &Node, rty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>, loc: &Loc)
        -> EvalResult<'ty, 'object, ()>
    {
        self.process_lhs(lhs, locals).and_then(|lhs, locals| {
            self.assign(lhs, rty, locals, loc)
        })
    }

    fn assign(&self, lhs: Lhs<'ty, 'object>, rty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>, loc: &Loc)
        -> EvalResult<'ty, 'object, ()>
    {
        match lhs {
            Lhs::Lvar(_, name) => {
                match locals.assign(name, rty) {
                    (Some(ty), locals) => {
                        self.compatible(ty, rty, Some(loc));
                        EvalResult::Ok((), locals)
                    }
                    (None, locals) => {
                        EvalResult::Ok((), locals)
                    }
                }
            }
            Lhs::Simple(_, lty) => {
                self.compatible(lty, rty, Some(loc));
                EvalResult::Ok((), locals)
            }
            Lhs::Send(_, recv_ty, ref id, ref args) => {
                let mut args = args.clone();
                args.push(CallArg::Pass(rty.loc().clone(), rty));

                let id = Id(id.0.clone(), id.1.clone() + "=");

                let send_comp = self.process_send_dispatch(loc, recv_ty, &id, args, None, locals);
                self.extract_results(send_comp, loc).map(|_| ())
            }
        }
    }

    fn eval_node(&self, node: &Node, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>>
    {
        let comp = self.process_node(node, locals);
        self.extract_results(comp, node.loc())
    }

    fn process_option_node(&self, loc: &Loc, node: Option<&Node>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        match node {
            Some(node) => self.process_node(node, locals),
            None => Computation::result(self.tyenv.nil(loc.clone()), locals),
        }
    }

    fn process_seq_stmts(&self, loc: &Loc, nodes: &[Rc<Node>], locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        let comp = Computation::result(self.tyenv.nil(loc.clone()), locals);

        nodes.iter().fold(comp, |comp, node|
            self.converge_results(self.seq_process(comp, node), node.loc()))
    }

    fn process_command_args(&self, loc: &Loc, nodes: &[Rc<Node>], locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        match nodes.len() {
            0 => Computation::result(self.tyenv.nil(loc.clone()), locals),
            1 => self.process_node(nodes.first().unwrap(), locals),
            _ => {
                let loc = nodes[0].loc().join(nodes.last().unwrap().loc());
                self.process_array_tuple(&loc, nodes, locals)
            }
        }
    }

    fn process_node(&self, node: &Node, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        match *node {
            Node::Array(ref loc, ref elements) => {
                if elements.is_empty() {
                    let elem_ty = self.tyenv.new_var(loc.clone());
                    let array_ty = self.tyenv.instance(loc.clone(), self.env.object.array_class(), vec![elem_ty]);
                    Computation::result(array_ty, locals)
                } else {
                    self.process_array_tuple(loc, elements, locals)
                }
            }
            Node::Begin(ref loc, ref nodes) |
            Node::Kwbegin(ref loc, ref nodes) => {
                self.process_seq_stmts(loc, nodes, locals)
            }
            Node::Lvar(ref loc, ref name) => {
                self.lookup_lvar_or_error(loc, name, locals).into_computation()
            }
            Node::LvarLhs(ref loc, Id(_, ref name)) => {
                self.lookup_lvar(loc, name, locals)
                    .map(|ty| ty.unwrap_or_else(|| self.tyenv.nil(loc.clone())))
                    .into_computation()
            }
            Node::LvarAsgn(ref asgn_loc, Id(_, ref lvar_name), ref expr) => {
                self.process_node(expr, locals).seq(&|expr_ty, l| {
                    let l = self.assign_lvar(lvar_name, expr_ty, l, asgn_loc);

                    let lvar_ty = self.tyenv.local_variable(asgn_loc.clone(), lvar_name.clone(), expr_ty);

                    Computation::result(lvar_ty, l)
                })
            }
            Node::Ivar(ref loc, ref name) |
            Node::IvarLhs(ref loc, Id(_, ref name)) => {
                let ty = self.lookup_ivar_or_error(&Id(loc.clone(), name.clone()), &self.type_context);

                Computation::result(ty, locals)
            }
            Node::IvarAsgn(ref loc, ref ivar, ref expr) => {
                let ivar_ty = self.lookup_ivar_or_error(ivar, &self.type_context);

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
                self.process_command_args(loc, exprs, locals).seq(&|ty, _|
                    Computation::return_(ty))
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
            Node::Next(ref loc, ref exprs) => {
                self.process_command_args(loc, exprs, locals).seq(&|ty, locals|
                    Computation::next(ty, locals))
            }
            Node::Break(ref loc, ref exprs) => {
                self.process_command_args(loc, exprs, locals).seq(&|ty, locals|
                    Computation::break_(ty, locals))
            }
            Node::Send(ref loc, ref recv, ref mid, ref args) => {
                let (block, args) = match args.last().map(|x| &**x) {
                    Some(&Node::BlockPass(ref block_pass_loc, ref block_node)) =>
                        (Some(BlockArg::Pass { loc: block_pass_loc.clone(), node: block_node.clone() }),
                            &args[..args.len() - 1]),
                    Some(_) => (None, args.as_slice()),
                    None => (None, args.as_slice()),
                };

                self.process_send(loc, recv, mid, args, block, locals)
            }
            Node::Block(ref loc, ref send, ref block_args, ref block_body) => {
                if let Node::Send(ref send_loc, ref recv, ref mid, ref args) = **send {
                    let mut block_loc = loc.clone();
                    block_loc.begin_pos = send_loc.end_pos + 1;

                    let block = BlockArg::Literal { loc: block_loc, args: block_args.clone(), body: block_body.clone() };

                    self.process_send(loc, recv, mid, args, Some(block), locals)
                } else {
                    panic!("expected Node::Send inside Node::Block")
                }
            }
            Node::Yield(ref loc, ref args) => {
                let invoc_loc = Loc {
                    file: loc.file.clone(),
                    begin_pos: loc.begin_pos,
                    end_pos: loc.begin_pos + 5,
                };

                self.process_yield(loc, &invoc_loc, args, locals)
            }
            Node::Hash(ref loc, ref pairs) => {
                let mut result = EvalResult::Ok((), locals);
                let mut entries = Vec::new();

                for pair in pairs {
                    match **pair {
                        Node::Pair(_, ref key, ref value) => {
                            result = result.and_then(|(), locals|
                                self.eval_node(key, locals).if_not(|| {
                                    self.warning("Expression never evalutes to a result", &[
                                        Detail::Loc("here", key.loc()),
                                    ])
                                })
                            ).and_then(|key_ty, locals| {
                                self.eval_node(value, locals).if_not(|| {
                                    self.warning("Expression never evalutes to a result", &[
                                        Detail::Loc("here", value.loc()),
                                    ])
                                }).map(|value_ty| {
                                    (key_ty, value_ty)
                                })
                            }).map(|(key_ty, value_ty)| {
                                if let Node::Symbol(ref sym_loc, ref sym) = **key {
                                    entries.push(HashEntry::Symbol(Id(sym_loc.clone(), sym.clone()), value_ty));
                                } else {
                                    entries.push(HashEntry::Pair(key_ty, value_ty));
                                }
                            });
                        },
                        Node::Kwsplat(_, ref splat) => {
                            result = result.and_then(|(), locals|
                                self.eval_node(splat, locals).if_not(|| {
                                    self.warning("Expression never evalutes to a result", &[
                                        Detail::Loc("here", splat.loc()),
                                    ]);
                                }).map(|ty| {
                                    entries.push(HashEntry::Kwsplat(ty));
                                })
                            );
                        },
                        _ => panic!("unexpected node type in hash literal: {:?}", *pair),
                    }
                }

                let is_keyword_hash = entries.len() > 0 && entries.iter().all(|entry| {
                    match *entry {
                        HashEntry::Symbol(..) => true,
                        HashEntry::Kwsplat(ty) => self.tyenv.is_keyword_hash(ty),
                        HashEntry::Pair(..) => false,
                    }
                });

                let hash_ty = if is_keyword_hash {
                    let mut keywords = Vec::new();
                    let mut splat_ty = None;

                    for entry in entries {
                        match entry {
                            HashEntry::Symbol(Id(_, key), value) => keywords.push((key, value)),
                            HashEntry::Kwsplat(kw_ty) => match *self.tyenv.to_keyword_hash(kw_ty).unwrap() {
                                Type::KeywordHash { keywords: ref splat_keywords, splat, .. } => {
                                    keywords.extend(splat_keywords.iter().cloned());
                                    splat_ty = match (splat_ty, splat) {
                                        (None, None) => None,
                                        (Some(t), None) | (None, Some(t)) => Some(t),
                                        (Some(a), Some(b)) => Some(self.tyenv.union(loc, a, b)),
                                    };
                                }
                                _ => panic!("should not happen"),
                            },
                            _ => panic!("should not happen"),
                        }
                    }

                    self.tyenv.keyword_hash(loc.clone(), keywords, splat_ty)
                } else {
                    let (key_ty, value_ty) =
                        entries.into_iter().filter_map(|hash_entry| {
                            match hash_entry {
                                HashEntry::Symbol(Id(sym_loc, _), value) =>
                                    Some((self.tyenv.instance0(sym_loc, self.env.object.Symbol), value)),
                                HashEntry::Pair(key, value) =>
                                    Some((key, value)),
                                HashEntry::Kwsplat(ty) =>
                                    match self.tyenv.kwsplat_to_hash(ty) {
                                        KwsplatResult::Err(err_ty) => {
                                            self.error(&format!("Cannot keyword splat {}", self.tyenv.describe(err_ty)), &[
                                                Detail::Loc(&self.tyenv.describe(ty), ty.loc()),
                                            ]);
                                            None
                                        }
                                        KwsplatResult::None => {
                                            Some((self.tyenv.instance0(ty.loc().clone(), self.env.object.Symbol),
                                                self.tyenv.new_var(ty.loc().clone())))
                                        }
                                        KwsplatResult::Ok(value_ty) =>
                                            Some((self.tyenv.instance0(ty.loc().clone(), self.env.object.Symbol),
                                                value_ty))
                                    },
                            }
                        }).fold1(|(k1, v1), (k2, v2)|
                            (self.tyenv.union(loc, k1, k2), self.tyenv.union(loc, v1, v2))
                        ).unwrap_or_else(||
                            (self.tyenv.new_var(loc.clone()), self.tyenv.new_var(loc.clone()))
                        );

                    self.create_hash_type(loc, key_ty, value_ty)
                };

                result.map(|()| hash_ty).into_computation()
            }
            Node::DString(ref loc, ref parts) => {
                let string_ty = self.tyenv.instance0(loc.clone(), self.env.object.String);
                let mut comp = Computation::result(string_ty, locals);

                for part in parts {
                    // TODO - verify that each string element responds to #to_s
                    comp = self.seq_process(comp, part);
                }

                comp.seq(&|_, l| Computation::result(string_ty, l))
            }
            Node::Const(..) => {
                match self.env.resolve_cpath(node, self.scope.clone()) {
                    Ok(object) => {
                        let ty = match *object {
                            ConstantEntry::Expression { node: ref const_node, ref scope, .. } => {
                                if let Node::TyCast(_, _, ref ty_node) = **const_node {
                                    let scope_self = self.env.object.metaclass(scope.module);
                                    let type_context = TypeContext::new(scope_self, vec![]);
                                    let ty = self.resolve_type(ty_node, &type_context, scope.clone());
                                    self.tyenv.update_loc(ty, node.loc().clone())
                                } else {
                                    // TODO - don't know the type of this constant
                                    self.tyenv.any(node.loc().clone())
                                }
                            }
                            ConstantEntry::Module { value, .. } => {
                                self.tyenv.instance0(node.loc().clone(), self.env.object.metaclass(value))
                            }
                        };
                        Computation::result(ty, locals)
                    }
                    Err((err_node, message)) => {
                        self.error(message, &[
                            Detail::Loc("here", err_node.loc()),
                        ]);
                        Computation::result(self.tyenv.any(node.loc().clone()), locals)
                    }
                }
            }
            Node::Regexp(ref loc, ref parts, _) => {
                let regexp_ty = self.tyenv.instance0(loc.clone(), self.env.object.Regexp);
                let mut comp = Computation::result(regexp_ty, locals);

                for part in parts {
                    comp = self.seq_process(comp, part);
                }

                comp.seq(&|_, l| Computation::result(regexp_ty, l))
            }
            Node::If(ref loc, ref cond, ref then, ref else_) => {
                let predicate = self.process_node(cond, locals).predicate(cond.loc(), &self.tyenv);

                let then_comp = predicate.truthy.map(|comp|
                    self.seq_process_option(self.converge_results(comp, cond.loc()), then, loc));

                let else_comp = predicate.falsy.map(|comp|
                    self.seq_process_option(self.converge_results(comp, cond.loc()), else_, loc));

                Computation::divergent_option(
                    Computation::divergent_option(then_comp, else_comp),
                    predicate.non_result,
                ).expect("at least one of the computations must be Some")
            }
            Node::OrAsgn(ref loc, ref lhs_node, ref rhs) => {
                self.process_lhs(lhs_node, locals).and_then_comp(|lhs, locals| {
                    let lhs_comp = match lhs {
                        Lhs::Lvar(ref lvar_loc, ref name) => self.lookup_lvar_or_error(lvar_loc, name, locals).into_computation(),
                        Lhs::Simple(_, ty) => Computation::result(ty, locals),
                        Lhs::Send(ref lhs_loc, recv_ty, ref id, ref args) => {
                            self.process_send_dispatch(lhs_loc, recv_ty, id, args.clone(), None, locals)
                        }
                    };

                    let lhs_pred = lhs_comp.predicate(lhs_node.loc(), &self.tyenv);

                    let asgn_comp = lhs_pred.falsy.map(|comp|
                        self.extract_results(self.seq_process(comp, rhs), rhs.loc()).and_then(|rhs_ty, locals| {
                            self.assign(lhs, rhs_ty, locals, loc).map(|()| rhs_ty)
                        }).into_computation()
                    );

                    Computation::divergent_option(lhs_pred.truthy,
                        Computation::divergent_option(asgn_comp, lhs_pred.non_result)).unwrap()
                })
            }
            Node::AndAsgn(ref loc, ref lhs_node, ref rhs) => {
                self.process_lhs(lhs_node, locals).and_then_comp(|lhs, locals| {
                    let lhs_comp = match lhs {
                        Lhs::Lvar(ref lvar_loc, ref name) => self.lookup_lvar_or_error(lvar_loc, name, locals).into_computation(),
                        Lhs::Simple(_, ty) => Computation::result(ty, locals),
                        Lhs::Send(ref lhs_loc, recv_ty, ref id, ref args) => {
                            self.process_send_dispatch(lhs_loc, recv_ty, id, args.clone(), None, locals)
                        }
                    };

                    let lhs_pred = lhs_comp.predicate(lhs_node.loc(), &self.tyenv);

                    let asgn_comp = lhs_pred.truthy.map(|comp|
                        self.extract_results(self.seq_process(comp, rhs), rhs.loc()).and_then(|rhs_ty, locals| {
                            self.assign(lhs, rhs_ty, locals, loc).map(|()| rhs_ty)
                        }).into_computation()
                    );

                    Computation::divergent_option(lhs_pred.falsy,
                        Computation::divergent_option(asgn_comp, lhs_pred.non_result)).unwrap()
                })
            }
            Node::OpAsgn(ref loc, ref lhs_node, ref op, ref rhs) => {
                self.process_lhs(lhs_node, locals).and_then(|lhs, locals| {
                    let lhs_ty = match lhs {
                        Lhs::Lvar(ref lvar_loc, ref name) => self.lookup_lvar_or_error(lvar_loc, name, locals),
                        Lhs::Simple(_, ty) => EvalResult::Ok(ty, locals),
                        Lhs::Send(ref lhs_loc, recv_ty, ref id, ref args) => {
                            let comp = self.process_send_dispatch(lhs_loc, recv_ty, id, args.clone(), None, locals);
                            self.extract_results(comp, lhs_loc)
                        }
                    };

                    lhs_ty.and_then(|lhs_ty, locals| {
                        self.eval_node(rhs, locals).and_then(|rhs_ty, locals| {
                            let args = vec![CallArg::Pass(rhs.loc().clone(), rhs_ty)];
                            let op_comp = self.process_send_dispatch(loc, lhs_ty, op, args, None, locals);
                            self.extract_results(op_comp, loc)
                        })
                    }).and_then(|op_ty, locals| {
                        self.assign(lhs, op_ty, locals, loc).map(|()| op_ty)
                    })
                }).into_computation()
            }
            Node::Or(_, ref lhs, ref rhs) => {
                let lhs_pred = self.process_node(lhs, locals).predicate(lhs.loc(), &self.tyenv);

                let falsy = lhs_pred.falsy.map(|comp| self.seq_process(comp, rhs));

                Computation::divergent_option(
                    Computation::divergent_option(lhs_pred.truthy, falsy),
                    lhs_pred.non_result
                ).expect("at least one of the computations must be Some")
            }
            Node::And(_, ref lhs, ref rhs) => {
                let lhs_pred = self.process_node(lhs, locals).predicate(lhs.loc(), &self.tyenv);

                let truthy = lhs_pred.truthy.map(|comp| self.seq_process(comp, rhs));

                Computation::divergent_option(
                    Computation::divergent_option(lhs_pred.falsy, truthy),
                    lhs_pred.non_result
                ).expect("at least one of the computations must be Some")
            }
            Node::Masgn(ref loc, ref mlhs, ref rhs) => {
                let rhs_comp = match **rhs {
                    Node::Array(ref loc, ref nodes) => self.process_array_tuple(loc, nodes, locals),
                    _ => self.process_node(rhs, locals),
                };

                rhs_comp.seq(&|ty, locals| {
                    self.process_asgn(mlhs, ty, locals, loc).map(|()| ty).into_computation()
                })
            }
            Node::FileLiteral(ref loc) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.String), locals)
            }
            Node::NthRef(ref loc, _) => {
                // TODO perhaps analyse regex to figure out what nthrefs are
                // always present:
                let ty = self.tyenv.nillable(loc,
                    self.tyenv.instance0(loc.clone(), self.env.object.String));

                Computation::result(ty, locals)
            }
            Node::Ensure(ref loc, ref body, ref ensure) => {
                let body_result = self.process_option_node(loc, body.as_ref().map(Rc::as_ref), locals.autopin())
                    .map_locals(&|l| l.unautopin());

                body_result.seq(&|ty, l| {
                    let uncertain_locals = self.merge_locals(locals.clone(), l);

                    self.process_option_node(loc, ensure.as_ref().map(Rc::as_ref), uncertain_locals).seq(&|_, l| {
                        Computation::result(ty, l)
                    })
                })
            }
            Node::Rescue(ref loc, ref body, ref resbodies, ref else_) => {
                let body_comp = self.process_option_node(loc, body.as_ref().map(Rc::as_ref), locals.autopin())
                    .map_locals(&|l| l.unautopin());

                let uncertain_comp = body_comp.seq(&|ty, l|
                    Computation::result(ty, self.merge_locals(locals.clone(), l)));

                let rescue_comps = resbodies.iter().map(|resbody| {
                    self.seq_process(uncertain_comp.clone(), resbody)
                }).collect::<Vec<_>>();

                let else_comp = match else_.as_ref() {
                    Some(else_body) => self.seq_process(body_comp, else_body),
                    None => body_comp,
                };

                rescue_comps.into_iter().fold(else_comp, Computation::divergent)
            }
            Node::Resbody(ref loc, ref classes, ref var, ref body) => {
                let ex_type = match classes.as_ref().map(Rc::as_ref) {
                    Some(&Node::Array(ref loc, ref nodes)) => {
                        self.extract_results(self.process_array_tuple(loc, nodes, locals), loc).map(&|tuple_ty: TypeRef<'ty, 'object>| {
                            let mut tys = Vec::new();

                            if let Type::Tuple { ref lead, ref splat, ref post, .. } = *tuple_ty {
                                for ty in lead {
                                    tys.push(*ty);
                                }

                                if let Some(ty) = *splat {
                                    tys.push(ty);
                                }

                                for ty in post {
                                    tys.push(*ty);
                                }
                            } else {
                                panic!("expected process_array_tuple to return a tuple");
                            }

                            tys.into_iter().filter_map(|ty| {
                                if let Type::Instance { class: &RubyObject::Metaclass { of, .. }, .. } = *ty {
                                    Some(of)
                                } else {
                                    self.error("Expected class or module", &[
                                        Detail::Loc(&self.tyenv.describe(ty), ty.loc()),
                                    ]);
                                    None
                                }
                            }).map(|class| {
                                self.create_instance_type(loc, class, Vec::new())
                            }).fold1(|a, b| {
                                self.tyenv.union(loc, a, b)
                            }).unwrap_or_else(||
                                self.tyenv.new_var(loc.clone())
                            )
                        })
                    },
                    Some(other) => panic!("unexpected node type in resbody class list: {:?}", other),
                    None => EvalResult::Ok(self.tyenv.instance0(loc.clone(), self.env.object.StandardError), locals)
                };

                ex_type.and_then(&|ex_type, locals| {
                    match var.as_ref() {
                        Some(var) => self.process_asgn(var, ex_type, locals, loc),
                        None => EvalResult::Ok((), locals),
                    }
                }).and_then_comp(|(), locals| {
                    self.process_option_node(loc, body.as_ref().map(Rc::as_ref), locals)
                })
            }
            Node::Case(ref loc, ref scrut, ref whens, ref else_) => {
                let scrut_ev = match *scrut {
                    Some(ref scrut) => self.eval_node(scrut, locals).map(Some),
                    None => EvalResult::Ok(None, locals),
                };

                scrut_ev.and_then(|scrut, locals| {
                    let nil_ty = self.tyenv.nil(loc.clone());
                    let init_comp = ComputationPredicate::result(None, Some(Computation::result(nil_ty, locals)));

                    let (else_comp, out_comp) = whens.iter().fold((init_comp, None), |(comp, out_comp), when| {
                        let (when_exprs, then) =
                            if let Node::When(_, ref when_exprs, ref then) = **when {
                                (when_exprs, then.as_ref().map(Rc::as_ref))
                            } else {
                                panic!("expected When in Case whens");
                            };

                        let when_exprs_loc = when_exprs[0].loc().join(when_exprs.last().unwrap().loc());

                        let when_comp = when_exprs.iter().fold(comp, |comp, when_expr| {
                            comp.seq_falsy(|falsy_comp| {
                                let cond_comp = falsy_comp.seq(&|_, l| {
                                    self.process_node(when_expr, l)
                                });

                                let cond_comp = match scrut {
                                    Some(scrut_ty) => self.extract_results(cond_comp, when_expr.loc()).and_then_comp(|cond_ty, locals| {
                                        self.process_send_dispatch(when_expr.loc(), cond_ty,
                                            &Id(when_expr.loc().clone(), "===".to_owned()),
                                            vec![CallArg::Pass(scrut_ty.loc().clone(), scrut_ty)],
                                            None, locals)
                                    }),
                                    None => cond_comp,
                                };

                                cond_comp.predicate(when_expr.loc(), &self.tyenv)
                            })
                        });

                        let comp = ComputationPredicate {
                            truthy: None,
                            falsy: when_comp.falsy,
                            non_result: when_comp.non_result,
                        };

                        let then_comp = when_comp.truthy.map(|truthy_comp| {
                            self.converge_results(truthy_comp, &when_exprs_loc).seq(&|_, l| {
                                self.process_option_node(when.loc(), then, l)
                            })
                        });

                        (comp, Computation::divergent_option(out_comp, then_comp))
                    });

                    assert!(else_comp.truthy.is_none());

                    let non_result_comp = else_comp.non_result;

                    let else_comp = else_comp.falsy.map(|comp|
                        self.extract_results(comp, loc).and_then(|_, l|
                            match *else_ {
                                None => EvalResult::Ok(None, l),
                                Some(ref else_) => self.eval_node(&else_, l).map(Some),
                            }))
                        .map(|ev| ev.map(|ty| ty.unwrap_or(nil_ty)))
                        .map(|ev| ev.into_computation());

                    let comp = Computation::divergent_option(out_comp,
                        Computation::divergent_option(non_result_comp, else_comp)).unwrap();

                    self.extract_results(comp, loc)
                }).into_computation()
            }
            Node::IRange(ref loc, ref begin, ref end) |
            Node::ERange(ref loc, ref begin, ref end) => {
                self.process_node(begin, locals).seq(&|begin_ty, locals| {
                    self.process_node(end, locals).seq(&|end_ty, locals| {
                        // TODO the Range class needs type constraints to make
                        // sure the two values can actually be compared
                        let ty = self.create_instance_type(loc,
                            self.env.object.range_class(),
                            vec![begin_ty, end_ty]);

                        Computation::result(ty, locals)
                    })
                })
            }
            Node::Defined(ref loc, ref expr) => {
                self.process_node(expr, locals).seq(&|_, l| {
                    // TODO actually implement the logic for defined?() and see
                    // if we can return either nil or String statically
                    let ty = self.tyenv.nillable(loc,
                        self.tyenv.instance0(loc.clone(), self.env.object.String));

                    Computation::result(ty, l)
                })
            }
            Node::Gvar(ref loc, ref name) => {
                let ty = match name.as_ref() {
                    "$$" => self.tyenv.instance0(loc.clone(), self.env.object.Integer),
                    _ => {
                        self.error("Unknown global variable", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.any(loc.clone())
                    }
                };

                Computation::result(ty, locals)
            }
            _ => panic!("node: {:?}", node),
        }
    }
}
