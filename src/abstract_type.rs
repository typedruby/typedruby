use std::rc::Rc;

use ast::{Id, Loc, Node};
use environment::Environment;
use errors::Detail;
use object::{Scope, RubyObject, ConstantEntry};

pub type TypeNodeRef<'object> = Rc<TypeNode<'object>>;

#[derive(Debug)]
pub enum TypeScope<'object> {
    Constant {
        scope: Rc<Scope<'object>>,
    },
    Param {
        parent: Rc<TypeScope<'object>>,
        name: String,
    }
}

impl<'object> TypeScope<'object> {
    pub fn new(scope: Rc<Scope<'object>>, module: &'object RubyObject<'object>)
        -> Rc<TypeScope<'object>>
    {
        let scope = Rc::new(TypeScope::Constant { scope });

        module.type_parameters().iter().fold(scope,
            |scope, &Id(_, ref param)| Self::extend(scope, param.clone()))
    }

    pub fn extend(parent: Rc<TypeScope<'object>>, name: String) -> Rc<TypeScope<'object>> {
        Rc::new(TypeScope::Param { parent, name })
    }

    pub fn is_param(&self, name: &str) -> bool {
        match *self {
            TypeScope::Constant { ref scope } =>
                scope.module.type_parameters()
                    .iter()
                    .any(|&Id(_, ref param_name)| param_name == name),
            TypeScope::Param { name: ref param_name, .. }
                if param_name == name => true,
            TypeScope::Param { ref parent, .. } =>
                parent.is_param(name),
        }
    }

    pub fn constant_scope(&self) -> Rc<Scope<'object>> {
        match *self {
            TypeScope::Constant { ref scope } => scope.clone(),
            TypeScope::Param { ref parent, .. } => parent.constant_scope(),
        }
    }
}

#[derive(Debug)]
pub enum TypeNode<'object> {
    Instance {
        loc: Loc,
        class: &'object RubyObject<'object>,
        type_parameters: Vec<TypeNodeRef<'object>>,
    },
    Tuple {
        loc: Loc,
        lead: Vec<TypeNodeRef<'object>>,
        splat: Option<TypeNodeRef<'object>>,
        post: Vec<TypeNodeRef<'object>>,
    },
    Union {
        loc: Loc,
        types: Vec<TypeNodeRef<'object>>,
    },
    Any {
        loc: Loc,
    },
    TypeParameter {
        loc: Loc,
        name: String,
    },
    Proc {
        loc: Loc,
        proto: Prototype<'object>,
    },
    SpecialSelf {
        loc: Loc,
    },
    SpecialInstance {
        loc: Loc,
    },
    SpecialClass {
        loc: Loc,
    },
    Error {
        loc: Loc,
    },
}

#[derive(Debug)]
pub struct ArgExpr<'object> {
    pub expr: Rc<Node>,
    pub scope: Rc<Scope<'object>>,
}

#[derive(Debug)]
pub enum ArgLhs {
    Lvar { name: Id },
    Mlhs { loc: Loc, items: Vec<ArgLhs> },
}

#[derive(Debug)]
pub enum ArgNode<'object> {
    Required {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        lhs: ArgLhs,
    },
    Procarg0 {
        loc: Loc,
        arg: Box<ArgNode<'object>>,
    },
    Optional {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Id,
        default: ArgExpr<'object>,
    },
    Rest {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Option<Id>,
    },
    Kwarg {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Id,
    },
    Kwoptarg {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Id,
        default: ArgExpr<'object>,
    },
    Kwrest {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Option<Id>,
    },
    Block {
        loc: Loc,
        ty: Option<TypeNodeRef<'object>>,
        name: Option<Id>,
    },
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum AnnotationStatus {
    Empty,
    Typed,
    Partial,
    Untyped,
}

impl AnnotationStatus {
    pub fn append(self, other: AnnotationStatus) -> AnnotationStatus {
        match (self, other) {
            (AnnotationStatus::Typed, AnnotationStatus::Typed) => AnnotationStatus::Typed,
            (AnnotationStatus::Untyped, AnnotationStatus::Untyped) => AnnotationStatus::Untyped,
            (AnnotationStatus::Empty, _) => other,
            _ => AnnotationStatus::Partial,
        }
    }
}

#[derive(Debug)]
pub struct Prototype<'object> {
    pub loc: Loc,
    pub type_vars: Vec<Id>,
    pub type_constraints: Vec<TypeConstraint<'object>>,
    pub args: Vec<ArgNode<'object>>,
    pub retn: Option<TypeNodeRef<'object>>,
}

impl<'object> Prototype<'object> {
    pub fn resolve(proto_loc: &Loc, node: Option<&Node>, env: &Environment<'object>, scope: Rc<TypeScope<'object>>)
        -> (AnnotationStatus, Prototype<'object>)
    {
        match node {
            Some(proto) => {
                let resolve = ResolveType { env: env, scope: scope };
                let (anno, proto) = resolve.resolve_prototype(proto);

                if let AnnotationStatus::Partial = anno {

                }

                (anno, proto)
            }
            None => {
                let proto = Prototype {
                    loc: proto_loc.clone(),
                    type_vars: vec![],
                    type_constraints: vec![],
                    args: vec![],
                    retn: None,
                };
                (AnnotationStatus::Untyped, proto)
            }
        }
    }
}

#[derive(Debug)]
pub enum TypeConstraint<'object> {
    Compatible { loc: Loc, sub: TypeNodeRef<'object>, super_: TypeNodeRef<'object> },
}

impl<'object> TypeNode<'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            TypeNode::Instance { ref loc, .. } |
            TypeNode::Tuple { ref loc, .. } |
            TypeNode::Union { ref loc, .. } |
            TypeNode::Any { ref loc, .. } |
            TypeNode::TypeParameter { ref loc, .. } |
            TypeNode::SpecialSelf { ref loc, .. } |
            TypeNode::SpecialInstance { ref loc, .. } |
            TypeNode::SpecialClass { ref loc, .. } |
            TypeNode::Proc { ref loc, .. } |
            TypeNode::Error { ref loc, .. } =>
                loc,
        }
    }

    pub fn resolve<'env>(node: &Node, env: &'env Environment<'object>, scope: Rc<TypeScope<'object>>)
        -> TypeNodeRef<'object>
    {
        ResolveType { env: env, scope: scope }.resolve_type(node)
    }
}

struct ResolveType<'env, 'object: 'env> {
    env: &'env Environment<'object>,
    scope: Rc<TypeScope<'object>>,
}

impl<'env, 'object> ResolveType<'env, 'object> {
    fn error(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn resolve_type_name(&self, cpath: &Node) -> Option<&'object RubyObject<'object>> {
        self.env.resolve_cpath(cpath, self.scope.constant_scope()).map(|constant| {
            match *constant {
                ConstantEntry::Expression { .. } => {
                    self.error("Constant mentioned in type name does not reference static class/module", &[
                        Detail::Loc("here", cpath.loc()),
                    ]);

                    None
                }
                ConstantEntry::Module { value, .. } =>
                    Some(value),
            }
        }).unwrap_or_else(|(node, msg)| {
            self.error(msg, &[
                Detail::Loc("here", node.loc()),
            ]);

            None
        })
    }

    fn resolve_class_instance_type(&self, loc: &Loc, type_parameters: &[Rc<Node>]) -> TypeNodeRef<'object> {
        if type_parameters.len() == 0 {
            return Rc::new(TypeNode::Instance {
                loc: loc.clone(),
                class: self.env.object.Class,
                type_parameters: vec![],
            });
        }

        if type_parameters.len() > 1 {
            self.error("Too many type parameters supplied in instantiation of metaclass", &[
                Detail::Loc("from here", type_parameters[1].loc()),
            ]);
        }

        let cpath = if let Node::TyCpath(_, ref cpath) = *type_parameters[0] {
            Some(cpath)
        } else {
            self.error("Type parameter in metaclass instantiation must be constant path", &[
                Detail::Loc("here", type_parameters[0].loc()),
            ]);
            None
        };

        cpath.and_then(|cpath| self.resolve_type_name(cpath))
             .map(|object| self.instance0(loc, self.env.object.metaclass(object)))
             .unwrap_or_else(|| Rc::new(TypeNode::Error { loc: loc.clone() }))
    }

    fn create_instance_type(&self, loc: &Loc, class: &'object RubyObject<'object>, mut type_parameters: Vec<TypeNodeRef<'object>>)
        -> TypeNodeRef<'object>
    {
        let supplied_params = type_parameters.len();
        let expected_params = class.type_parameters().len();

        if supplied_params == 0 && expected_params > 0 {
            self.error("Type referenced is generic but no type parameters were supplied", &[
                Detail::Loc("here", loc),
            ]);

            for _ in 0..expected_params {
                type_parameters.push(Rc::new(TypeNode::Error { loc: loc.clone() }));
            }
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
                type_parameters.push(Rc::new(TypeNode::Error { loc: loc.clone() }));
            }
        } else if supplied_params > expected_params {
            self.error("Too many type parameters supplied in instantiation of generic type", &[
                Detail::Loc("from here", type_parameters[expected_params].loc()),
            ]);

            for _ in 0..(supplied_params - expected_params) {
                type_parameters.pop();
            }
        }

        Rc::new(TypeNode::Instance {
            loc: loc.clone(),
            class: class,
            type_parameters: type_parameters,
        })
    }

    fn resolve_instance_type(&self, loc: &Loc, cpath: &Node, type_parameters: &[Rc<Node>]) -> TypeNodeRef<'object> {
        if let Node::Const(_, None, Id(ref name_loc, ref name)) = *cpath {
            if self.scope.is_param(name) {
                if !type_parameters.is_empty() {
                    self.error("Type parameters were supplied but type mentioned does not take any", &[
                        Detail::Loc("here", name_loc),
                    ]);
                }

                return Rc::new(TypeNode::TypeParameter {
                    loc: name_loc.clone(),
                    name: name.clone(),
                });
            }
        }

        match self.resolve_type_name(cpath) {
            Some(class) if class == self.env.object.Class => {
                self.resolve_class_instance_type(loc, type_parameters)
            }
            Some(class) => {
                let type_parameters = type_parameters.iter().map(|arg|
                    self.resolve_type(arg)
                ).collect();

                self.create_instance_type(loc, class, type_parameters)
            }
            None => {
                Rc::new(TypeNode::Error { loc: cpath.loc().clone() })
            }
        }
    }

    fn instance(&self, loc: &Loc, class: &'object RubyObject<'object>, type_parameters: Vec<TypeNodeRef<'object>>)
        -> TypeNodeRef<'object>
    {
        Rc::new(TypeNode::Instance {
            loc: loc.clone(),
            class: class,
            type_parameters: type_parameters,
        })
    }

    fn instance0(&self, loc: &Loc, class: &'object RubyObject<'object>) -> TypeNodeRef<'object> {
        self.instance(loc, class, vec![])
    }

    pub fn resolve_type(&self, node: &Node) -> TypeNodeRef<'object> {
        match *node {
            Node::TyCpath(ref loc, ref cpath) =>
                self.resolve_instance_type(loc, cpath, &[]),
            Node::TyGeninst(ref loc, ref cpath, ref args) =>
                self.resolve_instance_type(loc, cpath, args),
            Node::TyNil(ref loc) =>
                self.instance0(loc, self.env.object.NilClass),
            Node::TyAny(ref loc) =>
                Rc::new(TypeNode::Any { loc: loc.clone() }),
            Node::TyArray(ref loc, ref element) =>
                self.instance(loc, self.env.object.array_class(),
                    vec![self.resolve_type(element)]),
            Node::TyHash(ref loc, ref key, ref value) =>
                self.instance(loc, self.env.object.hash_class(),
                    vec![
                        self.resolve_type(key),
                        self.resolve_type(value),
                    ]),
            Node::TyProc(ref loc, ref prototype) =>
                Rc::new(TypeNode::Proc {
                    loc: loc.clone(),
                    proto: self.resolve_prototype(prototype).1,
                }),
            Node::TyClass(ref loc) =>
                Rc::new(TypeNode::SpecialClass { loc: loc.clone() }),
            Node::TySelf(ref loc) =>
                Rc::new(TypeNode::SpecialSelf { loc: loc.clone() }),
            Node::TyInstance(ref loc) =>
                Rc::new(TypeNode::SpecialInstance { loc: loc.clone() }),
            Node::TyNillable(ref loc, ref ty) =>
                Rc::new(TypeNode::Union {
                    loc: loc.clone(),
                    types: vec![
                        self.instance0(loc, self.env.object.NilClass),
                        self.resolve_type(ty),
                    ]
                }),
            Node::TyParen(_, ref inner) =>
                self.resolve_type(inner),
            Node::TyOr(ref loc, ref a, ref b) =>
                Rc::new(TypeNode::Union {
                    loc: loc.clone(),
                    types: vec![
                        self.resolve_type(a),
                        self.resolve_type(b),
                    ]
                }),
            Node::TyTuple(ref loc, ref types) =>
                Rc::new(TypeNode::Tuple {
                    loc: loc.clone(),
                    lead: types.iter().map(|n| self.resolve_type(n)).collect(),
                    splat: None,
                    post: vec![],
                }),
            _ =>
                panic!("unknown node type: {:?}", node),
        }
    }

    fn resolve_arg_mlhs(&self, node: &Node) -> Option<ArgLhs> {
        match *node {
            Node::Arg(ref loc, ref name) =>
                Some(ArgLhs::Lvar { name: Id(loc.clone(), name.clone()) }),
            Node::Restarg(ref loc, _) => {
                self.error("Only required arguments are supported in destructuring arguments", &[
                    Detail::Loc("here", loc),
                ]);
                None
            }
            Node::Mlhs(ref loc, ref items) => {
                let items = items.iter().filter_map(|n| self.resolve_arg_mlhs(n)).collect();

                Some(ArgLhs::Mlhs { loc: loc.clone(), items: items })
            }
            _ => panic!("unexpected node type in resolve_arg_mlhs")
        }
    }

    fn resolve_arg(&self, node: &Node) -> (AnnotationStatus, ArgNode<'object>) {
        let arg_loc = node.loc().clone();

        let (anno, ty, arg_node) = match *node {
            Node::TyTypedArg(_, ref type_node, ref arg_node) =>
                (AnnotationStatus::Typed, Some(self.resolve_type(type_node)), arg_node.as_ref()),
            _ =>
                (AnnotationStatus::Untyped, None, node),
        };

        let arg = match *arg_node {
            Node::Procarg0(_, ref arg) => {
                let (anno_inner, arg) = self.resolve_arg(arg);
                let arg = ArgNode::Procarg0 { loc: arg_loc, arg: Box::new(arg) };
                return (anno.append(anno_inner), arg);
            }
            Node::Arg(ref loc, ref name) =>
                ArgNode::Required { loc: arg_loc, ty: ty, lhs: ArgLhs::Lvar { name: Id(loc.clone(), name.clone()) } },
            Node::Blockarg(_, ref name) =>
                ArgNode::Block { loc: arg_loc, ty: ty, name: name.clone() },
            Node::Kwarg(ref loc, ref name) =>
                ArgNode::Kwarg { loc: arg_loc, ty: ty, name: Id(loc.clone(), name.clone()) },
            Node::Kwoptarg(_, ref name, ref expr) =>
                ArgNode::Kwoptarg { loc: arg_loc, ty: ty, name: name.clone(),
                    default: ArgExpr { expr: expr.clone(), scope: self.scope.constant_scope() } },
            Node::Mlhs(..) => {
                match (ty, self.resolve_arg_mlhs(arg_node).unwrap()) {
                    (Some(ty), arg@ArgLhs::Mlhs { .. }) => {
                        self.error("Exterior type annotations on destructuring arguments are not allowed", &[
                            Detail::Loc("here", ty.loc()),
                            Detail::Loc("for this destructuring argument", arg_node.loc()),
                        ]);

                        ArgNode::Required { loc: arg_loc, ty: None, lhs: arg }
                    }
                    (ty, arg) => {
                        ArgNode::Required { loc: arg_loc, ty: ty, lhs: arg }
                    }
                }
            }
            Node::Optarg(_, ref name, ref expr) =>
                ArgNode::Optional { loc: arg_loc, ty: ty, name: name.clone(),
                    default: ArgExpr { expr: expr.clone(), scope: self.scope.constant_scope() } },
            Node::Restarg(_, ref name) =>
                ArgNode::Rest { loc: arg_loc, ty: ty, name: name.clone() },
            Node::Kwrestarg(_, ref name) =>
                ArgNode::Kwrest { loc: arg_loc, ty: ty, name: name.clone() },
            _ => panic!("unexpected node type in resolve_arg"),
        };

        (anno, arg)
    }

    fn resolve_type_constraint(&self, node: &Node, scope: Rc<TypeScope<'object>>)
        -> TypeConstraint<'object>
    {
        match *node {
            Node::TyConSubtype(ref loc, ref sub, ref super_) =>
                TypeConstraint::Compatible {
                    loc: loc.clone(),
                    sub: TypeNode::resolve(sub, self.env, scope.clone()),
                    super_: TypeNode::resolve(super_, self.env, scope.clone()),
                },
            _ => panic!("unexpected node in resolve_type_constraint")
        }
    }

    fn resolve_prototype_genargs(&self, node: Option<&Node>)
        -> (AnnotationStatus, Rc<TypeScope<'object>>, Vec<Id>, Vec<TypeConstraint<'object>>)
    {
        if let Some(&Node::TyGenargs(_, ref vars, ref constraints)) = node {
            let mut type_vars = vec![];
            let mut type_constraints = vec![];

            let scope = vars.iter().fold(self.scope.clone(), |scope, var| {
                if let &Node::TyGendeclarg(ref loc, ref id, ref supertype) = var.as_ref() {
                    let scope = TypeScope::extend(scope, id.1.clone());

                    type_vars.push(id.clone());

                    if let Some(supertype) = supertype.as_ref() {
                        type_constraints.push(TypeConstraint::Compatible {
                            loc: loc.clone(),
                            sub: Rc::new(TypeNode::TypeParameter {
                                loc: id.0.clone(),
                                name: id.1.clone(),
                            }),
                            super_: TypeNode::resolve(supertype, self.env, scope.clone()),
                        });
                    }

                    scope
                } else {
                    panic!("expected TyGendeclarg in TyGenargs::1")
                }
            });

            type_constraints.extend(constraints.iter().map(|con|
                self.resolve_type_constraint(con, scope.clone())));

            (AnnotationStatus::Typed, scope, type_vars, type_constraints)
        } else {
            (AnnotationStatus::Empty, self.scope.clone(), vec![], vec![])
        }
    }

    pub fn resolve_prototype(&self, node: &Node)
        -> (AnnotationStatus, Prototype<'object>)
    {
        fn option_rc_ref<T>(rc: &Option<Rc<T>>) -> Option<&T> {
            rc.as_ref().map(Rc::as_ref)
        }

        let (genargs, args, retn) = match *node {
            Node::TyPrototype(_, ref genargs, ref args, ref retn) => {
                // Peel the ReturnSig to get the actual value
                // TODO: handle other (future) return structures here
                let retn = match option_rc_ref(retn) {
                    Some(&Node::TyReturnSig(_, ref cpath)) => Some(cpath.as_ref()),
                    _ => None,
                };
                (option_rc_ref(genargs), option_rc_ref(args), retn)
            }
            Node::Args(..) => (None, Some(node), None),
            _ => panic!("unexpected node type in resolve_prototype"),
        };

        let (anno, scope, type_vars, type_constraints)
            = self.resolve_prototype_genargs(genargs);

        let resolve = ResolveType { env: self.env, scope: scope };

        let (anno, args) = args.map(|args| {
            if let Node::Args(_, ref args) = *args {
                args.as_slice()
            } else {
                panic!("expected Args in args position of Prototype or definition node");
            }
        }).unwrap_or(&[]).iter().map(|arg| {
            resolve.resolve_arg(arg)
        }).fold((anno, Vec::new()), |(anno1, mut args), (anno2, arg)| {
            args.push(arg);
            (anno1.append(anno2), args)
        });

        let (anno, retn) = match retn {
            Some(retn) => (anno.append(AnnotationStatus::Typed), Some(resolve.resolve_type(retn))),
            None => (anno.append(AnnotationStatus::Untyped), None),
        };

        (anno, Prototype {
            loc: node.loc().clone(),
            type_vars: type_vars,
            type_constraints: type_constraints,
            args: args,
            retn: retn,
        })
    }
}
