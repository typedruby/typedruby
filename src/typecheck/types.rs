use std::cell::{Cell, RefCell};
use std::rc::Rc;
use ast::{Loc, Node};
use object::{ObjectGraph, RubyObject};
use typed_arena::Arena;
use immutable_map::TreeMap;
use util::Or;
use itertools::Itertools;

pub type TypeVarId = usize;

pub type UnificationError<'ty, 'object> = (&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>);
pub type UnificationResult<'ty, 'object> = Result<(), UnificationError<'ty, 'object>>;

#[derive(Clone)]
pub struct TypeEnv<'ty, 'env, 'object: 'ty + 'env> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    next_id: Rc<Cell<TypeVarId>>,
    instance_map: RefCell<TreeMap<TypeVarId, &'ty Type<'ty, 'object>>>,
    pub object: &'env ObjectGraph<'object>,
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

    pub fn any_prototype(&self, loc: Loc) -> Rc<Prototype<'ty, 'object>> {
        Rc::new(Prototype::Untyped { loc: loc })
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

    pub fn nillable(&self, loc: &Loc, ty: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        self.union(loc, self.nil(loc.clone()), ty)
    }

    pub fn union(&self, loc: &Loc, a: &'ty Type<'ty, 'object>, b: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        let mut reduced_types: Vec<&_> = Vec::new();

        let mut types = self.possible_types(a);
        types.extend(self.possible_types(b));

        for ty in types.into_iter() {
            if reduced_types.iter().any(|rty| self.same_type(rty, ty)) {
                continue;
            }

            reduced_types.push(ty);
        }

        assert!(!reduced_types.is_empty());

        if reduced_types.len() == 1 {
            reduced_types[0]
        } else {
            self.alloc(Type::Union { loc: loc.clone(), types: reduced_types })
        }
    }

    pub fn tuple(&self, loc: Loc, types: Vec<&'ty Type<'ty, 'object>>) -> &'ty Type<'ty, 'object> {
        self.alloc(Type::Tuple {
            loc: loc,
            lead: types,
            splat: None,
            post: Vec::new(),
        })
    }

    pub fn keyword_hash(&self, loc: Loc, keywords: Vec<(String, &'ty Type<'ty, 'object>)>) -> &'ty Type<'ty, 'object> {
        self.alloc(Type::KeywordHash {
            loc: loc,
            keywords: keywords,
            id: self.new_id(),
        })
    }

    pub fn local_variable(&self, loc: Loc, name: String, ty: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        let id = self.new_id();

        self.set_var(id, ty);

        self.alloc(Type::LocalVariable {
            loc: loc,
            name: name,
            id: id,
        })
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
            Type::LocalVariable { ref id, .. } |
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
            (&Type::Instance { .. }, &Type::KeywordHash { .. }) => {
                self.compatible(to, self.degrade_to_instance(from))
            },
            (&Type::Proc { proto: ref to_proto, .. }, &Type::Proc { proto: ref from_proto, .. }) => {
                self.compatible_prototype(to_proto, from_proto).unwrap_or(Err((to, from)))
            }
            (_, _) =>
                self.unify(to, from),
        }
    }

    pub fn compatible_prototype(&self, to: &Prototype<'ty, 'object>, from: &Prototype<'ty, 'object>) -> Option<UnificationResult<'ty, 'object>> {
        match (to, from) {
            (&Prototype::Untyped { .. }, _) => Some(Ok(())),
            (_, &Prototype::Untyped { .. }) => Some(Ok(())),
            (&Prototype::Typed { args: ref args1, retn: ref retn1, .. }, &Prototype::Typed { args: ref args2, retn: ref retn2, .. }) =>
                self.compatible_args(args1, args2).map(|_|
                    self.compatible(retn1, retn2)),
        }
    }

    fn args_to_tuple_arg(&self, args: &[Arg<'ty, 'object>]) -> Option<Arg<'ty, 'object>> {
        if args.len() == 0 {
            return None;
        }

        if args.len() == 1 {
            return Some(args[0].clone());
        }

        let mut arg_types = Vec::new();

        for arg in args {
            match *arg {
                Arg::Required { ty, .. } => arg_types.push(ty),
                _ => return None,
            }
        }

        let args_loc = args[0].loc().join(args[args.len() - 1].loc());

        Some(Arg::Required { loc: args_loc.clone(), ty: self.tuple(args_loc, arg_types) })
    }

    pub fn compatible_args(&self, to: &[Arg<'ty, 'object>], from: &[Arg<'ty, 'object>]) -> Option<UnificationResult<'ty, 'object>> {
        if to.len() == 1 {
            if let Arg::Procarg0 { arg: ref arg1, .. } = to[0] {
                if from.len() == 1 {
                    if let Arg::Procarg0 { arg: ref arg2, .. } = from[0] {
                        return self.compatible_arg(arg1, arg2);
                    }
                }

                return self.args_to_tuple_arg(from).and_then(|from_arg|
                    self.compatible_arg(arg1, &from_arg)
                );
            }
        }

        if from.len() == 1 {
            if let Arg::Procarg0 { arg: ref arg2, .. } = from[0] {
                return self.args_to_tuple_arg(to).and_then(|to_arg|
                    self.compatible_arg(&to_arg, arg2)
                );
            }
        }

        if to.len() != from.len() {
            return None;
        }

        for (to_arg, from_arg) in to.iter().zip(from.iter()) {
            match self.compatible_arg(to_arg, from_arg) {
                None => return None,
                e@Some(Err(..)) => return e,
                Some(Ok(())) => continue,
            }
        }

        Some(Ok(()))
    }

    pub fn compatible_arg(&self, to: &Arg<'ty, 'object>, from: &Arg<'ty, 'object>) -> Option<UnificationResult<'ty, 'object>> {
        match (to, from) {
            (&Arg::Procarg0 { arg: ref arg1, .. }, &Arg::Procarg0 { arg: ref arg2, .. }) =>
                self.compatible_arg(arg2, arg1),
            (&Arg::Procarg0 { arg: ref arg1, .. }, _) =>
                self.compatible_arg(arg1, from),
            (_, &Arg::Procarg0 { arg: ref arg2, .. }) =>
                self.compatible_arg(to, arg2),
            (&Arg::Required { ty: ref ty1, .. }, &Arg::Required { ty: ref ty2, .. }) |
            (&Arg::Optional { ty: ref ty1, .. }, &Arg::Required { ty: ref ty2, .. }) |
            (&Arg::Rest { ty: ref ty1, .. }, &Arg::Rest { ty: ref ty2, .. }) |
            (&Arg::Block { ty: ref ty1, .. }, &Arg::Block { ty: ref ty2, .. }) =>
                Some(self.compatible(ty2, ty1)),
            (&Arg::Kwarg { .. }, _) |
            (&Arg::Kwoptarg { .. }, _) =>
                panic!("TODO"),
            _ => None,
        }
    }

    fn is_unresolved_var(&self, ty: &'ty Type<'ty, 'object>) -> bool {
        if let Type::Var { .. } = *self.prune(ty) {
            true
        } else {
            false
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

            (&Type::Union { types: ref types1, .. }, &Type::Union { types: ref types2, .. }) => {
                if types1.len() != types2.len() {
                    return Err((t1, t2));
                }

                let mut marked = Vec::new();
                marked.resize(types1.len(), false);

                // attempt to unify all concrete types first:
                for ty2 in types2 {
                    if self.is_unresolved_var(ty2) { continue }

                    for (index, ty1) in types1.iter().enumerate() {
                        if marked[index] { continue }
                        if self.is_unresolved_var(ty1) { continue }

                        match self.unify(ty1, ty2) {
                            Ok(()) => {
                                marked[index] = true;
                                break
                            }
                            Err(..) => {
                                continue
                            }
                        }
                    }
                }

                // unify all unresolved type variables:
                for ty2 in types2 {
                    if !self.is_unresolved_var(ty2) { continue }

                    for (index, ty1) in types1.iter().enumerate() {
                        if marked[index] { continue }
                        if !self.is_unresolved_var(ty1) { continue }

                        self.unify(ty1, ty2).expect("unifying two unresolved type variables should never fail");
                        marked[index] = true;
                    }
                }

                // if by this point not all types are marked, there was a mismatch
                for m in marked {
                    if !m {
                        return Err((t1, t2));
                    }
                }

                Ok(())
            },

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

            (&Type::Proc { .. }, &Type::Proc { .. }) => {
                panic!("TODO unify proc");
            },

            (&Type::Proc { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::KeywordHash { .. }, &Type::KeywordHash { .. }) => {
                panic!("TODO unify keyword hash")
            }

            (&Type::KeywordHash { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::LocalVariable { .. }, _) =>
                panic!("LocalVariable should not be present after pruning!"),
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

    pub fn update_loc(&self, ty: &'ty Type<'ty, 'object>, loc: Loc) -> &'ty Type<'ty, 'object> {
        let tyvar = self.new_var(loc);

        self.unify(tyvar, ty).expect("unifying new tyvar");

        tyvar
    }

    fn describe_rec(&self, ty: &'ty Type<'ty, 'object>, buffer: &mut String) {
        use std::fmt::Write;

        match *self.prune(ty) {
            Type::Instance { ref class, ref type_parameters, .. } => {
                write!(buffer, "{}", class.name()).unwrap();

                if !type_parameters.is_empty() {
                    let mut print_comma = false;
                    write!(buffer, "::[").unwrap();

                    for param in type_parameters.iter() {
                        if print_comma { write!(buffer, ", ").unwrap(); }
                        self.describe_rec(param, buffer);
                        print_comma = true;
                    }

                    write!(buffer, "]").unwrap();
                }
            },
            Type::Tuple { ref lead, ref splat, ref post, .. } => {
                let mut print_comma = false;

                write!(buffer, "[").unwrap();

                for lead_ty in lead {
                    if print_comma { write!(buffer, ", ").unwrap(); }
                    self.describe_rec(lead_ty, buffer);
                    print_comma = true;
                }

                if let Some(splat_ty) = *splat {
                    if print_comma { write!(buffer, ", ").unwrap(); }
                    write!(buffer, "*").unwrap();
                    self.describe_rec(splat_ty, buffer);
                    print_comma = true;
                }

                for post_ty in post {
                    if print_comma { write!(buffer, ", ").unwrap(); }
                    self.describe_rec(post_ty, buffer);
                    print_comma = true;
                }

                write!(buffer, "]").unwrap();
            },
            Type::Union { ref types, .. } => {
                let mut print_pipe = false;

                for union_ty in types {
                    if print_pipe { write!(buffer, " | ").unwrap(); }
                    self.describe_rec(union_ty, buffer);
                    print_pipe = true;
                }
            },
            Type::Any { .. } => {
                write!(buffer, ":any").unwrap();
            },
            Type::TypeParameter { ref name, .. } => {
                write!(buffer, "type parameter {}", name).unwrap();
            },
            Type::KeywordHash { ref keywords, .. } => {
                let mut print_comma = false;

                write!(buffer, "{{").unwrap();

                for &(ref kw_name, ref kw_ty) in keywords {
                    if print_comma { write!(buffer, ", ").unwrap(); }
                    write!(buffer, "{}: ", kw_name).unwrap();
                    self.describe_rec(kw_ty, buffer);
                    print_comma = true;
                }

                write!(buffer, "}}").unwrap();
            },
            Type::Proc { .. } => {
                // TOOD
                write!(buffer, "Proc(todo)").unwrap();
            },
            Type::Var { ref id, .. } => {
                write!(buffer, "t{}", id).unwrap();
            },
            Type::LocalVariable { .. } => {
                panic!("should never remain after prune")
            },
        }
    }

    pub fn degrade_to_instance(&self, ty: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        match self.prune(ty) {
            &Type::KeywordHash { id, ref loc, ref keywords } => {
                let hash_class = self.object.hash_class();

                // degrade keyword hash to instance type:
                let key_ty = self.instance(loc.clone(), self.object.Symbol, vec![]);
                let value_ty = keywords.iter().map(|&(_, keyword_ty)|
                    keyword_ty
                ).fold1(|ty1, ty2|
                    self.union(loc, ty1, ty2)
                ).unwrap_or_else(||
                    self.new_var(loc.clone())
                );

                let instance_ty = self.instance(loc.clone(), hash_class, vec![key_ty, value_ty]);
                self.set_var(id, instance_ty);
                instance_ty
            },
            pruned => pruned,
        }
    }

    pub fn describe(&self, ty: &'ty Type<'ty, 'object>) -> String {
        let mut buffer = String::new();
        self.describe_rec(ty, &mut buffer);
        buffer
    }

    pub fn predicate(&self, ty: &'ty Type<'ty, 'object>) -> Or<&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>> {
        match *self.prune(ty) {
            Type::Instance { class, .. } => {
                if class.is_a(self.object.FalseClass) || class.is_a(self.object.NilClass) {
                    Or::Right(ty)
                } else if self.object.FalseClass.is_a(class) || self.object.NilClass.is_a(class) {
                    Or::Both(ty, ty)
                } else {
                    Or::Left(ty)
                }
            }
            Type::Union { ref types, ref loc } => {
                let mut preds = types.iter().map(|t| self.predicate(t));

                let first_pred = preds.next().expect("types is non-empty");

                preds.fold(first_pred, |a, b| {
                    a.append(b,
                        |a, b| self.union(loc, a, b),
                        |a, b| self.union(loc, a, b))
                })
            }
            Type::Tuple { .. } |
            Type::KeywordHash { .. } |
            Type::Proc { .. } => Or::Left(ty),
            Type::Any { .. } |
            Type::TypeParameter { .. } |
            Type::Var { .. } => Or::Both(ty, ty),
            Type::LocalVariable { .. } => panic!("should never remain after prune"),
        }
    }

    pub fn possible_types(&self, ty: &'ty Type<'ty, 'object>) -> Vec<&'ty Type<'ty, 'object>> {
        let mut tys = Vec::new();
        self.possible_types_rec(ty, &mut tys);
        tys
    }

    fn possible_types_rec(&self, ty: &'ty Type<'ty, 'object>, out_tys: &mut Vec<&'ty Type<'ty, 'object>>) {
        match *self.prune(ty) {
            Type::Union { types: ref union_types, .. } => {
                for ty in union_types {
                    self.possible_types_rec(ty, out_tys)
                }
            }
            _ => out_tys.push(ty),
        }
    }

    fn same_types(&self, tys1: &[&'ty Type<'ty, 'object>], tys2: &[&'ty Type<'ty, 'object>]) -> bool {
        tys1.iter().all(|ty1| tys2.iter().any(|ty2| self.same_type(ty1, ty2)))
    }

    pub fn same_type(&self, a: &'ty Type<'ty, 'object>, b: &'ty Type<'ty, 'object>) -> bool {
        match (self.prune(a), self.prune(b)) {
            (&Type::Instance { class: c1, type_parameters: ref tp1, .. },
                    &Type::Instance { class: c2, type_parameters: ref tp2, .. }) =>
                c1 == c2 && tp1.iter().zip(tp2).all(|(t1, t2)| self.same_type(t1, t2)),

            (&Type::Union { types: ref tys1, .. }, &Type::Union { types: ref tys2, .. }) =>
                self.same_types(tys1, tys2) && self.same_types(tys2, tys1),

            (&Type::Tuple { .. }, &Type::Tuple { .. }) =>
                panic!("TODO"),

            (&Type::KeywordHash { .. }, &Type::KeywordHash { .. }) =>
                panic!("TODO"),

            (&Type::Proc { .. }, &Type::Proc { .. }) =>
                false, // TODO

            (&Type::Any { .. }, &Type::Any { .. }) =>
                true,

            (&Type::TypeParameter { name: ref name1, .. }, &Type::TypeParameter { name: ref name2, .. }) =>
                name1 == name2,

            (&Type::Var { id: ref id1, .. }, &Type::Var { id: ref id2, .. }) =>
                id1 == id2,

            (&Type::LocalVariable { .. }, &Type::LocalVariable { .. }) =>
                panic!("should never happen"),

            _ => false,
        }
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
        proto: Rc<Prototype<'ty, 'object>>,
    },
    Var {
        loc: Loc,
        id: TypeVarId,
    },
    LocalVariable {
        loc: Loc,
        name: String,
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
            Type::LocalVariable { ref loc, .. } => loc,
        }
    }

    pub fn ref_eq(&self, other: &'ty Type<'ty, 'object>) -> bool {
        (self as *const _) == (other as *const _)
    }
}

#[derive(Debug)]
pub enum Prototype<'ty, 'object: 'ty> {
    Untyped {
        loc: Loc,
    },
    Typed {
        loc: Loc,
        args: Vec<Arg<'ty, 'object>>,
        retn: &'ty Type<'ty, 'object>,
    },
}

impl<'ty, 'object> Prototype<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            Prototype::Untyped { ref loc } => loc,
            Prototype::Typed { ref loc, .. } => loc,
        }
    }
}

#[derive(Debug,Clone)]
pub enum Arg<'ty, 'object: 'ty> {
    Required {
        loc: Loc,
        ty: &'ty Type<'ty, 'object>,
    },
    Procarg0 {
        loc: Loc,
        arg: Box<Arg<'ty, 'object>>,
    },
    Optional {
        loc: Loc,
        ty: &'ty Type<'ty, 'object>,
        expr: Rc<Node>,
    },
    Rest {
        loc: Loc,
        ty: &'ty Type<'ty, 'object>,
    },
    Kwarg {
        loc: Loc,
        name: String,
        ty: &'ty Type<'ty, 'object>,
    },
    Kwoptarg {
        loc: Loc,
        name: String,
        ty: &'ty Type<'ty, 'object>,
        expr: Rc<Node>,
    },
    Kwrest {
        loc: Loc,
        ty: &'ty Type<'ty, 'object>,
    },
    Block {
        loc: Loc,
        ty: &'ty Type<'ty, 'object>,
    },
}

impl<'ty, 'object> Arg<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            Arg::Required { ref loc, .. } => loc,
            Arg::Procarg0 { ref loc, .. } => loc,
            Arg::Optional { ref loc, .. } => loc,
            Arg::Rest { ref loc, .. } => loc,
            Arg::Kwarg { ref loc, .. } => loc,
            Arg::Kwoptarg { ref loc, .. } => loc,
            Arg::Block { ref loc, .. } => loc,
            Arg::Kwrest { ref loc, .. } => loc,
        }
    }
}
