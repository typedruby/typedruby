use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use ast::{Loc, Node};
use object::{ObjectGraph, RubyObject};
use typed_arena::Arena;
use immutable_map::TreeMap;

pub type TypeVarId = usize;

pub type UnificationResult<'ty, 'object> = Result<(), (&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>)>;

#[derive(Clone)]
pub struct TypeEnv<'ty, 'env, 'object: 'ty + 'env> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    next_id: Rc<Cell<TypeVarId>>,
    instance_map: RefCell<TreeMap<TypeVarId, &'ty Type<'ty, 'object>>>,
    object: &'env ObjectGraph<'object>,
}

impl<'ty, 'env, 'object: 'env> TypeEnv<'ty, 'env, 'object> {
    pub fn new(arena: &'ty Arena<Type<'ty, 'object>>, object: &'env ObjectGraph<'object>) -> TypeEnv<'ty, 'env, 'object> {
        TypeEnv {
            arena: arena,
            object: object,
            instance_map: RefCell::new(TreeMap::new()),
            next_id: Rc::new(Cell::new(1)),
        }
    }

    fn new_id(&self) -> TypeVarId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    pub fn alloc(&self, ty: Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        self.arena.alloc(ty)
    }

    pub fn new_var(&self, loc: Loc) -> &'ty Type<'ty, 'object> {
        self.alloc(Type::Var { loc: loc, id: self.new_id() })
    }

    pub fn any(&self, loc: Loc) -> &'ty Type<'ty, 'object> {
        self.alloc(Type::Any { loc: loc })
    }

    pub fn instance(&self, loc: Loc, class: &'object RubyObject<'object>, type_parameters: Vec<&'ty Type<'ty, 'object>>)
        -> &'ty Type<'ty, 'object>
    {
        assert!(class.type_parameters().len() == type_parameters.len());

        self.alloc(Type::Instance {
            loc: loc,
            class: class,
            type_parameters: type_parameters,
        })
    }

    pub fn instance0(&self, loc: Loc, class: &'object RubyObject<'object>) -> &'ty Type<'ty, 'object> {
        self.instance(loc, class, Vec::new())
    }

    pub fn nil(&self, loc: Loc) -> &'ty Type<'ty, 'object> {
        self.instance(loc, self.object.NilClass, Vec::new())
    }

    pub fn union(&self, a: &'ty Type<'ty, 'object>, b: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        panic!("not implemented!")
    }

    fn set_var(&self, id: TypeVarId, ty: &'ty Type<'ty, 'object>) {
        let mut instance_map_ref = self.instance_map.borrow_mut();

        *instance_map_ref = instance_map_ref.insert_or_update(id, ty.clone(), |v|
            panic!("attempted to set typevar {} to {:?}, but is already {:?}",
                id, ty, v)
        );
    }

    pub fn prune(&self, ty: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        match *ty {
            Type::Var { ref id, .. } |
            Type::KeywordHash { ref id, .. } => {
                if let Some(instance) = { self.instance_map.borrow().get(id) } {
                    return self.prune(instance)
                }
            },
            _ => {},
        }

        ty.clone()
    }

    pub fn compatible(&self, to: &'ty Type<'ty, 'object>, from: &'ty Type<'ty, 'object>) -> UnificationResult<'ty, 'object> {
        let to = self.prune(to);
        let from = self.prune(from);

        match (to, from) {
            (&Type::Var { .. }, _) =>
                self.unify(to, from),
            (_, &Type::Var { .. }) =>
                self.unify(to, from),
            (&Type::Instance { class: to_class, type_parameters: ref to_tp, .. }, &Type::Instance { class: from_class, type_parameters: ref from_tp, .. }) => {
                if from_class.ancestors().find(|c| c.delegate() == to_class).is_none() {
                    return Err((to, from));
                }

                if to_tp.len() > 0 {
                    // typedruby has no covariance, so we simply unify here
                    // rather than checking compatibility:
                    match self.unify_slice(to_tp, from_tp) {
                        None => Err((to, from)),
                        Some(e@Err(..)) => e,
                        Some(Ok(())) => Ok(())
                    }
                } else {
                    Ok(())
                }
            },
            (_, &Type::Union { types: ref from_types, .. }) => {
                for from_type in from_types {
                    if let e@Err(..) = self.compatible(to, from_type) {
                        return e;
                    }
                }

                Ok(())
            },
            (&Type::Union { types: ref to_types, .. }, _) => {
                for to_type in to_types {
                    if let Ok(()) = self.compatible(to_type, from) {
                        return Ok(());
                    }
                }

                Err((to, from))
            },
            (&Type::Any { .. }, _) => Ok(()),
            (_, &Type::Any { .. }) => Ok(()),
            (&Type::Instance { class: to_class, type_parameters: ref to_tp, .. }, &Type::KeywordHash { ref loc, ref keywords, id, .. }) => {
                let hash_class = self.object.get_const(self.object.Object, "Hash").expect("Hash to be defined");

                if to_class == hash_class {
                    // degrade keyword hash to instance type:
                    let key_ty = self.instance(loc.clone(), self.object.Symbol, vec![]);
                    let value_ty = self.new_var(loc.clone());

                    for &(_, kwty) in keywords {
                        try!(self.unify(value_ty, value_ty));
                    }

                    self.set_var(id, self.instance(loc.clone(), hash_class, vec![key_ty, value_ty]));

                    self.compatible(to, from)
                } else {
                    Err((to, from))
                }
            },
            (_, _) =>
                self.unify(to, from),
        }
    }

    pub fn unify(&self, t1: &'ty Type<'ty, 'object>, t2: &'ty Type<'ty, 'object>) -> UnificationResult<'ty, 'object> {
        let t1 = self.prune(t1);
        let t2 = self.prune(t2);

        match (t1, t2) {
            (&Type::Var { id: ref id1, .. }, _) => {
                if let Type::Var { id: ref id2, .. } = *t2 {
                    if id1 == id2 {
                        // already unified
                        return Ok(());
                    }
                }

                self.set_var(*id1, t2.clone());
                Ok(())
            },

            (_, &Type::Var { .. }) =>
                self.unify(&t2, &t1),

            (&Type::Instance { class: class1, type_parameters: ref tp1, .. }, &Type::Instance { class: class2, type_parameters: ref tp2, .. }) => {
                if class1 != class2 {
                    return Err((t1.clone(), t2.clone()));
                }

                self.unify_slice(tp1, tp2).expect("Instance types of same class to have same number of type parameters")
            },

            (&Type::Instance { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::Tuple { lead: ref lead1, splat: ref splat1, post: ref post1, .. }, &Type::Tuple { lead: ref lead2, splat: ref splat2, post: ref post2, .. }) => {
                self.unify_slice(lead1, lead2)
                    .and_then(|res|
                        match (*splat1, *splat2) {
                            (Some(ref a), Some(ref b)) => Some(res.and_then(|_|
                                self.unify(a, b)
                            )),
                            (None, None) => Some(res),
                            _ => None,
                        }
                    ).and_then(|res|
                        match res {
                            Ok(_) => self.unify_slice(post1, post2),
                            Err(e) => Some(Err(e)),
                        }
                    ).unwrap_or(
                        Err((t1.clone(), t2.clone()))
                    )
            }

            (&Type::Tuple { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::Union { .. }, &Type::Union { .. }) =>
                panic!("TODO unify union"),

            (&Type::Union { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::Any { .. }, &Type::Any { .. }) =>
                Ok(()),

            (&Type::Any { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::TypeParameter { name: ref name1, .. }, &Type::TypeParameter { name: ref name2, .. }) =>
                if name1 == name2 {
                    Ok(())
                } else {
                    Err((t1.clone(), t2.clone()))
                },

            (&Type::TypeParameter { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            _ => panic!("unify! t1={:?}, t2={:?}", t1, t2),
        }
    }

    fn unify_slice(&self, types1: &[&'ty Type<'ty, 'object>], types2: &[&'ty Type<'ty, 'object>]) -> Option<UnificationResult<'ty, 'object>> {
        if types1.len() != types2.len() {
            return None;
        }

        for (a, b) in types1.iter().zip(types2.iter()) {
            match self.unify(a, b) {
                Ok(_) => {},
                err@Err(..) => return Some(err),
            }
        }

        Some(Ok(()))
    }

    fn unify_option(&self, opt1: &Option<&'ty Type<'ty, 'object>>, opt2: &Option<&'ty Type<'ty, 'object>>) -> Option<UnificationResult<'ty, 'object>> {
        match (*opt1, *opt2) {
            (Some(ref t1), Some(ref t2)) => Some(self.unify(t1, t2)),
            (None, None) => Some(Ok(())),
            _ => None,
        }
    }

    pub fn update_loc(&self, ty: &'ty Type<'ty, 'object>, loc: Loc) -> &'ty Type<'ty, 'object> {
        let tyvar = self.new_var(loc);

        self.unify(tyvar, ty).expect("unifying new tyvar");

        tyvar
    }

    fn describe_rec(&self, ty: &'ty Type<'ty, 'object>, buffer: &mut String) {
        use std::fmt::Write;

        match *self.prune(ty) {
            Type::Instance { ref class, ref type_parameters, .. } => {
                write!(buffer, "{}", class.name());

                if !type_parameters.is_empty() {
                    write!(buffer, "::[");

                    self.describe_rec(type_parameters.first().unwrap(), buffer);

                    for param in type_parameters.iter().skip(1) {
                        write!(buffer, ", ");
                    }

                    write!(buffer, "]");
                }
            },
            Type::Tuple { ref lead, ref splat, ref post, .. } => {
                let mut print_comma = false;

                write!(buffer, "[");

                for lead_ty in lead {
                    if print_comma { write!(buffer, ", "); }
                    self.describe_rec(lead_ty, buffer);
                    print_comma = true;
                }

                if let Some(splat_ty) = *splat {
                    if print_comma { write!(buffer, ", "); }
                    self.describe_rec(splat_ty, buffer);
                    print_comma = true;
                }

                for post_ty in lead {
                    if print_comma { write!(buffer, ", "); }
                    self.describe_rec(post_ty, buffer);
                    print_comma = true;
                }

                write!(buffer, "]");
            },
            Type::Union { ref types, .. } => {
                let mut print_pipe = false;

                for union_ty in types {
                    if print_pipe { write!(buffer, " | "); }
                    self.describe_rec(union_ty, buffer);
                    print_pipe = true;
                }
            },
            Type::Any { .. } => {
                write!(buffer, ":any");
            },
            Type::TypeParameter { ref name, .. } => {
                write!(buffer, "type parameter {}", name);
            },
            Type::KeywordHash { ref keywords, .. } => {
                let mut print_comma = false;

                write!(buffer, "{{");

                for &(ref kw_name, ref kw_ty) in keywords {
                    if print_comma { write!(buffer, ", "); }
                    write!(buffer, "{}: ", kw_name);
                    self.describe_rec(kw_ty, buffer);
                    print_comma = true;
                }

                write!(buffer, "}}");
            },
            Type::Proc { ref args, ref retn, .. } => {
                // TOOD
                write!(buffer, "Proc(todo)");
            },
            Type::Var { ref id, .. } => {
                write!(buffer, "t{}", id);
            },
        }
    }

    pub fn describe(&self, ty: &'ty Type<'ty, 'object>) -> String {
        let mut buffer = String::new();
        self.describe_rec(ty, &mut buffer);
        buffer
    }
}

#[derive(Debug)]
pub enum Type<'ty, 'object: 'ty> {
    Instance {
        loc: Loc,
        class: &'object RubyObject<'object>,
        type_parameters: Vec<&'ty Type<'ty, 'object>>,
    },
    Tuple {
        loc: Loc,
        lead: Vec<&'ty Type<'ty, 'object>>,
        splat: Option<&'ty Type<'ty, 'object>>,
        post: Vec<&'ty Type<'ty, 'object>>,
    },
    Union {
        loc: Loc,
        types: Vec<&'ty Type<'ty, 'object>>,
    },
    Any {
        loc: Loc,
    },
    TypeParameter {
        loc: Loc,
        name: String,
    },
    KeywordHash {
        loc: Loc,
        keywords: Vec<(String, &'ty Type<'ty, 'object>)>,
        // keyword hash types can degrade to normal hash instances
        // when they do, there will be an entry in the instance_map for this
        // id:
        id: TypeVarId,
    },
    Proc {
        loc: Loc,
        args: Vec<Arg<'ty, 'object>>,
        retn: Option<&'ty Type<'ty, 'object>>,
    },
    Var {
        loc: Loc,
        id: TypeVarId,
    }
}

impl<'ty, 'object> Type<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            Type::Instance { ref loc, .. } => loc,
            Type::Tuple { ref loc, .. } => loc,
            Type::Union { ref loc, .. } => loc,
            Type::Any { ref loc, .. } => loc,
            Type::TypeParameter { ref loc, .. } => loc,
            Type::KeywordHash { ref loc, .. } => loc,
            Type::Proc { ref loc, .. } => loc,
            Type::Var { ref loc, .. } => loc,
        }
    }

    pub fn ref_eq(&self, other: &'ty Type<'ty, 'object>) -> bool {
        (self as *const _) == (other as *const _)
    }
}

#[derive(Debug)]
pub enum Arg<'ty, 'object: 'ty> {
    Required {
        loc: Loc,
        ty: Option<&'ty Type<'ty, 'object>>,
    },
    Procarg0 {
        loc: Loc,
        arg: Box<Arg<'ty, 'object>>,
    },
    Optional {
        loc: Loc,
        ty: Option<&'ty Type<'ty, 'object>>,
        expr: Rc<Node>,
    },
    Rest {
        loc: Loc,
        ty: Option<&'ty Type<'ty, 'object>>,
    },
    Block {
        loc: Loc,
        ty: Option<&'ty Type<'ty, 'object>>,
    },
}
