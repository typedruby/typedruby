use std::rc::Rc;

use ast::{Id, Loc, Node};
use environment::Environment;
use errors::Detail;
use object::{Scope, RubyObject, ConstantEntry};

pub type TypeNodeRef<'object> = Rc<TypeNode<'object>>;

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
    pub fn new(scope: Rc<Scope<'object>>) -> Rc<TypeScope<'object>> {
        Rc::new(TypeScope::Constant { scope })
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

#[derive(Debug)]
pub struct Prototype<'object> {
    pub loc: Loc,
    pub type_parameters: Vec<TypeParameter<'object>>,
    pub args: Vec<ArgNode<'object>>,
    pub retn: Option<TypeNodeRef<'object>>,
}

#[derive(Debug)]
pub struct TypeParameter<'object> {
    pub name: Id,
    pub constraint: Option<TypeConstraint<'object>>,
}

#[derive(Debug)]
pub enum TypeConstraint<'object> {
    Compatible { loc: Loc, sub: TypeNodeRef<'object>, super_: TypeNodeRef<'object> },
    Unify { loc: Loc, a: TypeNodeRef<'object>, b: TypeNodeRef<'object> },
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
                    proto: self.resolve_prototype(prototype),
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
                panic!("unknown node type: {:?}"),
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

    fn resolve_arg(&self, node: &Node) -> ArgNode<'object> {
        let arg_loc = node.loc().clone();

        let (ty, arg_node) = match *node {
            Node::TypedArg(_, ref type_node, ref arg_node) =>
                (Some(self.resolve_type(type_node)), arg_node.as_ref()),
            _ => (None, node),
        };

        match *arg_node {
            Node::Arg(ref loc, ref name) =>
                ArgNode::Required { loc: arg_loc, ty: ty, lhs: ArgLhs::Lvar { name: Id(loc.clone(), name.clone()) } },
            Node::Procarg0(_, ref arg) =>
                ArgNode::Procarg0 { loc: arg_loc, arg: Box::new(self.resolve_arg(arg)) },
            Node::Blockarg(_, ref name) =>
                ArgNode::Block { loc: arg_loc, ty: ty, name: name.clone() },
            Node::Kwarg(ref loc, ref name) =>
                ArgNode::Kwarg { loc: arg_loc, ty: ty, name: Id(loc.clone(), name.clone()) },
            Node::Kwoptarg(_, ref name, ref expr) =>
                ArgNode::Kwoptarg { loc: arg_loc, ty: ty, name: name.clone(),
                    default: ArgExpr { expr: expr.clone(), scope: self.scope.constant_scope() } },
            Node::Mlhs(..) =>
                ArgNode::Required { loc: arg_loc, ty: ty, lhs: self.resolve_arg_mlhs(arg_node).unwrap() },
            Node::Optarg(_, ref name, ref expr) =>
                ArgNode::Optional { loc: arg_loc, ty: ty, name: name.clone(),
                    default: ArgExpr { expr: expr.clone(), scope: self.scope.constant_scope() } },
            Node::Restarg(_, ref name) =>
                ArgNode::Rest { loc: arg_loc, ty: ty, name: name.clone() },
            Node::Kwrestarg(_, ref name) =>
                ArgNode::Kwrest { loc: arg_loc, ty: ty, name: name.clone() },
            _ => panic!("unexpected node type in resolve_arg"),
        }
    }

    pub fn resolve_prototype(&self, node: &Node) -> Prototype<'object> {
        fn option_rc_ref<T>(rc: &Option<Rc<T>>) -> Option<&T> {
            rc.as_ref().map(Rc::as_ref)
        }

        let mut type_parameters = Vec::new();

        let (genargs, args, retn) = match *node {
            Node::Prototype(_, ref genargs, ref args, ref retn) =>
                (option_rc_ref(genargs), option_rc_ref(args), option_rc_ref(retn)),
            Node::Args(..) => (None, Some(node), None),
            _ => panic!("unexpected node type in resolve_prototype"),
        };

        // first, collect all type parameters into a Vec<TypeParameter> while
        // also binding their names in type scope:
        let type_scope = genargs.map(|genargs| {
            if let Node::TyGenargs(_, ref gendeclargs) = *genargs {
                gendeclargs.as_slice()
            } else {
                panic!("Expected TyGenargs in genargs position of Prototype")
            }
        }).unwrap_or(&[]).iter().map(|gendeclarg| {
            if let Node::TyGendeclarg(ref loc, ref name, ref constraint) = **gendeclarg {
                (loc, name, option_rc_ref(constraint))
            } else {
                panic!("Expected TyGendeclarg in TyGenargs")
            }
        }).fold(self.scope.clone(), |scope, (loc, name, constraint)| {
            let scope = TypeScope::extend(scope, name.clone());

            // TODO - name should be an Id, not a String
            let id = Id(loc.with_end(loc.begin_pos + name.len()), name.clone());

            type_parameters.push(TypeParameter {
                name: id,
                constraint: constraint.map(|con| match *con {
                    Node::TyConUnify(ref loc, ref a, ref b) =>
                        TypeConstraint::Unify {
                            loc: loc.clone(),
                            a: TypeNode::resolve(a, self.env, scope.clone()),
                            b: TypeNode::resolve(b, self.env, scope.clone()),
                        },
                    Node::TyConSubtype(ref loc, ref sub, ref super_) =>
                        TypeConstraint::Compatible {
                            loc: loc.clone(),
                            sub: TypeNode::resolve(sub, self.env, scope.clone()),
                            super_: TypeNode::resolve(super_, self.env, scope.clone()),
                        },
                    _ => panic!("unexpected node type in constraint position"),
                }),
            });

            scope
        });

        let resolve = ResolveType { env: self.env, scope: type_scope };

        let args = args.map(|args| {
            if let Node::Args(_, ref args) = *args {
                args.as_slice()
            } else {
                panic!("expected Args in args position of Prototype or definition node");
            }
        }).unwrap_or(&[]).iter().map(|arg| {
            resolve.resolve_arg(arg)
        }).collect();

        let retn = retn.map(|retn| resolve.resolve_type(retn));

        Prototype {
            loc: node.loc().clone(),
            type_parameters: type_parameters,
            args: args,
            retn: retn,
        }
    }
}
