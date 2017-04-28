extern crate immutable_map;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use ast::Loc;
use object::{ObjectGraph, RubyObject};
use typed_arena::Arena;
use self::immutable_map::TreeMap;

pub type TypeVarId = usize;

pub type UnificationResult<'ty, 'env, 'object> = Result<TypeEnv<'ty, 'env, 'object>, (&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>)>;

#[derive(Clone)]
pub struct TypeEnv<'ty, 'env, 'object: 'ty + 'env> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    next_id: Rc<Cell<TypeVarId>>,
    instance_map: TreeMap<TypeVarId, &'ty Type<'ty, 'object>>,
    object: &'env ObjectGraph<'object>,
}

impl<'ty, 'env, 'object: 'env> TypeEnv<'ty, 'env, 'object> {
    pub fn new(arena: &'ty Arena<Type<'ty, 'object>>, object: &'env ObjectGraph<'object>) -> TypeEnv<'ty, 'env, 'object> {
        TypeEnv {
            arena: arena,
            object: object,
            instance_map: TreeMap::new(),
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

    fn set_var(&self, id: TypeVarId, ty: &'ty Type<'ty, 'object>) -> TypeEnv<'ty, 'env, 'object> {
        TypeEnv {
            arena: self.arena,
            object: self.object,
            instance_map: self.instance_map.insert_or_update(id, ty.clone(), |v|
                panic!("attempted to set typevar {} to {:?}, but is already {:?}",
                    id, ty, v)
            ),
            next_id: self.next_id.clone(),
        }
    }

    pub fn prune(&self, ty: &'ty Type<'ty, 'object>) -> &'ty Type<'ty, 'object> {
        if let Type::Var { ref loc, ref id } = *ty {
            if let Some(instance) = self.instance_map.get(id) {
                return self.prune(instance)
            }
        }

        ty.clone()
    }

    pub fn unify(&self, t1: &'ty Type<'ty, 'object>, t2: &'ty Type<'ty, 'object>) -> UnificationResult<'ty, 'env, 'object> {
        let t1 = self.prune(t1);
        let t2 = self.prune(t2);

        match (t1, t2) {
            (&Type::Var { id: ref id1, .. }, _) => {
                if let Type::Var { id: ref id2, .. } = *t2 {
                    if id1 == id2 {
                        // already unified
                        return Ok(self.clone());
                    }
                }

                Ok(self.set_var(*id1, t2.clone()))
            },

            (_, &Type::Var { .. }) =>
                self.unify(&t2, &t1),

            (&Type::Instance { class: ref class1, type_parameters: ref tp1, .. }, &Type::Instance { class: ref class2, type_parameters: ref tp2, .. }) => {
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
                            (Some(ref a), Some(ref b)) => Some(res.and_then(|env|
                                env.unify(a, b)
                            )),
                            (None, None) => Some(res),
                            _ => None,
                        }
                    ).and_then(|res|
                        match res {
                            Ok(env) => env.unify_slice(post1, post2),
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
                Ok(self.clone()),

            (&Type::Any { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            (&Type::TypeParameter { name: ref name1, .. }, &Type::TypeParameter { name: ref name2, .. }) =>
                if name1 == name2 {
                    Ok(self.clone())
                } else {
                    Err((t1.clone(), t2.clone()))
                },

            (&Type::TypeParameter { .. }, _) =>
                Err((t1.clone(), t2.clone())),

            _ => panic!("unify! t1={:?}, t2={:?}", t1, t2),
        }
    }

    fn unify_slice(&self, types1: &[&'ty Type<'ty, 'object>], types2: &[&'ty Type<'ty, 'object>]) -> Option<UnificationResult<'ty, 'env, 'object>> {
        let mut env = self.clone();

        if types1.len() != types2.len() {
            return None;
        }

        for (a, b) in types1.iter().zip(types2.iter()) {
            match env.unify(a, b) {
                Ok(new_env) => env = new_env,
                err@Err(..) => return Some(err),
            }
        }

        Some(Ok(env))
    }

    fn unify_option(&self, opt1: &Option<&'ty Type<'ty, 'object>>, opt2: &Option<&'ty Type<'ty, 'object>>) -> Option<UnificationResult<'ty, 'env, 'object>> {
        match (*opt1, *opt2) {
            (Some(ref t1), Some(ref t2)) => Some(self.unify(t1, t2)),
            (None, None) => Some(Ok(self.clone())),
            _ => None,
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
    },
    Proc {
        loc: Loc,
        args: Vec<&'ty Type<'ty, 'object>>,
        block: Option<&'ty Type<'ty, 'object>>,
        retn: &'ty Type<'ty, 'object>,
    },
    Var {
        loc: Loc,
        id: TypeVarId,
    }
}
