use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use ast::{Loc, Node};
use object::{ObjectGraph, RubyObject};
use typed_arena::Arena;
use immutable_map::TreeMap;
use util::Or;
use itertools::Itertools;

pub type TypeVarId = usize;

pub type TypeRef<'ty, 'object> = &'ty Type<'ty, 'object>;

pub type UnificationError<'ty, 'object> = (TypeRef<'ty, 'object>, TypeRef<'ty, 'object>);
pub type UnificationResult<'ty, 'object> = Result<(), UnificationError<'ty, 'object>>;

#[derive(Clone)]
pub struct TypeEnv<'ty, 'env, 'object: 'ty + 'env> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    next_id: Rc<Cell<TypeVarId>>,
    instance_map: RefCell<TreeMap<TypeVarId, TypeRef<'ty, 'object>>>,
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

    pub fn alloc(&self, ty: Type<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.arena.alloc(ty)
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
        self.instance(loc, self.object.NilClass, Vec::new())
    }

    pub fn nillable(&self, loc: &Loc, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        self.union(loc, self.nil(loc.clone()), ty)
    }

    pub fn union(&self, loc: &Loc, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
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

    fn set_var(&self, id: TypeVarId, ty: TypeRef<'ty, 'object>) {
        let mut instance_map_ref = self.instance_map.borrow_mut();

        *instance_map_ref = instance_map_ref.insert_or_update(id, ty.clone(), |v|
            panic!("attempted to set typevar {} to {:?}, but is already {:?}",
                id, ty, v)
        );
    }

    pub fn prune(&self, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        match *ty {
            Type::Var { ref id, .. } |
            Type::LocalVariable { ref id, .. } |
            Type::KeywordHash { ref id, .. } |
            Type::Tuple { ref id, .. } => {
                if let Some(instance) = { self.instance_map.borrow().get(id) } {
                    return self.prune(instance)
                }
            },
            _ => {},
        }

        ty.clone()
    }

    pub fn compatible(&self, to: TypeRef<'ty, 'object>, from: TypeRef<'ty, 'object>) -> UnificationResult<'ty, 'object> {
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
                    // because an object could be mutated after coercion, we
                    // require invariance in type parameters:
                    to_tp.iter().zip(from_tp).fold(Ok(()), |res, (to_ty, from_ty)|
                        res.and_then(|()| self.compatible(to_ty, from_ty))
                           .and_then(|()| self.compatible(from_ty, to_ty)))
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
            (&Type::Instance { class, .. }, &Type::KeywordHash { .. })
                if self.object.is_hash(class) =>
            {
                self.compatible(to, self.degrade_to_instance(from))
            }
            (&Type::Tuple { ref lead, ref splat, ref post, .. }, &Type::Instance { class, ref type_parameters, .. })
                if self.object.is_array(class) =>
            {
                // While very convenient, this compatibility rule is slightly
                // unsound as it assumes that the array instance has enough
                // elements to satisfy the tuple. In this case I think the
                // trade off makes sense so let's allow this.

                let array_element_ty = type_parameters[0];

                lead.iter().chain(splat).chain(post).fold(Ok(()), |result, ty| {
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

                while let Some(&TupleElement::Value(to_ty)) = to_elems.first() {
                    match from_elems.first() {
                        Some(&TupleElement::Value(from_ty)) => {
                            to_elems.consume_front();
                            from_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        Some(&TupleElement::Splat(from_ty)) => {
                            to_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        None => {
                            break;
                        }
                    }
                }

                while let Some(&TupleElement::Value(to_ty)) = to_elems.last() {
                    match from_elems.last() {
                        Some(&TupleElement::Value(from_ty)) => {
                            to_elems.consume_back();
                            from_elems.consume_back();
                            self.compatible(to_ty, from_ty)?;
                        }
                        Some(&TupleElement::Splat(from_ty)) => {
                            to_elems.consume_front();
                            self.compatible(to_ty, from_ty)?;
                        }
                        None => {
                            break;
                        }
                    }
                }

                if let Some(&TupleElement::Splat(from_ty)) = from_elems.first() {
                    if let Some(&TupleElement::Splat(to_ty)) = to_elems.first() {
                        to_elems.consume_front();
                        from_elems.consume_front();
                        self.compatible(to_ty, from_ty)?;
                    } else {
                        return Err((to, from));
                    }
                } else if let Some(&TupleElement::Splat(_)) = to_elems.first() {
                    to_elems.consume_front();
                }

                assert!(to_elems.is_empty());
                assert!(from_elems.is_empty());

                Ok(())
            }
            (&Type::KeywordHash { ref keywords, splat, .. }, &Type::Instance { class, ref type_parameters, .. }) => {
                if !self.object.is_hash(class) {
                    return Err((to, from));
                }

                let key_ty = type_parameters[0];
                let value_ty = type_parameters[1];

                match *self.prune(key_ty) {
                    Type::Instance { class, .. } if class.is_a(self.object.Symbol) => {
                        // ok!
                    },
                    _ => return Err((to, from)),
                }

                keywords.iter()
                    .map(|&(_, kw_ty)| kw_ty)
                    .chain(splat)
                    .fold(Ok(()), |res, ty| {
                        res.and_then(|()| self.compatible(ty, value_ty))
                    })
            }
            (&Type::Proc { proto: ref to_proto, .. }, &Type::Proc { proto: ref from_proto, .. }) => {
                self.compatible_prototype(to_proto, from_proto).unwrap_or(Err((to, from)))
            }
            (_, _) =>
                self.unify(to, from),
        }
    }

    pub fn compatible_prototype(&self, to: &Prototype<'ty, 'object>, from: &Prototype<'ty, 'object>) -> Option<UnificationResult<'ty, 'object>> {
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

    pub fn is_unresolved_var(&self, ty: TypeRef<'ty, 'object>) -> bool {
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

    pub fn unify(&self, t1: TypeRef<'ty, 'object>, t2: TypeRef<'ty, 'object>) -> UnificationResult<'ty, 'object> {
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

                let mut marked1 = Vec::new();
                marked1.resize(types1.len(), false);
                let mut marked2 = marked1.clone();

                // attempt to unify all concrete types first:
                for (idx2, ty2) in types2.iter().enumerate() {
                    if self.is_unresolved_var(ty2) { continue }

                    for (idx1, ty1) in types1.iter().enumerate() {
                        if marked1[idx1] { continue }
                        if self.is_unresolved_var(ty1) { continue }

                        match self.unify(ty1, ty2) {
                            Ok(()) => {
                                marked1[idx1] = true;
                                marked2[idx2] = true;
                                break
                            }
                            Err(..) => {
                                continue
                            }
                        }
                    }
                }

                // unify all unresolved type variables:
                for (idx2, ty2) in types2.iter().enumerate() {
                    if marked2[idx2] { continue }

                    for (idx1, ty1) in types1.iter().enumerate() {
                        if marked1[idx1] { continue }

                        self.unify(ty1, ty2).expect("unifying two unresolved type variables should never fail");
                        marked1[idx1] = true;
                        marked2[idx2] = true;
                    }
                }

                // if by this point not all types are marked, there was a mismatch
                if !(marked1.iter().any(|m| *m) && marked2.iter().any(|m| *m)) {
                    return Err((t1, t2));
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

    fn unify_slice(&self, types1: &[TypeRef<'ty, 'object>], types2: &[TypeRef<'ty, 'object>]) -> Option<UnificationResult<'ty, 'object>> {
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

    pub fn update_loc(&self, ty: TypeRef<'ty, 'object>, loc: Loc) -> TypeRef<'ty, 'object> {
        let tyvar = self.new_var(loc);

        self.unify(tyvar, ty).expect("unifying new tyvar");

        tyvar
    }

    pub fn describe(&self, ty: TypeRef<'ty, 'object>) -> String {
        let mut buffer = String::new();
        ty.describe(self, &mut buffer).unwrap();
        buffer
    }

    pub fn degrade_to_instance(&self, ty: TypeRef<'ty, 'object>) -> TypeRef<'ty, 'object> {
        match self.prune(ty) {
            &Type::KeywordHash { id, ref loc, ref keywords, splat } => {
                let hash_class = self.object.hash_class();

                // degrade keyword hash to instance type:
                let key_ty = self.instance(loc.clone(), self.object.Symbol, vec![]);
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
                let array_class = self.object.array_class();

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
            pruned => pruned,
        }
    }

    pub fn predicate(&self, ty: TypeRef<'ty, 'object>) -> Or<TypeRef<'ty, 'object>, TypeRef<'ty, 'object>> {
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
                types.iter()
                    .map(|t| self.predicate(t))
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
                    let mut instance_params = ty_params.to_vec();
                    let expected_params = class.type_parameters().len();

                    while instance_params.len() < expected_params {
                        instance_params.push(self.new_var(class_loc.clone()));
                    }

                    self.instance(class_loc.clone(), class, instance_params)
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
                partition_inner(self.object.Proc, Some(&[])),
            Type::Tuple { .. } =>
                partition_inner(self.object.array_class(), None),
            Type::KeywordHash { .. } =>
                partition_inner(self.object.hash_class(), None),
            Type::Union { ref types, ref loc, .. } => {
                types.iter()
                    .map(|t| self.partition_by_class(t, class, class_loc))
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
                    self.possible_types_rec(ty, out_tys)
                }
            }
            _ => out_tys.push(ty),
        }
    }

    fn same_unordered_types(&self, tys1: &[TypeRef<'ty, 'object>], tys2: &[TypeRef<'ty, 'object>]) -> bool {
        tys1.iter().all(|ty1| tys2.iter().any(|ty2| self.same_type(ty1, ty2)))
    }

    fn same_types(&self, tys1: &[TypeRef<'ty, 'object>], tys2: &[TypeRef<'ty, 'object>]) -> bool {
        tys1.len() == tys2.len() && tys1.iter().zip(tys2).all(|(t1, t2)| self.same_type(t1, t2))
    }

    pub fn same_type(&self, a: TypeRef<'ty, 'object>, b: TypeRef<'ty, 'object>) -> bool {
        match (self.prune(a), self.prune(b)) {
            (&Type::Instance { class: c1, type_parameters: ref tp1, .. },
                    &Type::Instance { class: c2, type_parameters: ref tp2, .. }) =>
                c1 == c2 && tp1.iter().zip(tp2).all(|(t1, t2)| self.same_type(t1, t2)),

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
        match self.prune(ty) {
            kw_ty@&Type::KeywordHash { .. } => Some(kw_ty),
            &Type::Instance { class, ref type_parameters, .. }
                if self.object.is_hash(class) =>
                    if self.is_instance(type_parameters[0], self.object.Symbol) {
                        Some(self.keyword_hash(ty.loc().clone(), vec![], Some(type_parameters[1])))
                    } else {
                        None
                    },
            _ => None,
        }
    }

    pub fn is_keyword_hash(&self, ty: TypeRef<'ty, 'object>) -> bool {
        match *self.prune(ty) {
            Type::KeywordHash { .. } => true,
            Type::Instance { class, ref type_parameters, .. }
                if self.object.is_hash(class) =>
                    self.is_instance(type_parameters[0], self.object.Symbol),
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
                if self.object.is_hash(class)
                && self.is_instance(type_parameters[0], self.object.Symbol)
                =>
                    KwsplatResult::Ok(type_parameters[1]),
            Type::Union { ref types, .. } =>
                types.iter()
                    .map(|union_ty| self.kwsplat_to_hash(union_ty))
                    .fold(KwsplatResult::None, |a, b| a.append(self, ty.loc(), b)),
            _ if self.is_instance(ty, self.object.NilClass) =>
                KwsplatResult::None,
            _ =>
                KwsplatResult::Err(ty),
        }
    }

    fn elements_from_tuple(&self, tuple: TypeRef<'ty, 'object>) -> Vec<TupleElement<'ty, 'object>> {
        if let Type::Tuple { ref lead, splat, ref post, .. } = *self.prune(tuple) {
            let mut elements = Vec::new();

            elements.extend(lead.iter().map(|&ty| TupleElement::Value(ty)));
            elements.extend(splat.iter().map(|&ty| TupleElement::Splat(ty)));
            elements.extend(post.iter().map(|&ty| TupleElement::Value(ty)));

            elements
        } else {
            panic!("type not a tuple")
        }
    }
}

#[derive(Debug)]
pub enum TupleElement<'ty, 'object: 'ty> {
    Value(TypeRef<'ty, 'object>),
    Splat(TypeRef<'ty, 'object>),
}

pub enum KwsplatResult<'ty, 'object: 'ty> {
    Err(TypeRef<'ty, 'object>),
    None,
    Ok(TypeRef<'ty, 'object>),
}

impl<'ty, 'object: 'ty> KwsplatResult<'ty, 'object> {
    fn append_ty<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>, loc: &Loc, ty: TypeRef<'ty, 'object>)
        -> KwsplatResult<'ty, 'object>
    {
        self.append(tyenv, loc, KwsplatResult::Ok(ty))
    }

    fn append<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>, loc: &Loc, other: KwsplatResult<'ty, 'object>)
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

    pub fn ref_eq(&self, other: TypeRef<'ty, 'object>) -> bool {
        (self as *const _) == (other as *const _)
    }

    pub fn describe<'env>(&'ty self, tyenv: &TypeEnv<'ty, 'env, 'object>, f: &mut fmt::Write) -> fmt::Result {
        match *tyenv.prune(self) {
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

#[derive(Debug)]
pub struct Prototype<'ty, 'object: 'ty> {
    pub loc: Loc,
    pub args: Vec<Arg<'ty, 'object>>,
    pub retn: TypeRef<'ty, 'object>,
}

impl<'ty, 'object> Prototype<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        &self.loc
    }

    pub fn describe<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>, f: &mut fmt::Write) -> fmt::Result {
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

    pub fn describe<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>, f: &mut fmt::Write) -> fmt::Result {
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
