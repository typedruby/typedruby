use std::rc::Rc;
use std::collections::HashMap;
use typecheck::flow::{Computation, Locals, LocalEntry, LocalEntryMerge, ComputationPredicate};
use typecheck::types::{Arg, TypeEnv, Type, Prototype};
use object::{Scope, RubyObject, MethodEntry};
use ast::{Node, Loc, Id};
use environment::Environment;
use errors::Detail;
use typed_arena::Arena;
use util::Or;
use typecheck::call;
use typecheck::call::{CallArg, ArgError};

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

enum HashEntry<'ty, 'object: 'ty> {
    Symbol(Id, &'ty Type<'ty, 'object>),
    Pair(&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>),
    Kwsplat(&'ty Type<'ty, 'object>),
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
    Literal { loc: Loc, args: Rc<Node>, body: Option<Rc<Node>> },
}

impl BlockArg {
    pub fn loc(&self) -> &Loc {
        match *self {
            BlockArg::Pass { ref loc, .. } => loc,
            BlockArg::Literal { ref loc, .. } => loc,
        }
    }
}

enum EvalResult<'ty, 'object: 'ty, T> {
    Ok(T, Locals<'ty, 'object>),
    Both(T, Locals<'ty, 'object>, Computation<'ty, 'object>),
    NonResult(Computation<'ty, 'object>)
}

impl<'ty, 'object, T> EvalResult<'ty, 'object, T> {
    fn map<F, U>(self, mut f: F) -> EvalResult<'ty, 'object, U>
        where F : FnMut(T) -> U
    {
        match self {
            EvalResult::Ok(val, locals) => EvalResult::Ok(f(val), locals),
            EvalResult::Both(val, locals, non_result) => EvalResult::Both(f(val), locals, non_result),
            EvalResult::NonResult(non_result) => EvalResult::NonResult(non_result),
        }
    }

    fn and_then<F, U>(self, mut f: F) -> EvalResult<'ty, 'object, U>
        where F : FnMut(T, Locals<'ty, 'object>) -> EvalResult<'ty, 'object, U>
    {
        match self {
            EvalResult::Ok(val, locals) => f(val, locals),
            EvalResult::Both(val, locals, non_result) => {
                match f(val, locals) {
                    EvalResult::Ok(val, locals) =>
                        EvalResult::Both(val, locals, non_result),
                    EvalResult::Both(val, locals, other_non_result) =>
                        EvalResult::Both(val, locals,
                            Computation::divergent(non_result, other_non_result)),
                    EvalResult::NonResult(other_non_result) =>
                        EvalResult::NonResult(
                            Computation::divergent(non_result, other_non_result)),
                }
            }
            EvalResult::NonResult(non_result) => EvalResult::NonResult(non_result),
        }
    }

    fn if_not<F>(self, mut f: F) -> EvalResult<'ty, 'object, T>
        where F : FnMut()
    {
        if let EvalResult::NonResult(_) = self {
            f();
        }

        self
    }
}

impl<'ty, 'object> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>> {
    fn into_computation(self) -> Computation<'ty, 'object> {
        match self {
            EvalResult::Ok(ty, locals) => Computation::result(ty, locals),
            EvalResult::Both(ty, locals, comp) => Computation::divergent(Computation::result(ty, locals), comp),
            EvalResult::NonResult(comp) => comp,
        }
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

        let (annotation_status, prototype, locals) = self.resolve_prototype(prototype_node, Locals::new(), &self.type_context, self.scope.clone());

        match annotation_status {
            AnnotationStatus::Empty |
            AnnotationStatus::Untyped =>
                return,
            AnnotationStatus::Partial => {
                self.error("Partial type signatures are not permitted in method definitions", &[
                    Detail::Loc("all arguments and return value must be annotated", prototype_node.loc()),
                ]);
                return;
            },
            AnnotationStatus::Typed => {},
        };

        // don't typecheck a method if it has no body
        if let Some(ref body_node) = *body {
            self.process_node(body_node, locals).terminate(&|ty|
                if let Prototype::Typed { retn, .. } = *prototype {
                    self.compatible(retn, ty, None)
                }
            );
        }
    }

    fn create_instance_type(&self, loc: &Loc, class: &'object RubyObject<'object>, mut type_parameters: Vec<&'ty Type<'ty, 'object>>) -> &'ty Type<'ty, 'object> {
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

                    self.tyenv.new_var(cpath.loc().clone())
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

                self.tyenv.new_var(cpath.loc().clone())
            }
        }
    }

    fn create_array_type(&self, loc: &Loc, element_type: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        self.tyenv.instance(loc.clone(), self.env.object.array_class(), vec![element_type])
    }

    fn create_hash_type(&self, loc: &Loc, key_type: &'ty Type<'ty, 'object>, value_type: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        self.tyenv.instance(loc.clone(), self.env.object.hash_class(), vec![key_type, value_type])
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
            Node::TyHash(ref loc, ref key, ref value) => {
                self.create_hash_type(loc,
                    self.resolve_type(key, context, scope.clone()),
                    self.resolve_type(value, context, scope))
            },
            Node::TyProc(ref loc, ref prototype) => {
                self.tyenv.alloc(Type::Proc {
                    loc: loc.clone(),
                    proto: self.resolve_prototype(prototype, Locals::new(), context, scope).1,
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

                self.tyenv.tuple(loc.clone(), tys)
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

                let tuple_ty = self.tyenv.tuple(loc.clone(), mlhs_types);

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

    fn resolve_prototype(&self, node: &Node, locals: Locals<'ty, 'object>, context_: &TypeContext<'ty, 'object>, scope: Rc<Scope<'object>>)
        -> (AnnotationStatus, Rc<Prototype<'ty, 'object>>, Locals<'ty, 'object>)
    {
        let mut context = context_.clone();

        let (mut status, args_node, return_type) = match *node {
            Node::Prototype(_, ref genargs, ref args, ref ret) => {
                let mut status = AnnotationStatus::empty();

                if let Some(ref genargs_) = *genargs {
                    if let Node::TyGenargs(_, ref gendeclargs) = **genargs_ {
                        for gendeclarg in gendeclargs {
                            if let Node::TyGendeclarg(ref loc, ref name) = **gendeclarg {
                                context.type_names.insert(name.clone(), self.tyenv.new_var(loc.clone()));
                            }
                        }
                    }

                    status.append_into(AnnotationStatus::Typed);
                }

                match *ret {
                    Some(ref type_node) =>
                        (status.append(AnnotationStatus::Typed), &**args, self.resolve_type(type_node, &context, scope.clone())),
                    None =>
                        (status.append(AnnotationStatus::Untyped), &**args, self.tyenv.new_var(node.loc().clone())),
                }
            },
            Node::Args(..) => {
                (AnnotationStatus::Untyped, node, self.tyenv.new_var(node.loc().clone()))
            },
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
            let (arg_status, arg, locals_) = self.resolve_arg(arg_node, locals, &context, scope.clone());
            status.append_into(arg_status);
            args.push(arg);
            locals = locals_;
        }

        (status, Rc::new(Prototype::Typed { loc: node.loc().clone(), args: args, retn: return_type }), locals)
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

    fn process_array_tuple(&self, loc: &Loc, exprs: &[Rc<Node>], locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        use slice_util::View;

        #[derive(Debug)]
        enum TupleElement<'ty, 'object: 'ty> {
            Value(&'ty Type<'ty, 'object>),
            Splat(&'ty Type<'ty, 'object>),
        };

        let mut elements = Vec::new();
        let mut result = EvalResult::Ok((), locals);

        for expr in exprs {
            let (splat, node) = match **expr {
                Node::Splat(_, Some(ref node)) => (true, node),
                _ => (false, expr),
            };

            result = result.and_then(|(), locals| {
                self.extract_results(self.process_node(node, locals), expr.loc())
            }).map(|ty| {
                if splat {
                    match *ty {
                        Type::Tuple { ref lead, ref splat, ref post, .. } => {
                            for lead_ty in lead {
                                elements.push(TupleElement::Value(lead_ty));
                            }

                            if let Some(splat_ty) = *splat {
                                elements.push(TupleElement::Splat(splat_ty));
                            }

                            for post_ty in post {
                                elements.push(TupleElement::Value(post_ty));
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
            let mut v = View(elements.as_slice());

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

            self.tyenv.alloc(Type::Tuple {
                loc: loc.clone(),
                lead: lead_types,
                splat: splat_type,
                post: post_types,
            })
        }).into_computation()
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

                self.resolve_prototype(&prototype_node, Locals::new(), &type_context, scope.clone()).1
            }
            MethodEntry::AttrReader { ref ivar, .. } => {
                Rc::new(match self.lookup_ivar(ivar, &type_context) {
                    Some(ivar_type) => Prototype::Typed { loc: loc.clone(), args: vec![], retn: ivar_type },
                    None => Prototype::Untyped { loc: loc.clone() },
                })
            }
            MethodEntry::AttrWriter { ref ivar, ref node } => {
                Rc::new(match self.lookup_ivar(ivar, &type_context) {
                    Some(ivar_type) =>
                        Prototype::Typed { loc: loc.clone(), args: vec![Arg::Required { ty: ivar_type, loc: node.loc().clone() }], retn: ivar_type },
                    None => Prototype::Untyped { loc: loc.clone() },
                })
            }
            MethodEntry::Untyped => self.tyenv.any_prototype(loc.clone()),
            MethodEntry::IntrinsicClassNew => {
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

                        let proto = self.prototype_from_method_entry(loc, &initialize_method, initialize_type_context);

                        match *proto {
                            Prototype::Untyped { .. } => proto.clone(),
                            Prototype::Typed { ref loc, ref args, .. } => Rc::new(Prototype::Typed { loc: loc.clone(), args: args.clone(), retn: instance_type }),
                        }
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
        }
    }

    fn prototypes_for_invocation(&self, recv_loc: Option<&Loc>, recv_type: &'ty Type<'ty, 'object>, id: &Id) -> Vec<Rc<Prototype<'ty, 'object>>> {
        let degraded_recv_type = self.tyenv.degrade_to_instance(recv_type);

        let loc = match recv_loc {
            Some(recv_loc) => recv_loc.join(&id.0),
            None => id.0.clone(),
        };

        match *degraded_recv_type {
            Type::Instance { class, type_parameters: ref tp, .. } => {
                match self.env.object.lookup_method(class, &id.1) {
                    Some(method) => {
                        vec![self.prototype_from_method_entry(&loc, &method, TypeContext::new(class, tp.clone()))]
                    }
                    None => Vec::new(),
                }
            }
            Type::Proc { .. } => {
                match self.env.object.lookup_method(self.env.object.Proc, &id.1) {
                    Some(method) => vec![self.prototype_from_method_entry(&loc, &method, TypeContext::new(&self.env.object.Proc, Vec::new()))],
                    None => Vec::new(),
                }
            }
            Type::Union { ref types, .. } => {
                types.iter().flat_map(|ty| {
                    let prototypes = self.prototypes_for_invocation(recv_loc, ty, id);

                    if prototypes.is_empty() {
                        let message = format!("Union member {} does not respond to #{}", self.tyenv.describe(ty), &id.1);
                        self.error(&message, &[
                            Detail::Loc(&self.tyenv.describe(recv_type), recv_type.loc()),
                        ]);
                    }

                    prototypes
                }).collect()
            }
            Type::Any { .. } => vec![self.tyenv.any_prototype(id.0.clone())],
            Type::TypeParameter { ref name, .. } => {
                self.error(&format!("Type parameter {} is of unknown type", name), &[
                    Detail::Loc("in receiver of this invocation", &loc),
                ]);

                vec![]
            }
            Type::Var { id, .. } => {
                self.error(&format!("Type of receiver is not known at this point"), &[
                    Detail::Loc(&format!("t{}", id), recv_loc.expect("self type should never be an unresolved type variable")),
                ]);

                vec![]
            }
            Type::KeywordHash { .. } => panic!("should have degraded to instance"),
            Type::Tuple { .. } => panic!("should have degraded to instance"),
            Type::LocalVariable { .. } => panic!("should never remain after prune"),
        }
    }

    fn process_local_merges(&self, merges: Vec<LocalEntryMerge<'ty, 'object>>) {
        for merge in merges {
            match merge {
                LocalEntryMerge::Ok(_) => {},
                LocalEntryMerge::MustMatch(_, to, from) => self.compatible(to, from, None),
            }
        }
    }

    fn process_and_extract(&self, node: &Node, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>>
    {
        let comp = self.process_node(node, locals);
        self.extract_results(comp, node.loc())
    }

    fn extract_results(&self, comp: Computation<'ty, 'object>, loc: &Loc) -> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>> {
        let mut merges = Vec::new();
        let result = comp.extract_results(loc, &self.tyenv, &mut merges);
        self.process_local_merges(merges);

        match result {
            Or::Left((ty, locals)) => EvalResult::Ok(ty, locals),
            Or::Both((ty, locals), comp) => EvalResult::Both(ty, locals, comp),
            Or::Right(comp) => EvalResult::NonResult(comp),
        }
    }

    fn converge_results(&self, comp: Computation<'ty, 'object>, loc: &Loc) -> Computation<'ty, 'object> {
        let mut merges = Vec::new();
        let comp = comp.converge_results(loc, &self.tyenv, &mut merges);
        self.process_local_merges(merges);
        comp
    }

    fn process_call_arg(&self, node: &Node, locals: Locals<'ty, 'object>) -> EvalResult<'ty, 'object, CallArg<'ty, 'object>> {
        match *node {
            Node::Splat(_, ref n) =>
                self.process_and_extract(n.as_ref().expect("splat in call arg must have node"), locals).map(|ty|
                    CallArg::Splat(node.loc().clone(), ty)
                ),
            _ =>
                self.process_and_extract(node, locals).map(|ty|
                    CallArg::Pass(node.loc().clone(), ty)
                ),
        }
    }

    fn prototype_from_procish_type(&self, procish_ty: &'ty Type<'ty, 'object>)
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

    fn infer_symbol_as_proc_type(&self, proto_block_ty: &'ty Type<'ty, 'object>, mid: &str, loc: &Loc)
        -> &'ty Type<'ty, 'object>
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

        let (proto_args, proto_retn, proto_loc) = if let Prototype::Typed { ref args, ref retn, ref loc } = *proto {
            (args, retn, loc)
        } else {
            self.error("Can't infer type for symbol-as-proc", &[
                Detail::Loc("passed here", loc),
                Detail::Loc("because the block defined in the method prototype is untyped", proto_block_ty.loc()),
            ]);

            return self.tyenv.new_var(loc.clone());
        };

        fn recv_ty_from_arg<'ty, 'object: 'ty>(arg: Option<&Arg<'ty, 'object>>) -> Option<&'ty Type<'ty, 'object>> {
            match arg {
                Some(&Arg::Required { ty, .. }) => Some(ty),
                Some(&Arg::Procarg0 { ref arg, .. }) => recv_ty_from_arg(Some(arg)),
                _ => None,
            }
        }

        if let Some(ty) = recv_ty_from_arg(proto_args.first()) {
            let invokee_proc_ty = self.tyenv.alloc(Type::Proc {
                loc: proto_block_ty.loc().clone(),
                proto: Rc::new(Prototype::Typed {
                    loc: proto_loc.clone(),
                    args: proto_args[1..].iter().cloned().collect(),
                    retn: proto_retn,
                })
            });

            let prototypes = self.prototypes_for_invocation(None, ty, &Id(loc.clone(), mid.to_owned()));

            if prototypes.is_empty() {
                self.error(&format!("Could not resolve method #{}", mid), &[
                    Detail::Loc(&format!("on {}", &self.tyenv.describe(ty)), ty.loc()),
                    Detail::Loc("in symbol-as-proc", loc),
                ]);

                return self.tyenv.any(loc.clone());
            }

            if prototypes.len() > 0 {
                for prototype in prototypes {
                    let prototype_ty = self.tyenv.alloc(Type::Proc {
                        loc: prototype.loc().clone(),
                        proto: prototype.clone(),
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

    fn block_type_from_block_pass(&self, proto_block_ty: &'ty Type<'ty, 'object>, node: &Node, locals: Locals<'ty, 'object>)
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

    fn process_block(&self, send_loc: &Loc, block: Option<&BlockArg>, locals: Locals<'ty, 'object>, prototype_block: Option<&'ty Type<'ty, 'object>>)
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

                let (_, block_prototype, block_locals) = self.resolve_prototype(args, block_locals, &self.type_context, self.scope.clone());

                let block_return_type = if let Prototype::Typed { ref retn, .. } = *block_prototype {
                    retn
                } else {
                    self.tyenv.new_var(loc.clone())
                };

                let block_proc_type = self.tyenv.alloc(Type::Proc {
                    loc: loc.clone(),
                    proto: block_prototype,
                });

                self.compatible(proto_block_ty, block_proc_type, None);

                let block_comp = match *body {
                    None => Computation::result(self.tyenv.nil(loc.clone()), block_locals),
                    Some(ref body_node) => self.process_node(body_node, block_locals),
                };

                self.extract_results(block_comp
                    .capture_next(), loc)
                    .and_then(|ty, locals| {
                        self.compatible(block_return_type, ty, None);
                        EvalResult::Ok((), locals.unextend())
                    })
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

    fn process_send_receiver(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>>
    {
        match *recv {
            Some(ref recv_node) => {
                let comp = self.converge_results(self.process_node(recv_node, locals), recv_node.loc());

                if !comp.has_results() {
                    self.warning("Useless method call", &[
                        Detail::Loc("here", &id.0),
                        Detail::Loc("receiver never evaluates to a result", recv_node.loc()),
                    ]);
                }

                self.extract_results(comp, recv_node.loc())
            },
            None => EvalResult::Ok(self.type_context.self_type(&self.tyenv, id.0.clone()), locals),
        }
    }

    fn process_send_args(&self, loc: &Loc, id: &Id, arg_nodes: &[Rc<Node>], locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, Vec<CallArg<'ty, 'object>>>
    {
        let mut args = Vec::new();

        let mut result = EvalResult::Ok((), locals);

        for arg_node in arg_nodes {
            result = result.and_then(|(), locals| {
                self.process_call_arg(arg_node, locals).and_then(|call_arg, locals| {
                    args.push(call_arg);
                    EvalResult::Ok((), locals)
                }).if_not(|| {
                    self.warning("Useless method call", &[
                        Detail::Loc("here", &id.0),
                        Detail::Loc("argument never evaluates to a result", arg_node.loc()),
                    ]);
                })
            })
        }

        match result {
            EvalResult::Ok((), l) => EvalResult::Ok(args, l),
            EvalResult::Both((), l, comp) => EvalResult::Both(args, l, comp),
            EvalResult::NonResult(comp) => EvalResult::NonResult(comp),
        }
    }

    fn process_send_dispatch(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, recv_type: &'ty Type<'ty, 'object>, args: Vec<CallArg<'ty, 'object>>, block: Option<BlockArg>, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        let prototypes = self.prototypes_for_invocation(recv.as_ref().map(|r| r.loc()), recv_type, id);

        if prototypes.is_empty() {
            self.error(&format!("Could not resolve method #{}", &id.1), &[
                Detail::Loc(&format!("on {}", &self.tyenv.describe(recv_type)), recv_type.loc()),
                Detail::Loc("in this invocation", &id.0),
            ]);

            return Computation::result(self.tyenv.any(loc.clone()), locals);
        }

        let mut result_comp = None;

        for proto in prototypes {
            let comp = match *proto {
                Prototype::Typed { args: ref proto_args, ref retn, loc: ref proto_loc } => {
                    let (proto_block, proto_args) = match proto_args.last() {
                        Some(&Arg::Block { ty, .. }) => (Some(ty), &proto_args[..proto_args.len() - 1]),
                        Some(_) | None => (None, proto_args.as_slice()),
                    };

                    let match_result = call::match_prototype_with_invocation(&self.tyenv, proto_args, &args);

                    for match_error in match_result.errors {
                        match match_error {
                            ArgError::TooFewArguments => {
                                self.error("Too few arguments supplied", &[
                                    Detail::Loc("in this invocation", &id.0),
                                    Detail::Loc("for this prototype", proto_loc),
                                ])
                            }
                            ArgError::TooManyArguments(ref loc) => {
                                self.error("Too many arguments supplied", &[
                                    Detail::Loc("from here", loc),
                                    Detail::Loc("in this invocation", &id.0),
                                    Detail::Loc("for this prototype", proto_loc),
                                ])
                            }
                            ArgError::MissingKeyword(ref name) => {
                                self.error(&format!("Missing keyword argument :{}", name), &[
                                    Detail::Loc("in this invocation", &id.0),
                                    Detail::Loc("for this prototype", proto_loc),
                                ])
                            }
                        }
                    }

                    for (proto_ty, pass_ty) in match_result.matches {
                        self.compatible(proto_ty, pass_ty, Some(loc));
                    }

                    let retn_ty = self.tyenv.update_loc(retn, loc.clone());

                    self.process_block(&id.0, block.as_ref(), locals.clone(), proto_block).map(|()| retn_ty).into_computation()
                },
                Prototype::Untyped { .. } =>
                    Computation::result(self.tyenv.any(loc.clone()), locals.clone()),
            };

            result_comp = Computation::divergent_option(result_comp, Some(comp));
        }

        result_comp.unwrap_or_else(|| Computation::result(self.tyenv.any(loc.clone()), locals))
    }

    fn type_for_attr_asgn(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, arg_nodes: &[Rc<Node>], locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>>
    {
        let id = Id(id.0.clone(), id.1.clone() + "=");

        let (recv_type, locals, non_result_comp) = match self.process_send_receiver(loc, recv, &id, locals) {
            EvalResult::Ok(recv_type, locals) => (recv_type, locals, None),
            EvalResult::Both(recv_type, locals, comp) => (recv_type, locals, Some(comp)),
            EvalResult::NonResult(comp) => return EvalResult::NonResult(comp),
        };

        let (args, locals, non_result_comp) = match self.process_send_args(loc, &id, arg_nodes, locals) {
            EvalResult::Ok(args, locals) => (args, locals, non_result_comp),
            EvalResult::Both(args, locals, comp) => (args, locals, Computation::divergent_option(non_result_comp, Some(comp))),
            EvalResult::NonResult(comp) => return EvalResult::NonResult(comp),
        };

        let attr_asgn_ty = self.tyenv.new_var(loc.clone());

        let mut args = args;
        args.push(CallArg::Pass(loc.clone(), attr_asgn_ty));

        let dispatch_comp = self.process_send_dispatch(loc, recv, &id, recv_type, args, None, locals);

        let comp = Computation::divergent_option(Some(dispatch_comp), non_result_comp).unwrap();

        self.extract_results(comp, loc).map(|_| attr_asgn_ty)
    }

    fn process_send(&self, loc: &Loc, recv: &Option<Rc<Node>>, id: &Id, arg_nodes: &[Rc<Node>], block: Option<BlockArg>, locals: Locals<'ty, 'object>)
        -> Computation<'ty, 'object>
    {
        let (recv_type, locals, non_result_comp) = match self.process_send_receiver(loc, recv, id, locals) {
            EvalResult::Ok(recv_type, locals) => (recv_type, locals, None),
            EvalResult::Both(recv_type, locals, comp) => (recv_type, locals, Some(comp)),
            EvalResult::NonResult(comp) => return comp,
        };

        let (args, locals, non_result_comp) = match self.process_send_args(loc, id, arg_nodes, locals) {
            EvalResult::Ok(args, locals) => (args, locals, non_result_comp),
            EvalResult::Both(args, locals, comp) => (args, locals, Computation::divergent_option(non_result_comp, Some(comp))),
            EvalResult::NonResult(comp) => return comp,
        };

        let dispatch_comp = self.process_send_dispatch(loc, recv, id, recv_type, args, block, locals);

        Computation::divergent_option(Some(dispatch_comp), non_result_comp).unwrap()
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

    fn cond_asgn<T>(&self, lhs: &Rc<Node>, rhs: &Rc<Node>, locals: Locals<'ty, 'object>, f: T) -> Computation<'ty, 'object>
        where T : Fn(ComputationPredicate<'ty, 'object>, &Node) -> ComputationPredicate<'ty, 'object>
    {
        let asgn_node = match **lhs {
            Node::Lvassignable(ref loc, ref name) =>
                Node::Lvasgn(loc.join(rhs.loc()), Id(loc.clone(), name.clone()), rhs.clone()),
            Node::Ivar(ref loc, ref name) =>
                Node::Ivasgn(loc.join(rhs.loc()), Id(loc.clone(), name.clone()), rhs.clone()),
            _ =>
                panic!("unknown lhs in cond_asgn: {:?}", lhs),
        };

        let lhs_pred = self.process_node(lhs, locals).predicate(lhs.loc(), &self.tyenv);

        let asgn_pred = f(lhs_pred, &asgn_node);

        Computation::divergent_option(
            Computation::divergent_option(asgn_pred.truthy, asgn_pred.falsy),
            asgn_pred.non_result,
        ).expect("at least one of the computations must be Some")
    }

    fn lookup_ivar(&self, name: &str, type_context: &TypeContext<'ty, 'object>) -> Option<&'ty Type<'ty, 'object>> {
        self.env.object.lookup_ivar(type_context.class, name).map(|ivar|
            self.resolve_type(&ivar.type_node, type_context, ivar.scope.clone()))
    }

    fn lookup_ivar_or_error(&self, id: &Id, type_context: &TypeContext<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        self.lookup_ivar(&id.1, type_context).unwrap_or_else(|| {
            self.error("Use of undeclared instance variable", &[
                Detail::Loc("here", &id.0),
            ]);

            self.tyenv.any(id.0.clone())
        })
    }

    fn assign_lvar(&self, name: &str, ty: &'ty Type<'ty, 'object>, locals: Locals<'ty, 'object>, loc: &Loc)
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

    fn process_lhs(&self, lhs: &Node, locals: Locals<'ty, 'object>)
        -> EvalResult<'ty, 'object, &'ty Type<'ty, 'object>>
    {
        match *lhs {
            Node::Lvassignable(ref loc, ref name) => {
                let lv_ty = self.tyenv.new_var(loc.clone());

                match locals.assign(name.to_owned(), lv_ty) {
                    (None, locals) =>
                        EvalResult::Ok(lv_ty, locals),
                    (Some(existing_lv_ty), locals) =>
                        EvalResult::Ok(existing_lv_ty, locals),
                }
            }
            Node::Ivar(ref loc, ref name) => {
                let iv_ty = self.lookup_ivar_or_error(&Id(loc.clone(), name.clone()), &self.type_context);

                EvalResult::Ok(iv_ty, locals)
            }
            Node::Const(..) => panic!("shouldn't happen"),
            Node::Cvar(..) | Node::Gvar(..) => panic!("TODO"),
            Node::Send(ref loc, ref recv, ref id, ref args) => {
                self.type_for_attr_asgn(loc, recv, id, args, locals)
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

                    let ty = match self.process_lhs(node, locals) {
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

                let tuple = self.tyenv.alloc(Type::Tuple {
                    loc: lhs.loc().clone(),
                    lead: lead_types,
                    splat: splat_type,
                    post: post_types,
                });

                match non_result_comp {
                    Some(comp) => EvalResult::Both(tuple, locals, comp),
                    None => EvalResult::Ok(tuple, locals),
                }
            }
            _ => panic!("unknown node type in lhs: {:?}", lhs),
        }
    }

    fn process_masgn(&self, lhs: &Node, rty: &'ty Type<'ty, 'object>, locals: Locals<'ty, 'object>, loc: &Loc)
        -> EvalResult<'ty, 'object, ()>
    {
        self.process_lhs(lhs, locals).map(|lhs_ty| {
            self.compatible(lhs_ty, rty, Some(loc));
        })
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
                    self.converge_results(self.seq_process(comp, node), node.loc()))
            }
            Node::Kwbegin(ref loc, ref node) => {
                match *node {
                    Some(ref n) => self.process_node(n, locals),
                    None => Computation::result(self.tyenv.nil(loc.clone()), locals),
                }
            }
            Node::Lvasgn(ref asgn_loc, Id(_, ref lvar_name), ref expr) => {
                self.process_node(expr, locals).seq(&|expr_ty, l| {
                    let l = self.assign_lvar(lvar_name, expr_ty, l, asgn_loc);

                    let lvar_ty = self.tyenv.local_variable(asgn_loc.clone(), lvar_name.clone(), expr_ty);

                    Computation::result(lvar_ty, l)
                })
            }
            Node::Lvar(ref loc, ref name) => {
                let (ty, locals) = locals.lookup(name);

                let ty = match ty {
                    LocalEntry::Bound(ty) |
                    LocalEntry::Pinned(ty) => self.tyenv.local_variable(loc.clone(), name.clone(), ty),
                    LocalEntry::ConditionallyPinned(ty) => {
                        self.tyenv.nillable(loc, self.tyenv.local_variable(loc.clone(), name.clone(), ty))
                    }
                    LocalEntry::Unbound => {
                        self.error("Use of uninitialised local variable", &[
                            Detail::Loc("here", loc),
                        ]);

                        self.tyenv.nil(loc.clone())
                    }
                };

                Computation::result(ty, locals)
            }
            // same as Lvar but does not error on use of uninitialised local
            // variable since we'll be assigning it straight away anyway:
            Node::Lvassignable(ref loc, ref name) => {
                let (ty, locals) = locals.lookup(name);

                let ty = match ty {
                    LocalEntry::Bound(ty) |
                    LocalEntry::Pinned(ty) => self.tyenv.local_variable(loc.clone(), name.clone(), ty),
                    LocalEntry::ConditionallyPinned(ty) => {
                        self.tyenv.nillable(loc, self.tyenv.local_variable(loc.clone(), name.clone(), ty))
                    }
                    LocalEntry::Unbound => self.tyenv.nil(loc.clone())
                };

                Computation::result(ty, locals)
            }
            Node::Ivar(ref loc, ref name) => {
                let ty = self.lookup_ivar_or_error(&Id(loc.clone(), name.clone()), &self.type_context);

                Computation::result(ty, locals)
            }
            Node::Ivasgn(ref loc, ref ivar, ref expr) => {
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
                let comp = match exprs.len() {
                    0 => Computation::result(self.tyenv.nil(loc.clone()), locals),
                    1 => self.process_node(exprs.first().unwrap(), locals),
                    _ => {
                        let loc = exprs[0].loc().join(exprs.last().unwrap().loc());
                        self.process_array_tuple(&loc, exprs, locals)
                    }
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
                if let Node::Send(_, ref recv, ref mid, ref args) = **send {
                    let mut block_loc = block_args.loc().clone();

                    if let Some(ref block_body) = *block_body {
                        block_loc = block_loc.join(block_body.loc());
                    }

                    let block = BlockArg::Literal { loc: block_loc, args: block_args.clone(), body: block_body.clone() };

                    self.process_send(loc, recv, mid, args, Some(block), locals)
                } else {
                    panic!("expected Node::Send inside Node::Block")
                }
            }
            Node::Hash(ref loc, ref pairs) => {
                let mut result = EvalResult::Ok((), locals);
                let mut entries = Vec::new();

                for pair in pairs {
                    match **pair {
                        Node::Pair(_, ref key, ref value) => {
                            result = result.and_then(|(), locals|
                                self.extract_results(self.process_node(key, locals), loc).if_not(|| {
                                    self.warning("Expression never evalutes to a result", &[
                                        Detail::Loc("here", key.loc()),
                                    ])
                                })
                            ).and_then(|key_ty, locals| {
                                self.extract_results(self.process_node(value, locals), loc).if_not(|| {
                                    self.warning("Expression never evalutes to a result", &[
                                        Detail::Loc("here", value.loc()),
                                    ])
                                }).map(|value_ty| {
                                    (value_ty, key_ty)
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
                                self.extract_results(self.process_node(splat, locals), loc).if_not(|| {
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
                        HashEntry::Kwsplat(ty) => {
                            if let Type::KeywordHash { .. } = *self.tyenv.prune(ty) {
                                true
                            } else {
                                false
                            }
                        },
                        HashEntry::Pair(..) => false,
                    }
                });

                let hash_ty = if is_keyword_hash {
                    let mut keywords = Vec::new();

                    for entry in entries {
                        match entry {
                            HashEntry::Symbol(Id(_, key), value) => keywords.push((key, value)),
                            HashEntry::Kwsplat(kw_ty) => {
                                if let Type::KeywordHash { keywords: ref splat_keywords, .. } = *self.tyenv.prune(kw_ty) {
                                    for &(ref key, value) in splat_keywords {
                                        keywords.push((key.clone(), value));
                                    }
                                } else {
                                    panic!()
                                }
                            },
                            _ => panic!(),
                        }
                    }

                    self.tyenv.keyword_hash(loc.clone(), keywords)
                } else {
                    let key_ty = self.tyenv.new_var(loc.clone());
                    let value_ty = self.tyenv.new_var(loc.clone());

                    for entry in entries {
                        match entry {
                            HashEntry::Symbol(Id(sym_loc, _), value) => {
                                self.compatible(key_ty, self.tyenv.instance0(sym_loc, self.env.object.Symbol), Some(loc));
                                self.compatible(value_ty, value, Some(loc));
                            }
                            HashEntry::Pair(key, value) => {
                                self.compatible(key_ty, key, Some(loc));
                                self.compatible(value_ty, value, Some(loc));
                            }
                            HashEntry::Kwsplat(_) => {
                                panic!("TODO")
                            }
                        }
                    }

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
                        let ty = match object {
                            &RubyObject::Object { ref type_node, ref type_scope, .. } => {
                                let scope_self = self.env.object.metaclass(type_scope.module);
                                let type_context = TypeContext::new(scope_self, vec![]);
                                self.resolve_type(type_node, &type_context, type_scope.clone())
                            }
                            _ => {
                                self.tyenv.instance0(node.loc().clone(), self.env.object.metaclass(object))
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
            Node::OrAsgn(_, ref lhs, ref rhs) => {
                self.cond_asgn(lhs, rhs, locals, |pred, asgn_node| ComputationPredicate {
                    falsy: pred.falsy.map(|comp| self.seq_process(comp, asgn_node)),
                    ..pred
                })
            }
            Node::AndAsgn(_, ref lhs, ref rhs) => {
                self.cond_asgn(lhs, rhs, locals, |pred, asgn_node| ComputationPredicate {
                    truthy: pred.truthy.map(|comp| self.seq_process(comp, asgn_node)),
                    ..pred
                })
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
                    self.process_masgn(mlhs, ty, locals, loc).map(|()| ty).into_computation()
                })
            }
            Node::FileLiteral(ref loc) => {
                Computation::result(self.tyenv.instance0(loc.clone(), self.env.object.String), locals)
            }
            _ => panic!("node: {:?}", node),
        }
    }
}
