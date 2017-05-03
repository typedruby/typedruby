use std::fmt;
use std::rc::Rc;
use typecheck::types::{TypeEnv, Type};
use immutable_map::TreeMap;

#[derive(Debug,Clone)]
pub struct LocalEntry<'ty, 'object: 'ty> {
    pub ty: &'ty Type<'ty, 'object>,
    pub pinned: bool,
}

#[derive(Debug)]
pub struct Locals_<'ty, 'object: 'ty> {
    parent: Option<Locals<'ty, 'object>>,
    vars: TreeMap<String, LocalEntry<'ty, 'object>>,
}

#[derive(Debug,Clone)]
pub struct Locals<'ty, 'object: 'ty>(Rc<Locals_<'ty, 'object>>);

impl<'ty, 'object> Locals<'ty, 'object> {
    fn new_(l: Locals_<'ty, 'object>) -> Locals<'ty, 'object>{
        Locals(Rc::new(l))
    }

    pub fn new() -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: None, vars: TreeMap::new() })
    }

    pub fn extend(&self) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: Some(Locals(self.0.clone())), vars: TreeMap::new() })
    }

    pub fn unextend(&self) -> Locals<'ty, 'object> {
        self.0.parent.as_ref().expect("unbalanced extend/unextend (parent is None)").clone()
    }

    fn update_parent(&self, parent: Option<Locals<'ty, 'object>>) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: parent, vars: self.0.vars.clone() })
    }

    fn update_vars(&self, vars: TreeMap<String, LocalEntry<'ty, 'object>>) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: self.0.parent.clone(), vars: vars })
    }

    fn insert_var(&self, name: String, entry: LocalEntry<'ty, 'object>) -> Locals<'ty, 'object> {
        self.update_vars(self.0.vars.insert(name, entry))
    }

    fn update_upvar<F, T>(&self, name: &str, f: &F) -> (Option<T>, Option<Locals<'ty, 'object>>)
        where F: Fn(&LocalEntry<'ty, 'object>) -> (T, Option<LocalEntry<'ty, 'object>>)
    {
        if let Some(local) = self.0.vars.get(name) {
            match f(local) {
                (x, Some(new_local)) => {
                    (Some(x), Some(self.insert_var(name.to_owned(), new_local)))
                },
                (x, None) => (Some(x), None),
            }
        } else if let Some(ref parent) = self.0.parent {
            let (x, parent) = parent.update_upvar(name, f);

            (x, parent.map(|parent| self.update_parent(Some(parent))))
        } else {
            (None, None)
        }
    }

    pub fn lookup(&self, name: &str) -> (Option<&'ty Type<'ty, 'object>>, Locals<'ty, 'object>) {
        if let Some(local) = self.0.vars.get(name) {
            (Some(local.ty), self.clone())
        } else {
            let result = self.update_upvar(name, &|local|
                if local.pinned {
                    // no need to repin
                    (local.ty, None)
                } else {
                    (local.ty, Some(LocalEntry { ty: local.ty, pinned: true }))
                }
            );

            match result {
                (ty, None) => (ty, self.clone()),
                (ty, Some(parent)) => (ty, self.update_parent(Some(parent))),
            }
        }
    }

    pub fn assign_shadow(&self, name: String, ty: &'ty Type<'ty, 'object>) -> Locals<'ty, 'object> {
        self.insert_var(name, LocalEntry { ty: ty, pinned: false })
    }

    pub fn assign(&self, name: String, ty: &'ty Type<'ty, 'object>) -> (Option<&'ty Type<'ty, 'object>>, Locals<'ty, 'object>) {
        if let Some(local) = self.0.vars.get(&name) {
            if local.pinned {
                return (Some(local.ty), self.clone());
            } else {
                return (None, self.insert_var(name, LocalEntry {
                    pinned: false,
                    ty: ty,
                }));
            }
        }

        if let Some(ref parent) = self.0.parent {
            if let ret@(Some(_), _) = parent.lookup(&name) {
                return ret;
            }
        }

        (None, self.insert_var(name, LocalEntry { pinned: false, ty: ty }))
    }

    pub fn merge(&self, other: Locals<'ty, 'object>) -> Locals<'ty, 'object> {
        panic!("unimplemented");
    }
}

#[derive(Debug)]
enum Computation_<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Locals<'ty, 'object>),
    Return(&'ty Type<'ty, 'object>),
    Divergent(Computation<'ty, 'object>, Computation<'ty, 'object>),
}

#[derive(Clone)]
pub struct Computation<'ty, 'object: 'ty>(Rc<Computation_<'ty, 'object>>);

impl<'ty, 'object: 'ty> fmt::Debug for Computation<'ty, 'object> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'ty, 'object: 'ty> Computation<'ty, 'object> {
    pub fn result(ty: &'ty Type<'ty, 'object>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Result(ty, locals)))
    }

    pub fn return_(ty: &'ty Type<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Return(ty)))
    }

    pub fn divergent(a: Computation<'ty, 'object>, b: Computation<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Divergent(a, b)))
    }

    pub fn seq<F>(&self, f: &F) -> Computation<'ty, 'object>
        where F: Fn(&'ty Type<'ty, 'object>, Locals<'ty, 'object>) -> Computation<'ty, 'object>
    {
        match *self.0 {
            Computation_::Result(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation_::Return(_) => self.clone(),
            Computation_::Divergent(ref a, ref b) => Self::divergent(a.seq(f), b.seq(f)),
        }
    }

    pub fn terminate<F>(&self, f: &F)
        where F: Fn(&'ty Type<'ty, 'object>)
    {
        match *self.0 {
            Computation_::Result(ref ty, _) |
            Computation_::Return(ref ty) => f(ty.clone()),
            Computation_::Divergent(ref a, ref b) => {
                a.terminate(f);
                b.terminate(f);
            },
        }
    }

    pub fn converge_results<'env>(&self, tyenv: &TypeEnv<'ty, 'env, 'object>) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Result(..) => self.clone(),
            Computation_::Return(..) => self.clone(),
            Computation_::Divergent(ref a, ref b) => {
                let a = a.converge_results(tyenv);
                let b = b.converge_results(tyenv);

                if let Computation_::Result(a_ty, ref a_l) = *a.0 {
                    if let Computation_::Result(b_ty, ref b_l) = *b.0 {
                        return Computation::result(tyenv.union(a_ty, b_ty), a_l.merge(b_l.clone()));
                    }

                    if let Computation_::Divergent(ref ba, ref bb) = *b.0 {
                        if let Computation_::Result(ba_ty, ref ba_l) = *ba.0 {
                            return Computation::divergent(
                                Computation::result(tyenv.union(a_ty, ba_ty), a_l.merge(ba_l.clone())),
                                bb.clone());
                        }
                    }
                } else if let Computation_::Result(..) = *b.0 {
                    return Computation::divergent(b.clone(), a.clone());
                }

                return Computation::divergent(a.clone(), b.clone());
            }
        }
    }
}
