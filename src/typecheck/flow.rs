use std::fmt;
use std::rc::Rc;
use typecheck::types::{TypeEnv, Type};
use ast::Loc;
use immutable_map::TreeMap;
use util::Or;
use std::collections::HashSet;

#[derive(Debug,Clone)]
pub enum LocalEntry<'ty, 'object: 'ty> {
    Unbound,
    Bound(&'ty Type<'ty, 'object>),
    Pinned(&'ty Type<'ty, 'object>),
    ConditionallyPinned(&'ty Type<'ty, 'object>),
}

#[derive(Debug,Clone)]
pub enum LocalEntryMerge<'ty, 'object: 'ty> {
    Ok(LocalEntry<'ty, 'object>),
    MustMatch(LocalEntry<'ty, 'object>, &'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>)
}

impl<'ty, 'object> LocalEntry<'ty, 'object> {
    pub fn merge<'env>(self, other: LocalEntry<'ty, 'object>, tyenv: &TypeEnv<'ty, 'env, 'object>) -> LocalEntryMerge<'ty, 'object> {
        match (self, other) {
            (LocalEntry::Unbound, LocalEntry::Unbound) =>
                LocalEntryMerge::Ok(LocalEntry::Unbound),
            (LocalEntry::Unbound, LocalEntry::Bound(ty)) =>
                LocalEntryMerge::Ok(LocalEntry::Bound(tyenv.nillable(ty.loc(), ty))),
            (LocalEntry::Unbound, LocalEntry::Pinned(ty)) =>
                LocalEntryMerge::Ok(LocalEntry::ConditionallyPinned(ty)),
            (LocalEntry::Unbound, LocalEntry::ConditionallyPinned(ty)) =>
                LocalEntryMerge::Ok(LocalEntry::ConditionallyPinned(ty)),

            (LocalEntry::Bound(ty), LocalEntry::Unbound) =>
                LocalEntryMerge::Ok(LocalEntry::Bound(tyenv.nillable(ty.loc(), ty))),
            (LocalEntry::Bound(tya), LocalEntry::Bound(tyb)) =>
                LocalEntryMerge::Ok(LocalEntry::Bound(tyenv.union(tya.loc() /* TODO incorporate tyb too */, tya, tyb))),
            (LocalEntry::Bound(bound_ty), LocalEntry::Pinned(pinned_ty)) =>
                LocalEntryMerge::MustMatch(LocalEntry::Pinned(pinned_ty), pinned_ty, bound_ty),
            (LocalEntry::Bound(bound_ty), LocalEntry::ConditionallyPinned(pinned_ty)) =>
                LocalEntryMerge::MustMatch(LocalEntry::ConditionallyPinned(pinned_ty), pinned_ty, bound_ty),

            (LocalEntry::Pinned(pinned_ty), LocalEntry::Unbound) =>
                LocalEntryMerge::Ok(LocalEntry::ConditionallyPinned(pinned_ty)),
            (LocalEntry::Pinned(pinned_ty), LocalEntry::Bound(bound_ty)) =>
                LocalEntryMerge::MustMatch(LocalEntry::Pinned(pinned_ty), pinned_ty, bound_ty),
            (LocalEntry::Pinned(tya), LocalEntry::Pinned(tyb)) =>
                LocalEntryMerge::MustMatch(LocalEntry::Pinned(tya), tya, tyb),
            (LocalEntry::Pinned(tya), LocalEntry::ConditionallyPinned(tyb)) =>
                LocalEntryMerge::MustMatch(LocalEntry::ConditionallyPinned(tyb), tyb, tya),

            (LocalEntry::ConditionallyPinned(pinned_ty), LocalEntry::Unbound) =>
                LocalEntryMerge::Ok(LocalEntry::ConditionallyPinned(pinned_ty)),
            (LocalEntry::ConditionallyPinned(pinned_ty), LocalEntry::Bound(bound_ty)) =>
                LocalEntryMerge::MustMatch(LocalEntry::ConditionallyPinned(pinned_ty), pinned_ty, bound_ty),
            (LocalEntry::ConditionallyPinned(tya), LocalEntry::Pinned(tyb)) =>
                LocalEntryMerge::MustMatch(LocalEntry::ConditionallyPinned(tya), tya, tyb),
            (LocalEntry::ConditionallyPinned(tya), LocalEntry::ConditionallyPinned(tyb)) =>
                LocalEntryMerge::MustMatch(LocalEntry::ConditionallyPinned(tya), tya, tyb),
        }
    }
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

    fn get_var_direct(&self, name: &str) -> LocalEntry<'ty, 'object> {
        match self.0.vars.get(name) {
            Some(entry) => entry.clone(),
            None => LocalEntry::Unbound,
        }
    }

    fn insert_var(&self, name: String, entry: LocalEntry<'ty, 'object>) -> Locals<'ty, 'object> {
        self.update_vars(self.0.vars.insert(name, entry))
    }

    fn update_upvar<F>(&self, name: &str, f: &F) -> (LocalEntry<'ty, 'object>, Option<Locals<'ty, 'object>>)
        where F: Fn(&LocalEntry<'ty, 'object>) -> (LocalEntry<'ty, 'object>)
    {
        if let Some(local) = self.0.vars.get(name) {
            let new_local = f(local);

            (new_local.clone(), Some(self.insert_var(name.to_owned(), new_local)))
        } else if let Some(ref parent) = self.0.parent {
            let (x, parent) = parent.update_upvar(name, f);

            (x, parent.map(|parent| self.update_parent(Some(parent))))
        } else {
            (LocalEntry::Unbound, None)
        }
    }

    pub fn lookup(&self, name: &str) -> (LocalEntry<'ty, 'object>, Locals<'ty, 'object>) {
        if let Some(local) = self.0.vars.get(name) {
            (local.clone(), self.clone())
        } else {
            let updated = self.update_upvar(name, &|local|
                match *local {
                    LocalEntry::Unbound => LocalEntry::Unbound,
                    LocalEntry::Bound(ty) => LocalEntry::Pinned(ty),
                    LocalEntry::Pinned(ty) => LocalEntry::Pinned(ty),
                    LocalEntry::ConditionallyPinned(ty) => LocalEntry::ConditionallyPinned(ty),
                }
            );

            match updated {
                (x, Some(locals)) => (x, locals),
                (x, None) => (x, self.clone()),
            }
        }
    }

    pub fn assign_shadow(&self, name: String, ty: &'ty Type<'ty, 'object>) -> Locals<'ty, 'object> {
        self.insert_var(name, LocalEntry::Bound(ty))
    }

    pub fn assign(&self, name: String, ty: &'ty Type<'ty, 'object>) -> (Option<&'ty Type<'ty, 'object>>, Locals<'ty, 'object>) {
        if let Some(local) = self.0.vars.get(&name) {
            return match *local {
                LocalEntry::Bound(_) => (None, self.insert_var(name, LocalEntry::Bound(ty))),
                LocalEntry::Pinned(ty) => (Some(ty), self.clone()),
                LocalEntry::ConditionallyPinned(ty) => (Some(ty), self.clone()),
                LocalEntry::Unbound => panic!("should not happen"),
            }
        }

        if let Some(ref parent) = self.0.parent {
            let (entry, locals) = parent.update_upvar(&name, &|local| {
                match *local {
                    LocalEntry::Bound(_) => LocalEntry::Pinned(ty),
                    LocalEntry::Pinned(ty) => LocalEntry::Pinned(ty),
                    LocalEntry::ConditionallyPinned(ty) => LocalEntry::Pinned(ty),
                    LocalEntry::Unbound => panic!("should not happen"),
                }
            });

            if let LocalEntry::Pinned(pinned_ty) = entry {
                return (Some(pinned_ty), locals.unwrap_or_else(|| self.clone()))
            }
        }

        (None, self.insert_var(name, LocalEntry::Bound(ty)))
    }

    pub fn refine(&self, name: String, ty: &'ty Type<'ty, 'object>) -> Locals<'ty, 'object> {
        match self.get_var_direct(&name) {
            LocalEntry::Unbound => panic!("should not happen"),
            LocalEntry::Bound(_) => self.insert_var(name, LocalEntry::Bound(ty)),
            LocalEntry::Pinned(_) => self.clone(),
            LocalEntry::ConditionallyPinned(_) => self.clone(),
        }
    }

    pub fn merge<'env>(&self, other: Locals<'ty, 'object>, tyenv: &TypeEnv<'ty, 'env, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Locals<'ty, 'object> {
        let mut names = HashSet::new();
        names.extend(self.0.vars.keys());
        names.extend(other.0.vars.keys());

        let vars = names.into_iter().fold(TreeMap::new(), |map, name| {
            let merge = self.get_var_direct(name).merge(other.get_var_direct(name), tyenv);

            merges.push(merge.clone());

            match merge {
                LocalEntryMerge::Ok(entry) |
                LocalEntryMerge::MustMatch(entry, ..) =>
                    map.insert(name.clone(), entry)
            }
        });

        self.update_vars(vars)
    }
}

impl<'ty, 'object> PartialEq for Locals<'ty, 'object> {
    fn eq(&self, other: &Locals<'ty, 'object>) -> bool {
        (&*self.0 as *const _) == (&*other.0 as *const _)
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

    pub fn has_results(&self) -> bool {
        match *self.0 {
            Computation_::Result(..) => true,
            Computation_::Divergent(ref a, ref b) => a.has_results() || b.has_results(),
            _ => false,
        }
    }

    pub fn converge_results<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Result(..) |
            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry => self.clone(),

            Computation_::Divergent(ref a, ref b) => {
                let a = a.converge_results(loc, tyenv, merges);
                let b = b.converge_results(loc, tyenv, merges);

                if let Computation_::Result(a_ty, ref a_l) = *a.0 {
                    if let Computation_::Result(b_ty, ref b_l) = *b.0 {
                        return Computation::result(tyenv.union(loc, a_ty, b_ty), a_l.merge(b_l.clone(), tyenv, merges));
                    }

                    if let Computation_::Divergent(ref ba, ref bb) = *b.0 {
                        if let Computation_::Result(ba_ty, ref ba_l) = *ba.0 {
                            return Computation::divergent(
                                Computation::result(tyenv.union(loc, a_ty, ba_ty), a_l.merge(ba_l.clone(), tyenv, merges)),
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

    pub fn extract_results<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>)
        -> Or<(&'ty Type<'ty, 'object>, Locals<'ty, 'object>), Computation<'ty, 'object>>
    {
        let converged = self.converge_results(loc, tyenv, merges);

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

    pub fn predicate<'env>(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'env, 'object>) -> ComputationPredicate<'ty, 'object> {
        fn refine_computation<'ty, 'object: 'ty>(ty: &'ty Type<'ty, 'object>, refined_ty: &'ty Type<'ty, 'object>, locals: &Locals<'ty, 'object>) -> Computation<'ty, 'object> {
            let locals = if let Type::LocalVariable { ref name, .. } = *ty {
                locals.refine(name.clone(), refined_ty)
            } else {
                locals.clone()
            };

            Computation::result(refined_ty, locals)
        }

        match *self.0 {
            Computation_::Result(ty, ref locals) => {
                match tyenv.predicate(ty) {
                    Or::Left(tya) => ComputationPredicate::result(Some(refine_computation(ty, tya, locals)), None),
                    Or::Right(tyb) => ComputationPredicate::result(None, Some(refine_computation(ty, tyb, locals))),
                    Or::Both(tya, tyb) => {
                        let compa = refine_computation(ty, tya, locals);
                        let compb = refine_computation(ty, tyb, locals);
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
