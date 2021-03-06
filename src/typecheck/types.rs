use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use std::cmp::Ordering;
use ast::{Loc, Node, Id};
use environment::Environment;
use object::RubyObject;
use typed_arena::Arena;
use util::Or;
use itertools::Itertools;
use std::ops::Deref;
use std::iter::{self, FromIterator};
use std::collections::HashMap;
use typecheck::materialize::Materialize;
use vec_map::VecMap;

pub type TypeVarId = usize;

#[derive(Debug)]
pub enum TypeError<'ty, 'object: 'ty> {
    Incompatible(TypeRef<'ty, 'object>, TypeRef<'ty, 'object>),
    Recursive(TypeRef<'ty, 'object>, TypeVarId, Loc),
}

pub type TypeResult<'ty, 'object, T> = Result<T, TypeError<'ty, 'object>>;

pub type MatchResult<'ty, 'object> = TypeResult<'ty, 'object, ()>;

enum UnionCompatibilityError {
    NoMatch,
}

#[derive(Debug,Clone)]
enum SelfInfo<'ty, 'object: 'ty> {
     Type(TypeRef<'ty, 'object>),
     // this case is used where we know the class and type parameters of self,
     // but we have no location information to create a real type with:
     Instance(&'object RubyObject<'object>, Vec<TypeRef<'ty, 'object>>),
}

impl<'ty, 'object> SelfInfo<'ty, 'object> {
    pub fn ty(&self, tyenv: &TypeEnv<'ty, 'object>, loc: Loc) -> TypeRef<'ty, 'object> {
        match *self {
            SelfInfo::Type(ty) =>
                tyenv.update_loc(ty, loc),
            SelfInfo::Instance(class, ref type_parameters) =>
                tyenv.instance(loc, class, type_parameters.clone())
        }
    }

    pub fn class(&self, tyenv: &TypeEnv<'ty, 'object>) -> &'object RubyObject<'object> {
        match *self {
            SelfInfo::Type(ty) => {
                match *tyenv.prune(ty) {
                    Type::Instance { class, .. } => class,
                    Type::Proc { .. } => tyenv.env.object.Proc,
                    Type::Tuple { .. } => tyenv.env.object.array_class(),
                    Type::KeywordHash { .. } => tyenv.env.object.hash_class(),
                    ref ty => panic!("illegal self_type in TypeContext: {:?}", ty),
                }
            }
            SelfInfo::Instance(class, _) => class,
        }
    }
}

pub struct TypeMapData<T> {
    map: VecMap<T>,
    log: Vec<(TypeVarId, T)>,
}

impl<T: Clone> TypeMapData<T> {
    pub fn new() -> Self {
        TypeMapData { map: VecMap::new(), log: Vec::new() }
    }

    pub fn insert(&mut self, id: TypeVarId, val: T) {
        match self.map.insert(id, val.clone()) {
            None => {
                // insert into log:
                self.log.push((id, val));
            }
            Some(_) => {
                panic!("tried to set already-set TypeVarId in TypeMap - this is a critical logic error");
            }
        }
    }

    pub fn get(&self, id: TypeVarId) -> Option<T> {
        self.map.get(id).cloned()
    }
}

impl<T: Clone> FromIterator<(TypeVarId, T)> for TypeMapData<T> {
    fn from_iter<It: IntoIterator<Item = (TypeVarId, T)>>(iter: It) -> Self {
        let mut map = Self::new();

        for (id, val) in iter {
            map.insert(id, val)
        }

        map
    }
}

pub enum TransactionFlag<U> {
    Commit(U),
    Rollback(U),
}

pub enum TransactionResult<T> {
    Committed,
    RolledBack(TypeMapData<T>)
}

struct TypeMap<T> {
    inner: RefCell<TypeMapData<T>>,
}

impl<T: Clone> TypeMap<T> {
    pub fn new() -> Self {
        TypeMap { inner: RefCell::new(TypeMapData::new()) }
    }

    pub fn insert(&self, id: TypeVarId, val: T) {
        self.inner.borrow_mut().insert(id, val)
    }

    pub fn get(&self, id: TypeVarId) -> Option<T> {
        self.inner.borrow().get(id)
    }

    pub fn transaction<F, U>(&self, f: F) -> (U, TransactionResult<T>)
        where F: FnOnce() -> TransactionFlag<U>
    {
        let log_len = {
            self.inner.borrow().log.len()
        };

        match f() {
            TransactionFlag::Commit(val) => (val, TransactionResult::Committed),
            TransactionFlag::Rollback(val) => {
                let mut inner = self.inner.borrow_mut();

                let changes = inner.log.split_off(log_len);

                for &(id, _) in &changes {
                    match inner.map.remove(id) {
                        None => { panic!("expected TypeVarId to be mapped to a type"); }
                        Some(_) => { /* ok */ }
                    }
                }

                (val, TransactionResult::RolledBack(changes.into_iter().collect()))
            }
        }
    }
}

#[derive(Debug,Clone)]
pub struct TypeContext<'ty, 'object: 'ty> {
    self_: SelfInfo<'ty, 'object>,
    pub class: &'object RubyObject<'object>,
    type_parameters: Vec<TypeRef<'ty, 'object>>,
    pub type_names: HashMap<String, TypeRef<'ty, 'object>>,
}

impl<'ty, 'object> TypeContext<'ty, 'object> {
    fn new(self_: SelfInfo<'ty, 'object>, class: &'object RubyObject<'object>, type_parameters: Vec<TypeRef<'ty, 'object>>) -> Self {
        let type_names =
            class.type_parameters().iter()
                .map(|&Id(_, ref name)| name.clone())
                .zip(type_parameters.iter().cloned())
                .collect();

        TypeContext {
            self_,
            class,
            type_parameters,
            type_names,
        }
    }

    pub fn instance(class: &'object RubyObject<'object>, type_parameters: Vec<TypeRef<'ty, 'object>>) -> Self {
        let self_ = SelfInfo::Instance(class, type_parameters.clone());
        Self::new(self_, class, type_parameters)
    }

    pub fn with_type(self_type: TypeRef<'ty, 'object>, class: &'object RubyObject<'object>, type_parameters: Vec<TypeRef<'ty, 'object>>) -> TypeContext<'ty, 'object> {
        Self::new(SelfInfo::Type(self_type), class, type_parameters)
    }

    pub fn self_class(&self, tyenv: &TypeEnv<'ty, 'object>) -> &'object RubyObject<'object> {
        self.self_.class(tyenv)
    }

    pub fn self_type(&self, tyenv: &TypeEnv<'ty, 'object>, loc: Loc) -> TypeRef<'ty, 'object> {
        self.self_.ty(tyenv, loc)
    }
}

pub struct TypeEnv<'ty, 'object: 'ty> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    next_id: Rc<Cell<TypeVarId>>,
    type_map: TypeMap<TypeRef<'ty, 'object>>,
    env: &'ty Environment<'object>,
}

impl<'ty, 'object: 'ty> TypeEnv<'ty, 'object> {
    pub fn new(arena: &'ty Arena<Type<'ty, 'object>>, env: &'ty Environment<'object>) -> TypeEnv<'ty, 'object> {
        TypeEnv {
            arena: arena,
            env: env,
            type_map: TypeMap::new(),
            next_id: Rc::new(Cell::new(1)),
        }
    }

    fn new_id(&self) -> TypeVarId {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        id
    }

    pub fn alloc(&self, ty: Type<'ty, 'object>) -> TypeRef<'ty, 'object> {
        TypeRef(self.arena.alloc(ty))
    }

    pub fn new_var(&self, loc: Loc) -> TypeRef<'ty, 'object> {
        self.alloc(Type::Var { loc: loc, id: self.new_id() })
    }

    pub fn any(&self, loc: Loc) -> TypeRef<'ty, 'object> {
        self.alloc(Type::Any { loc: loc })
    }

    pub fn any_prototype(&self, loc: Loc) -> Rc<Prototype<'ty, 'object>> {
        let any_ty = self.any(loc.clone());

        Rc::new(Prototype {
            loc: loc.clone(),
            constraints: vec![],
            args: vec![
                Arg::Rest { loc: loc.clone(), ty: any_ty },
                Arg::Block { loc: loc.clone(), ty: any_ty },
            ],
            retn: any_ty,
        })
    }

    pub fn instance(&self, loc: Loc, class: &'object RubyObject<'object>, type_parameters: Vec<TypeRef<'ty, 'object>>)
        -> TypeRef<'ty, 'object>
    {
        assert!(class.type_parameters().len() == type_parameters.len());

        self.alloc(Type::Instance {
            loc: loc,
            class: class,
            type_parameters: type_parameters,
        })
    }

    pub fn instance0(&self, loc: Loc, class: &'object RubyObject<'object>) -> TypeRef<'ty, 'object> {
        self.instance(loc, class, Vec::new())
    }

    pub fn nil(&self, loc: Loc) -> TypeRef<'ty, 'object> {
        self.instance(loc, self.env.object.NilClass, Vec::new())
    }

    pub fn nillable(&self, loc: &Loc, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.union(loc, self.nil(loc.clone()), ty)
    }

    pub fn union(&self, loc: &Loc, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        let mut reduced_types: Vec<TypeRef> = Vec::new();

        let mut types = self.possible_types(a);
        types.extend(self.possible_types(b));

        for ty in types {
            if reduced_types.iter().any(|rty| self.same_type(*rty, ty)) {
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

    pub fn tuple(&self, loc: Loc, lead: Vec<TypeRef<'ty, 'object>>, splat: Option<TypeRef<'ty, 'object>>, post: Vec<TypeRef<'ty, 'object>>)
        -> TypeRef<'ty, 'object>
    {
        self.alloc(Type::Tuple {
            loc: loc,
            lead: lead,
            splat: splat,
            post: post,
            id: self.new_id(),
        })
    }

    pub fn keyword_hash(&self, loc: Loc, keywords: Vec<(String, TypeRef<'ty, 'object>)>, splat: Option<TypeRef<'ty, 'object>>)
        -> TypeRef<'ty, 'object>
    {
        self.alloc(Type::KeywordHash {
            loc: loc,
            keywords: keywords,
            splat: splat,
            id: self.new_id(),
        })
    }

    pub fn local_variable(&self, loc: Loc, name: String, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        let id = self.new_id();

        self.set_var(id, ty);

        self.alloc(Type::LocalVariable {
            loc: loc,
            name: name,
            id: id,
        })
    }

    pub fn array(&self, loc: Loc, element_type: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.instance(loc, self.env.object.array_class(), vec![element_type])
    }

    pub fn hash(&self, loc: Loc, key_type: TypeRef<'ty, 'object>, value_type: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.instance(loc, self.env.object.hash_class(), vec![key_type, value_type])
    }

    fn set_var(&self, id: TypeVarId, ty: TypeRef<'ty, 'object>) {
        self.type_map.insert(id, ty);
    }

    pub fn prune(&self, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        match *ty {
            Type::Var { id, .. } |
            Type::LocalVariable { id, .. } |
            Type::KeywordHash { id, .. } |
            Type::Tuple { id, .. } => {
                if let Some(instance) = { self.type_map.get(id) } {
                    return self.prune(instance)
                }
            },
            _ => {},
        }

        ty.clone()
    }

    pub fn map_type_context(&self, type_context: TypeContext<'ty, 'object>, module: &'object RubyObject<'object>)
        -> TypeContext<'ty, 'object>
    {
        let mat = Materialize::new(self.env, self);

        let include_chain = type_context.class.include_chain(module);

        let self_ = type_context.self_.clone();

        include_chain.iter().rev()
            .fold(type_context, |context, site| {
                let type_params = site.type_parameters.iter()
                    .map(|param| mat.materialize_type(param, &context))
                    .collect::<Vec<_>>();

                let type_names = site.module.type_parameters().iter()
                    .map(|&Id(_, ref name)| name.to_owned())
                    .zip(type_params.iter().cloned())
                    .collect();

                TypeContext {
                    self_: self_.clone(),
                    class: site.module,
                    type_parameters: type_params,
                    type_names: type_names,
                }
            })
    }

    fn ordered_union_types(&self, union_tys: &[TypeRef<'ty, 'object>])
        -> Vec<TypeRef<'ty, 'object>>
    {
        let mut types = union_tys.iter()
            .map(|ty| {
                let depth = self.type_var_depth(*ty);
                (depth, *ty)
            })
            .collect::<Vec<_>>();

        types.sort_by(|&(a_depth, _), &(b_depth, _)|
            match (a_depth, b_depth) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(a), Some(b)) => {
                    // This cmp is intentionally flipped - we want to sort
                    // types so that those with depth None are first, followed
                    // by types with Some depth in descending order:
                    b.cmp(&a)
                }
            });

        types.into_iter().map(|(_, ty)| ty).collect()
    }

    fn compatible_from_union(&self, to_ty: TypeRef<'ty, 'object>, union_tys: &[TypeRef<'ty, 'object>])
        -> Result<(), TypeError<'ty, 'object>>
    {
        let (result, _) = self.type_map.transaction(|| {
            for ty in self.ordered_union_types(union_tys) {
                match self.compatible(to_ty, ty) {
                    Ok(()) => {}
                    Err(e) => return TransactionFlag::Rollback(Err(e))
                }
            }

            TransactionFlag::Commit(Ok(()))
        });

        result
    }

    fn compatible_to_union(&self, union_tys: &[TypeRef<'ty, 'object>], from_ty: TypeRef<'ty, 'object>)
        -> Result<(), UnionCompatibilityError>
    {
        let candidates = self.ordered_union_types(union_tys)
            .into_iter()
            .filter_map(|ty| {
                let (result, _) = self.type_map.transaction(|| {
                    TransactionFlag::Rollback(self.compatible(ty, from_ty))
                });

                result.map(|()| ty).ok()
            })
            .collect::<Vec<_>>();

        let (result, _) = self.type_map.transaction(|| {
            let is_match = candidates.into_iter().any(|ty| {
                self.compatible(ty, from_ty).is_ok()
            });

            if is_match {
                TransactionFlag::Commit(Ok(()))
            } else {
                TransactionFlag::Rollback(Err(UnionCompatibilityError::NoMatch))
            }
        });

        result
    }

    fn unify_var(&self, ty: TypeRef<'ty, 'object>, var_id: TypeVarId, loc: &Loc) -> MatchResult<'ty, 'object> {
        if let Type::Var { id: ty_id, .. } = *ty {
            if var_id == ty_id {
                return Ok(())
            }
        }

        if self.occurs(ty, var_id) {
            Err(TypeError::Recursive(ty, var_id, loc.clone()))
        } else {
            self.set_var(var_id, ty);
            Ok(())
        }
    }

    pub fn compatible(&self, to_: TypeRef<'ty, 'object>, from_: TypeRef<'ty, 'object>) -> MatchResult<'ty, 'object> {
        let to = self.prune(to_);
        let from = self.prune(from_);

        match (to.deref(), from.deref()) {
            (&Type::Var { id: to_id, .. }, _) => {
                self.unify_var(from, to_id, from_.loc())
            }
            (_, &Type::Var { id: from_id, .. }) => {
                self.unify_var(to, from_id, to_.loc())
            }
            (&Type::Instance { class: to_class, type_parameters: ref to_tp, .. }, &Type::Instance { class: from_class, type_parameters: ref from_tp, .. }) => {
                if from_class.ancestors().find(|c| c.delegate() == to_class).is_none() {
                    return Err(TypeError::Incompatible(to, from));
                }

                let from_tyctx = TypeContext::with_type(from, from_class, from_tp.clone());

                let to_tyctx = self.map_type_context(from_tyctx, to_class);

                to_tp.iter().zip(to_tyctx.type_parameters).fold(Ok(()), |res, (&to_ty, from_ty)|
                    // because an object could be mutated after coercion, we
                    // require invariance in type parameters:
                    res.and_then(|()| self.compatible(to_ty, from_ty))
                       .and_then(|()| self.compatible(from_ty, to_ty)))
            },
            (_, &Type::Union { types: ref from_types, .. }) => {
                self.compatible_from_union(to, from_types)
            },
            (&Type::Union { types: ref to_types, .. }, _) => {
                self.compatible_to_union(to_types, from)
                    .map_err(|e| match e {
                        UnionCompatibilityError::NoMatch => TypeError::Incompatible(to, from)
                    })
            },
            (&Type::Any { .. }, _) => Ok(()),
            (_, &Type::Any { .. }) => Ok(()),
            (&Type::Instance { class, .. }, &Type::KeywordHash { .. })
                if self.is_hash(class) =>
            {
                self.compatible(to, self.degrade_to_instance(from))
            }
            (&Type::Tuple { ref lead, ref splat, ref post, .. }, &Type::Instance { class, ref type_parameters, .. })
                if self.env.object.is_array(class) =>
            {
                // While very convenient, this compatibility rule is slightly
                // unsound as it assumes that the array instance has enough
                // elements to satisfy the tuple. In this case I think the
                // trade off makes sense so let's allow this.

                let array_element_ty = type_parameters[0];

                lead.iter().chain(splat).chain(post).fold(Ok(()), |result, &ty| {
                    result.and_then(|()| self.compatible(ty, array_element_ty))
                })
            }
            (&Type::Instance { .. }, &Type::Tuple { .. }) => {
                self.compatible(to, self.degrade_to_instance(from))
            }
            (&Type::Tuple { .. }, &Type::Tuple { .. }) => {
                use slice_util::View;

                let to_elems = self.elements_from_tuple(to);
                let mut to_elems = View(&to_elems);

                let from_elems = self.elements_from_tuple(from);
                let mut from_elems = View(&from_elems);

                while let Some(&SplatArg::Value(to_ty)) = to_elems.first() {
                    match from_elems.first() {
                        Some(&SplatArg::Value(from_ty)) => {
                            to_elems.consume_front();
                            from_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        Some(&SplatArg::Splat(from_ty)) => {
                            to_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        None => {
                            break;
                        }
                    }
                }

                while let Some(&SplatArg::Value(to_ty)) = to_elems.last() {
                    match from_elems.last() {
                        Some(&SplatArg::Value(from_ty)) => {
                            to_elems.consume_back();
                            from_elems.consume_back();
                            self.compatible(to_ty, from_ty)?;
                        }
                        Some(&SplatArg::Splat(from_ty)) => {
                            to_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        None => {
                            break;
                        }
                    }
                }

                if let Some(&SplatArg::Splat(from_ty)) = from_elems.first() {
                    if let Some(&SplatArg::Splat(to_ty)) = to_elems.first() {
                        to_elems.consume_front();
                        from_elems.consume_front();
                        self.compatible(to_ty, from_ty)?;
                    } else {
                        return Err(TypeError::Incompatible(to, from));
                    }
                } else if let Some(&SplatArg::Splat(_)) = to_elems.first() {
                    to_elems.consume_front();
                }

                if to_elems.is_empty() && from_elems.is_empty() {
                    Ok(())
                } else {
                    Err(TypeError::Incompatible(to, from))
                }
            }
            (&Type::KeywordHash { .. }, &Type::KeywordHash { .. }) => {
                panic!("TODO implement compatible for keyword hash to keyword hash");
            }
            (&Type::KeywordHash { ref keywords, splat, .. }, &Type::Instance { class, ref type_parameters, .. }) => {
                if !self.is_hash(class) {
                    return Err(TypeError::Incompatible(to, from));
                }

                let key_ty = type_parameters[0];
                let value_ty = type_parameters[1];

                match *self.prune(key_ty) {
                    Type::Instance { class, .. } if class.is_a(self.env.object.Symbol) => {
                        // ok!
                    },
                    _ => return Err(TypeError::Incompatible(to, from)),
                }

                keywords.iter()
                    .map(|&(_, kw_ty)| kw_ty)
                    .chain(splat)
                    .fold(Ok(()), |res, ty| {
                        res.and_then(|()| self.compatible(ty, value_ty))
                    })
            }
            (&Type::Proc { proto: ref to_proto, .. }, &Type::Proc { proto: ref from_proto, .. }) => {
                self.compatible_prototype(to_proto, from_proto).unwrap_or(Err(TypeError::Incompatible(to, from)))
            }
            (&Type::TypeParameter { name: ref name1, .. }, &Type::TypeParameter { name: ref name2, .. }) => {
                if name1 == name2 {
                    Ok(())
                } else {
                    Err(TypeError::Incompatible(to, from))
                }
            }
            (_, _) => Err(TypeError::Incompatible(to, from))
        }
    }

    pub fn compatible_prototype(&self, to: &Prototype<'ty, 'object>, from: &Prototype<'ty, 'object>) -> Option<MatchResult<'ty, 'object>> {
        self.compatible_args(&to.args, &from.args).map(|result|
            result.and_then(|()|
                self.compatible(to.retn, from.retn)))
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

        Some(Arg::Required { loc: args_loc.clone(), ty: self.tuple(args_loc, arg_types, None, vec![]) })
    }

    pub fn compatible_args(&self, to: &[Arg<'ty, 'object>], from: &[Arg<'ty, 'object>]) -> Option<MatchResult<'ty, 'object>> {
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

    pub fn compatible_arg(&self, to: &Arg<'ty, 'object>, from: &Arg<'ty, 'object>) -> Option<MatchResult<'ty, 'object>> {
        match (to, from) {
            (&Arg::Procarg0 { arg: ref arg1, .. }, &Arg::Procarg0 { arg: ref arg2, .. }) =>
                self.compatible_arg(arg2, arg1),
            (&Arg::Procarg0 { arg: ref arg1, .. }, _) =>
                self.compatible_arg(arg1, from),
            (_, &Arg::Procarg0 { arg: ref arg2, .. }) =>
                self.compatible_arg(to, arg2),
            (&Arg::Required { ty: ty1, .. }, &Arg::Required { ty: ty2, .. }) |
            (&Arg::Optional { ty: ty1, .. }, &Arg::Required { ty: ty2, .. }) |
            (&Arg::Rest { ty: ty1, .. }, &Arg::Rest { ty: ty2, .. }) |
            (&Arg::Block { ty: ty1, .. }, &Arg::Block { ty: ty2, .. }) =>
                Some(self.compatible(ty2, ty1)),
            (&Arg::Kwarg { .. }, _) |
            (&Arg::Kwoptarg { .. }, _) =>
                panic!("TODO"),
            _ => None,
        }
    }

    pub fn instantiate_var(&self, tyvar: TypeRef<'ty, 'object>, ty: TypeRef<'ty, 'object>) {
        if let Type::Var { id, .. } = *self.prune(tyvar) {
            self.set_var(id, ty);
        } else {
            panic!("attempting to instantiate already instantiated type: {:?}", tyvar);
        }
    }

    pub fn is_uninstantiated_var(&self, ty: TypeRef<'ty, 'object>) -> bool {
        if let Type::Var { .. } = *self.prune(ty) {
            true
        } else {
            false
        }
    }

    pub fn is_instance(&self, ty: TypeRef<'ty, 'object>, class: &'object RubyObject<'object>) -> bool {
        if let Type::Instance { class: ty_class, .. } = *self.prune(ty) {
            ty_class.is_a(class)
        } else {
            false
        }
    }

    fn occurs(&self, ty: TypeRef<'ty, 'object>, id: TypeVarId) -> bool {
        match *self.prune(ty).deref() {
            Type::Var { id: var_id, .. } if id == var_id => true,
            Type::Var { .. } => false,
            Type::Instance { ref type_parameters, .. } =>
                type_parameters.iter().any(|t| self.occurs(*t, id)),
            Type::Tuple { ref lead, ref splat, ref post, .. } =>
                lead.iter()
                    .chain(splat)
                    .chain(post)
                    .any(|t| self.occurs(*t, id)),
            Type::Union { ref types, .. } =>
                types.iter().any(|t| self.occurs(*t, id)),
            Type::Any { .. } |
            Type::TypeParameter { .. } => false,
            Type::KeywordHash { ref keywords, .. } =>
                keywords.iter()
                    .map(|&(_, t)| t)
                    .any(|t| self.occurs(t, id)),
            Type::Proc { ref proto, .. } =>
                proto.args.iter()
                    .map(|arg| match *arg.unwrap_procarg0() {
                        Arg::Procarg0 { .. } => unreachable!(),
                        Arg::Required { ty, .. } |
                        Arg::Optional { ty, .. } |
                        Arg::Rest { ty, .. } |
                        Arg::Kwarg { ty, .. } |
                        Arg::Kwoptarg { ty, .. } |
                        Arg::Kwrest { ty, .. } |
                        Arg::Block { ty, .. } => ty
                    })
                    .chain(iter::once(proto.retn))
                    .any(|t| self.occurs(t, id)),
            Type::LocalVariable { .. } => unreachable!(),
        }
    }

    fn type_var_depth(&self, ty: TypeRef<'ty, 'object>) -> Option<usize> {
        match *self.prune(ty).deref() {
            Type::Var { .. } => Some(0),
            Type::Instance { ref type_parameters, .. } =>
                type_parameters.iter()
                    .filter_map(|ty| self.type_var_depth(*ty))
                    .min()
                    .map(|depth| depth + 1),
            Type::Tuple { ref lead, ref splat, ref post, .. } =>
                lead.iter().chain(splat).chain(post)
                    .filter_map(|ty| self.type_var_depth(*ty))
                    .min()
                    .map(|depth| depth + 1),
            Type::Union { ref types, .. } =>
                types.iter()
                    .filter_map(|ty| self.type_var_depth(*ty))
                    .min()
                    .map(|depth| depth + 1),
            Type::KeywordHash { ref keywords, .. } =>
                keywords.iter()
                    .map(|&(_, ty)| ty)
                    .filter_map(|ty| self.type_var_depth(ty))
                    .min()
                    .map(|depth| depth + 1),
            Type::Proc { ref proto, .. } =>
                proto.args.iter()
                    .map(|arg| match *arg.unwrap_procarg0() {
                        Arg::Procarg0 { .. } => unreachable!(),
                        Arg::Required { ty, .. } |
                        Arg::Optional { ty, .. } |
                        Arg::Rest { ty, .. } |
                        Arg::Kwarg { ty, .. } |
                        Arg::Kwoptarg { ty, .. } |
                        Arg::Kwrest { ty, .. } |
                        Arg::Block { ty, .. } => ty
                    })
                    .chain(iter::once(proto.retn))
                    .filter_map(|ty| self.type_var_depth(ty))
                    .min()
                    .map(|depth| depth + 1),
            Type::Any { .. } |
            Type::TypeParameter { .. } => None,
            Type::LocalVariable { .. } => unreachable!(),
        }
    }

    pub fn update_loc(&self, ty: TypeRef<'ty, 'object>, loc: Loc) -> TypeRef<'ty, 'object> {
        let tyvar = self.new_var(loc);
        self.instantiate_var(tyvar, ty);
        tyvar
    }

    pub fn describe(&self, ty: TypeRef<'ty, 'object>) -> String {
        let mut buffer = String::new();
        ty.describe(self, &mut buffer).unwrap();
        buffer
    }

    pub fn degrade_to_instance(&self, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        let pruned = self.prune(ty);

        match pruned.deref() {
            &Type::KeywordHash { id, ref loc, ref keywords, splat } => {
                let hash_class = self.env.object.hash_class();

                // degrade keyword hash to instance type:
                let key_ty = self.instance(loc.clone(), self.env.object.Symbol, vec![]);
                let value_ty = keywords.iter().map(|&(_, keyword_ty)|
                    keyword_ty
                ).chain(splat).fold1(|ty1, ty2|
                    self.union(loc, ty1, ty2)
                ).unwrap_or_else(||
                    self.new_var(loc.clone())
                );

                let instance_ty = self.instance(loc.clone(), hash_class, vec![key_ty, value_ty]);
                self.set_var(id, instance_ty);
                instance_ty
            },
            &Type::Tuple { id, ref lead, ref splat, ref post, ref loc } => {
                let array_class = self.env.object.array_class();

                let element_ty = lead.iter()
                    .chain(splat)
                    .chain(post)
                    .map(|&t| t)
                    .fold1(|ty1, ty2| self.union(loc, ty1, ty2))
                    .unwrap_or_else(|| self.new_var(loc.clone()));

                let instance_ty = self.instance(loc.clone(), array_class, vec![element_ty]);
                self.set_var(id, instance_ty);
                instance_ty
            }
            _ => pruned,
        }
    }

    pub fn predicate(&self, ty: TypeRef<'ty, 'object>) -> Or<TypeRef<'ty, 'object>, TypeRef<'ty, 'object>> {
        match *self.prune(ty) {
            Type::Instance { class, .. } => {
                if class.is_a(self.env.object.FalseClass) || class.is_a(self.env.object.NilClass) {
                    Or::Right(ty)
                } else if self.env.object.FalseClass.is_a(class) || self.env.object.NilClass.is_a(class) {
                    Or::Both(ty, ty)
                } else {
                    Or::Left(ty)
                }
            }
            Type::Union { ref types, ref loc } => {
                types.iter()
                    .map(|t| self.predicate(*t))
                    .fold1(|a, b|
                        a.append(b,
                            |a, b| self.union(loc, a, b),
                            |a, b| self.union(loc, a, b)))
                    .unwrap()
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

    pub fn partition_by_class(&self, ty: TypeRef<'ty, 'object>, class: &'object RubyObject<'object>, class_loc: &Loc)
        -> Or<TypeRef<'ty, 'object>, TypeRef<'ty, 'object>>
    {
        let partition_inner = |ty_class: &'object RubyObject<'object>, ty_params: Option<&[TypeRef<'ty, 'object>]>| {
            if ty_class.is_a(class) {
                Or::Left(ty)
            } else if class.is_a(ty_class) {
                let narrowed_ty = if let Some(ty_params) = ty_params {
                    let sub_class = class;
                    let sub_params = sub_class.type_parameters().iter()
                        .map(|_| self.new_var(class_loc.clone()))
                        .collect::<Vec<_>>();
                    let sub_ty = self.instance(class_loc.clone(), sub_class, sub_params.clone());
                    let sub_tyctx = TypeContext::with_type(sub_ty, sub_class, sub_params);

                    let super_tyctx = self.map_type_context(sub_tyctx, ty_class);

                    ty_params.iter()
                        .zip(super_tyctx.type_parameters)
                        .map(|(sub_tp, super_tp)| self.compatible(super_tp, *sub_tp))
                        .fold_results((), |(), ()| ())
                        .map(|()| sub_ty)
                        .unwrap_or(ty)
                } else {
                    ty
                };

                Or::Both(narrowed_ty, ty)
            } else {
                Or::Right(ty)
            }
        };

        match *self.prune(ty) {
            Type::Instance { class: ty_class, type_parameters: ref ty_params, .. } =>
                partition_inner(ty_class, Some(ty_params)),
            Type::Proc { .. } =>
                partition_inner(self.env.object.Proc, Some(&[])),
            Type::Tuple { .. } =>
                partition_inner(self.env.object.array_class(), None),
            Type::KeywordHash { .. } =>
                partition_inner(self.env.object.hash_class(), None),
            Type::Union { ref types, ref loc, .. } => {
                types.iter()
                    .map(|t| self.partition_by_class(*t, class, class_loc))
                    .fold1(|a, b|
                        a.append(b,
                            |a, b| self.union(class_loc, a, b),
                            |a, b| self.union(loc, a, b)))
                    .unwrap()
            }
            Type::Any { .. } |
            Type::TypeParameter { .. } |
            Type::Var { .. } =>
                Or::Both(ty, ty),
            Type::LocalVariable { .. } =>
                panic!("should never remain after prune"),
        }
    }

    pub fn possible_types(&self, ty: TypeRef<'ty, 'object>) -> Vec<TypeRef<'ty, 'object>> {
        let mut tys = Vec::new();
        self.possible_types_rec(ty, &mut tys);
        tys
    }

    fn possible_types_rec(&self, ty: TypeRef<'ty, 'object>, out_tys: &mut Vec<TypeRef<'ty, 'object>>) {
        match *self.prune(ty) {
            Type::Union { types: ref union_types, .. } => {
                for ty in union_types {
                    self.possible_types_rec(*ty, out_tys)
                }
            }
            _ => out_tys.push(ty),
        }
    }

    fn same_unordered_types(&self, tys1: &[TypeRef<'ty, 'object>], tys2: &[TypeRef<'ty, 'object>]) -> bool {
        tys1.iter().all(|ty1| tys2.iter().any(|ty2| self.same_type(*ty1, *ty2)))
    }

    fn same_types(&self, tys1: &[TypeRef<'ty, 'object>], tys2: &[TypeRef<'ty, 'object>]) -> bool {
        tys1.len() == tys2.len() && tys1.iter().zip(tys2).all(|(t1, t2)| self.same_type(*t1, *t2))
    }

    pub fn same_type(&self, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>) -> bool {
        match (&*self.prune(a), &*self.prune(b)) {
            (&Type::Instance { class: c1, type_parameters: ref tp1, .. },
                    &Type::Instance { class: c2, type_parameters: ref tp2, .. }) =>
                c1 == c2 && tp1.iter().zip(tp2).all(|(t1, t2)| self.same_type(*t1, *t2)),

            (&Type::Union { types: ref tys1, .. }, &Type::Union { types: ref tys2, .. }) =>
                self.same_unordered_types(tys1, tys2) && self.same_unordered_types(tys2, tys1),

            (&Type::Tuple { lead: ref lead1, splat: ref splat1, post: ref post1, .. },
                &Type::Tuple { lead: ref lead2, splat: ref splat2, post: ref post2, .. }) =>
            {
                self.same_types(lead1, lead2) &&
                    match (splat1, splat2) {
                        (&Some(t1), &Some(t2)) => self.same_type(t1, t2),
                        (&None, &None) => true,
                        _ => false
                    } &&
                    self.same_types(post1, post2)
            }

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

    pub fn to_keyword_hash(&self, ty: TypeRef<'ty, 'object>) -> Option<TypeRef<'ty, 'object>> {
        let pruned = self.prune(ty);

        match *pruned {
            Type::KeywordHash { .. } => Some(pruned),
            Type::Instance { class, ref type_parameters, .. }
                if self.is_hash(class) =>
                    if self.is_instance(type_parameters[0], self.env.object.Symbol) {
                        Some(self.keyword_hash(ty.loc().clone(), vec![], Some(type_parameters[1])))
                    } else {
                        None
                    },
            _ => None,
        }
    }

    pub fn is_hash(&self, class: &'object RubyObject<'object>) -> bool {
        self.env.object.is_hash(class)
    }

    pub fn is_keyword_hash(&self, ty: TypeRef<'ty, 'object>) -> bool {
        match *self.prune(ty) {
            Type::KeywordHash { .. } => true,
            Type::Instance { class, ref type_parameters, .. }
                if self.is_hash(class) =>
                    self.is_instance(type_parameters[0], self.env.object.Symbol),
            _ => false,
        }
    }

    pub fn kwsplat_to_hash(&self, ty: TypeRef<'ty, 'object>)
        -> KwsplatResult<'ty, 'object>
    {
        match *self.prune(ty) {
            Type::KeywordHash { ref keywords, splat, .. } =>
                keywords.iter()
                    .map(|&(_,v)| v)
                    .chain(splat)
                    .fold(KwsplatResult::None, |res, ty| res.append_ty(self, ty.loc(), ty)),
            Type::Instance { class, ref type_parameters, .. }
                if self.is_hash(class)
                && self.is_instance(type_parameters[0], self.env.object.Symbol)
                =>
                    KwsplatResult::Ok(type_parameters[1]),
            Type::Union { ref types, .. } =>
                types.iter()
                    .map(|union_ty| self.kwsplat_to_hash(*union_ty))
                    .fold(KwsplatResult::None, |a, b| a.append(self, ty.loc(), b)),
            _ if self.is_instance(ty, self.env.object.NilClass) =>
                KwsplatResult::None,
            _ =>
                KwsplatResult::Err(ty),
        }
    }

    fn elements_from_tuple(&self, tuple: TypeRef<'ty, 'object>) -> Vec<SplatArg<'ty, 'object>> {
        if let Type::Tuple { ref lead, splat, ref post, .. } = *self.prune(tuple) {
            let mut elements = Vec::new();

            elements.extend(lead.iter().map(|&ty| SplatArg::Value(ty)));
            elements.extend(splat.iter().map(|&ty| SplatArg::Splat(ty)));
            elements.extend(post.iter().map(|&ty| SplatArg::Value(ty)));

            elements
        } else {
            panic!("type not a tuple")
        }
    }
}

#[derive(Debug,Clone)]
pub enum SplatArg<'ty, 'object: 'ty> {
    Value(TypeRef<'ty, 'object>),
    Splat(TypeRef<'ty, 'object>),
}

impl<'ty, 'object> SplatArg<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            SplatArg::Value(ref ty) |
            SplatArg::Splat(ref ty) => ty.loc(),
        }
    }
}

pub enum KwsplatResult<'ty, 'object: 'ty> {
    Err(TypeRef<'ty, 'object>),
    None,
    Ok(TypeRef<'ty, 'object>),
}

impl<'ty, 'object: 'ty> KwsplatResult<'ty, 'object> {
    fn append_ty(&self, tyenv: &TypeEnv<'ty, 'object>, loc: &Loc, ty: TypeRef<'ty, 'object>)
        -> KwsplatResult<'ty, 'object>
    {
        self.append(tyenv, loc, KwsplatResult::Ok(ty))
    }

    fn append(&self, tyenv: &TypeEnv<'ty, 'object>, loc: &Loc, other: KwsplatResult<'ty, 'object>)
        -> KwsplatResult<'ty, 'object>
    {
        match *self {
            KwsplatResult::Err(ty) => KwsplatResult::Err(ty),
            KwsplatResult::None => other,
            KwsplatResult::Ok(ty) => match other {
                KwsplatResult::Err(err_ty) => KwsplatResult::Err(err_ty),
                KwsplatResult::None => KwsplatResult::Ok(ty),
                KwsplatResult::Ok(other_ty) => KwsplatResult::Ok(tyenv.union(loc, ty, other_ty)),
            },
        }
    }
}

#[derive(Copy,Clone)]
pub struct TypeRef<'ty, 'object: 'ty>(&'ty Type<'ty, 'object>);

impl<'ty, 'object> TypeRef<'ty, 'object> {
    pub fn deref(&self) -> &'ty Type<'ty, 'object> {
        self.0
    }

    pub fn describe(&self, tyenv: &TypeEnv<'ty, 'object>, f: &mut fmt::Write) -> fmt::Result {
        match *tyenv.prune(*self) {
            Type::Instance { ref class, ref type_parameters, .. } => {
                write!(f, "{}", class.name())?;

                if !type_parameters.is_empty() {
                    let mut print_comma = false;
                    write!(f, "::[")?;

                    for param in type_parameters.iter() {
                        if print_comma { write!(f, ", ")?; }
                        param.describe(tyenv, f)?;
                        print_comma = true;
                    }

                    write!(f, "]")?;
                }
            },
            Type::Tuple { ref lead, ref splat, ref post, .. } => {
                let mut print_comma = false;

                write!(f, "[")?;

                for lead_ty in lead {
                    if print_comma { write!(f, ", ")?; }
                    lead_ty.describe(tyenv, f)?;
                    print_comma = true;
                }

                if let Some(splat_ty) = *splat {
                    if print_comma { write!(f, ", ")?; }
                    write!(f, "*")?;
                    splat_ty.describe(tyenv, f)?;
                    print_comma = true;
                }

                for post_ty in post {
                    if print_comma { write!(f, ", ")?; }
                    post_ty.describe(tyenv, f)?;
                    print_comma = true;
                }

                write!(f, "]")?;
            },
            Type::Union { ref types, .. } => {
                let mut print_pipe = false;

                for union_ty in types {
                    if print_pipe { write!(f, " | ")?; }
                    union_ty.describe(tyenv, f)?;
                    print_pipe = true;
                }
            },
            Type::Any { .. } => {
                write!(f, ":any")?;
            },
            Type::TypeParameter { ref name, .. } => {
                write!(f, "type parameter {}", name)?;
            },
            Type::KeywordHash { ref keywords, splat, .. } => {
                let mut print_comma = false;

                write!(f, "{{")?;

                for &(ref kw_name, ref kw_ty) in keywords {
                    if print_comma { write!(f, ", ")?; }
                    write!(f, "{}: ", kw_name)?;
                    kw_ty.describe(tyenv, f)?;
                    print_comma = true;
                }

                if let Some(splat) = splat {
                    if print_comma { write!(f, ", ")?; }
                    write!(f, "**")?;
                    splat.describe(tyenv, f)?;
                }

                write!(f, "}}")?;
            },
            Type::Proc { ref proto, .. } => {
                write!(f, "{{ ")?;
                proto.describe(tyenv, f)?;
                write!(f, " }}")?;
            },
            Type::Var { ref id, .. } => {
                write!(f, "t{}", id)?;
            },
            Type::LocalVariable { .. } => {
                panic!("should never remain after prune")
            },
        }

        Ok(())
    }
}

impl<'ty, 'object> Deref for TypeRef<'ty, 'object> {
    type Target = Type<'ty, 'object>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'ty, 'object> fmt::Debug for TypeRef<'ty, 'object> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
pub enum Type<'ty, 'object: 'ty> {
    Instance {
        loc: Loc,
        class: &'object RubyObject<'object>,
        type_parameters: Vec<TypeRef<'ty, 'object>>,
    },
    Tuple {
        loc: Loc,
        lead: Vec<TypeRef<'ty, 'object>>,
        splat: Option<TypeRef<'ty, 'object>>,
        post: Vec<TypeRef<'ty, 'object>>,
        // tuples can degrade to normal array instances:
        id: TypeVarId,
    },
    Union {
        loc: Loc,
        types: Vec<TypeRef<'ty, 'object>>,
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
        keywords: Vec<(String, TypeRef<'ty, 'object>)>,
        splat: Option<TypeRef<'ty, 'object>>,
        // keyword hash types can degrade to normal hash instances
        // when they do, there will be an entry in the type_map for this
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

#[derive(Debug)]
pub enum TypeConstraint<'ty, 'object: 'ty> {
    Compatible {
        loc: Loc,
        sub: TypeRef<'ty, 'object>,
        super_: TypeRef<'ty, 'object>,
    },
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

    pub fn ref_eq(&self, other: &Type<'ty, 'object>) -> bool {
        (self as *const _) == (other as *const _)
    }
}

#[derive(Debug)]
pub struct Prototype<'ty, 'object: 'ty> {
    pub loc: Loc,
    pub constraints: Vec<TypeConstraint<'ty, 'object>>,
    pub args: Vec<Arg<'ty, 'object>>,
    pub retn: TypeRef<'ty, 'object>,
}

impl<'ty, 'object> Prototype<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    pub fn describe(&self, tyenv: &TypeEnv<'ty, 'object>, f: &mut fmt::Write) -> fmt::Result {
        let mut print_comma = false;

        write!(f, "|")?;

        for arg in &self.args {
            if print_comma { write!(f, ", ")?; }
            arg.describe(tyenv, f)?;
            print_comma = true;
        }

        write!(f, "| => ")?;

        self.retn.describe(tyenv, f)
    }
}

#[derive(Debug,Clone)]
pub enum Arg<'ty, 'object: 'ty> {
    Required {
        loc: Loc,
        ty: TypeRef<'ty, 'object>,
    },
    Procarg0 {
        loc: Loc,
        arg: Box<Arg<'ty, 'object>>,
    },
    Optional {
        loc: Loc,
        ty: TypeRef<'ty, 'object>,
        expr: Rc<Node>,
    },
    Rest {
        loc: Loc,
        ty: TypeRef<'ty, 'object>,
    },
    Kwarg {
        loc: Loc,
        name: String,
        ty: TypeRef<'ty, 'object>,
    },
    Kwoptarg {
        loc: Loc,
        name: String,
        ty: TypeRef<'ty, 'object>,
        expr: Rc<Node>,
    },
    Kwrest {
        loc: Loc,
        ty: TypeRef<'ty, 'object>,
    },
    Block {
        loc: Loc,
        ty: TypeRef<'ty, 'object>,
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

    pub fn unwrap_procarg0(&self) -> &Self {
        if let Arg::Procarg0 { ref arg, .. } = *self {
            &**arg
        } else {
            self
        }
    }

    pub fn describe(&self, tyenv: &TypeEnv<'ty, 'object>, f: &mut fmt::Write) -> fmt::Result {
        match *self {
            Arg::Required { ty, .. } => ty.describe(tyenv, f),
            Arg::Procarg0 { ref arg, .. } => arg.describe(tyenv, f),
            Arg::Optional { ty, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " = ?")
            }
            Arg::Rest { ty, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " *")
            }
            Arg::Kwarg { ty, ref name, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " {}:", name)
            }
            Arg::Kwoptarg { ty, ref name, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " {}: ?", name)
            }
            Arg::Kwrest { ty, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " **")
            }
            Arg::Block { ty, .. } => {
                ty.describe(tyenv, f)?;
                write!(f, " &")
            }
        }
    }
}
