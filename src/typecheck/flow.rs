use std::fmt;
use std::rc::Rc;
use typecheck::types::{TypeEnv, Type};
use ast::Loc;
use immutable_map::TreeMap;
use util::Or;

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

#[derive(Debug)]
pub struct ComputationPredicate<'ty, 'object: 'ty> {
    pub truthy: Option<Computation<'ty, 'object>>,
    pub falsy: Option<Computation<'ty, 'object>>,
    pub non_result: Option<Computation<'ty, 'object>>,
}

impl<'ty, 'object> ComputationPredicate<'ty, 'object> {
    pub fn empty() -> ComputationPredicate<'ty, 'object> {
        ComputationPredicate {
            truthy: None,
            falsy: None,
            non_result: None,
        }
    }

    pub fn result(truthy: Option<Computation<'ty, 'object>>, falsy: Option<Computation<'ty, 'object>>) -> ComputationPredicate<'ty, 'object> {
        ComputationPredicate {
            truthy: truthy,
            falsy: falsy,
            non_result: None,
        }
    }

    pub fn non_result(comp: Computation<'ty, 'object>) -> ComputationPredicate<'ty, 'object> {
        ComputationPredicate {
            truthy: None,
            falsy: None,
            non_result: Some(comp),
        }
    }

    pub fn append(self, other: ComputationPredicate<'ty, 'object>) -> ComputationPredicate<'ty, 'object> {
        ComputationPredicate {
            truthy: Computation::divergent_option(self.truthy, other.truthy),
            falsy: Computation::divergent_option(self.falsy, other.falsy),
            non_result: Computation::divergent_option(self.non_result, other.non_result),
        }
    }
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
        let _ = other;
        panic!("TODO");
    }
}

#[derive(Debug)]
enum Computation_<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Locals<'ty, 'object>),
    Return(&'ty Type<'ty, 'object>),
    Redo,
    Retry,
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

    pub fn redo() -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Redo))
    }

    pub fn retry() -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Retry))
    }

    pub fn divergent(a: Computation<'ty, 'object>, b: Computation<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Divergent(a, b)))
    }

    pub fn divergent_option(a: Option<Computation<'ty, 'object>>, b: Option<Computation<'ty, 'object>>) -> Option<Computation<'ty, 'object>> {
        match (a, b) {
            (Some(a), Some(b)) => Some(Computation::divergent(a, b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub fn seq<F>(&self, f: &F) -> Computation<'ty, 'object>
        where F: Fn(&'ty Type<'ty, 'object>, Locals<'ty, 'object>) -> Computation<'ty, 'object>
    {
        match *self.0 {
            Computation_::Result(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation_::Return(_) => self.clone(),
            Computation_::Redo |
            Computation_::Retry => self.clone(),
            Computation_::Divergent(ref a, ref b) => Self::divergent(a.seq(f), b.seq(f)),
        }
    }

    pub fn terminate<F>(&self, f: &F)
        where F: Fn(&'ty Type<'ty, 'object>)
    {
        match *self.0 {
            Computation_::Result(ref ty, _) |
            Computation_::Return(ref ty) => f(ty.clone()),
            Computation_::Redo |
            Computation_::Retry => {},
            Computation_::Divergent(ref a, ref b) => {
                a.terminate(f);
                b.terminate(f);
            },
        }
    }

    pub fn capture_next(&self) -> Computation<'ty, 'object> {
        // TODO when Computation_::Next is implemented this needs to turn
        // Next computations into Result computations:
        self.clone()
    }

    pub fn converge_results<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Result(..) |
            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry => self.clone(),

            Computation_::Divergent(ref a, ref b) => {
                let a = a.converge_results(loc, tyenv);
                let b = b.converge_results(loc, tyenv);

                if let Computation_::Result(a_ty, ref a_l) = *a.0 {
                    if let Computation_::Result(b_ty, ref b_l) = *b.0 {
                        return Computation::result(tyenv.union(loc, a_ty, b_ty), a_l.merge(b_l.clone()));
                    }

                    if let Computation_::Divergent(ref ba, ref bb) = *b.0 {
                        if let Computation_::Result(ba_ty, ref ba_l) = *ba.0 {
                            return Computation::divergent(
                                Computation::result(tyenv.union(loc, a_ty, ba_ty), a_l.merge(ba_l.clone())),
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

    pub fn extract_results<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>)
        -> Or<(&'ty Type<'ty, 'object>, Locals<'ty, 'object>), Computation<'ty, 'object>>
    {
        let converged = self.converge_results(loc, tyenv);

        match *converged.0 {
            Computation_::Result(ty, ref locals) => Or::Left((ty, locals.clone())),

            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry => Or::Right(converged.clone()),

            Computation_::Divergent(ref a, ref b) => {
                // if there were any result computations, converge_results
                // guarantees that they will have been collapsed into the
                // left hand side of the divergent computation it returns.
                if let Computation_::Result(ty, ref locals) = *a.0 {
                    Or::Both((ty, locals.clone()), b.clone())
                } else {
                    Or::Right(converged.clone())
                }
            }
        }
    }

    pub fn result_type<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>) -> Option<&'ty Type<'ty, 'object>> {
        match self.extract_results(loc, tyenv) {
            Or::Left((ty, _)) => Some(ty),
            Or::Both((ty, _), _) => Some(ty),
            Or::Right(_) => None,
        }
    }

    pub fn predicate<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>) -> ComputationPredicate<'ty, 'object> {
        match *self.0 {
            Computation_::Result(ty, ref locals) => {
                match tyenv.predicate(ty) {
                    Or::Left(ty) => ComputationPredicate::result(Some(Computation::result(ty, locals.clone())), None),
                    Or::Right(ty) => ComputationPredicate::result(None, Some(Computation::result(ty, locals.clone()))),
                    Or::Both(tya, tyb) => {
                        let compa = Computation::result(tya, locals.clone());
                        let compb = Computation::result(tyb, locals.clone());
                        ComputationPredicate::result(Some(compa), Some(compb))
                    }
                }
            },
            Computation_::Divergent(ref a, ref b) => {
                a.predicate(loc, tyenv).append(b.predicate(loc, tyenv))
            }
            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry => {
                ComputationPredicate::non_result(self.clone())
            }
        }
    }
}
