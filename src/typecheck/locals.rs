use std::fmt;
use std::rc::Rc;
use std::collections::HashSet;
use immutable_map::TreeMap;

use typecheck::types::{TypeEnv, Type};

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
    autopin: usize,
}

#[derive(Clone)]
pub struct Locals<'ty, 'object: 'ty>(Rc<Locals_<'ty, 'object>>);

impl<'ty, 'object> fmt::Debug for Locals<'ty, 'object> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'ty, 'object> Locals<'ty, 'object> {
    fn new_(l: Locals_<'ty, 'object>) -> Locals<'ty, 'object>{
        Locals(Rc::new(l))
    }

    pub fn new() -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: None, vars: TreeMap::new(), autopin: 0 })
    }

    pub fn extend(&self) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: Some(self.clone()), vars: TreeMap::new(), autopin: 0 })
    }

    pub fn unextend(&self) -> Locals<'ty, 'object> {
        self.0.parent.as_ref().expect("unbalanced extend/unextend (parent is None)").clone()
    }

    pub fn autopin(&self) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: self.0.parent.clone(), vars: self.0.vars.clone(), autopin: self.0.autopin + 1 })
    }

    pub fn unautopin(&self) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: self.0.parent.clone(), vars: self.0.vars.clone(), autopin: self.0.autopin - 1 })
    }

    fn update_parent(&self, parent: Option<Locals<'ty, 'object>>) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: parent, vars: self.0.vars.clone(), autopin: self.0.autopin })
    }

    fn update_vars(&self, vars: TreeMap<String, LocalEntry<'ty, 'object>>) -> Locals<'ty, 'object> {
        Self::new_(Locals_ { parent: self.0.parent.clone(), vars: vars, autopin: self.0.autopin })
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
                LocalEntry::Bound(_) if self.0.autopin == 0 => (None, self.insert_var(name, LocalEntry::Bound(ty))),
                LocalEntry::Bound(ty) => (Some(ty), self.insert_var(name, LocalEntry::Pinned(ty))),
                LocalEntry::Pinned(ty) => (Some(ty), self.clone()),
                LocalEntry::ConditionallyPinned(ty) => (Some(ty), self.clone()),
                LocalEntry::Unbound => panic!("should not happen"),
            }
        }

        if let Some(ref parent) = self.0.parent {
            let (entry, locals) = parent.update_upvar(&name, &|local| {
                match *local {
                    LocalEntry::Bound(ty) |
                    LocalEntry::Pinned(ty) |
                    LocalEntry::ConditionallyPinned(ty) => LocalEntry::Pinned(ty),
                    LocalEntry::Unbound => panic!("should not happen"),
                }
            });

            if let LocalEntry::Pinned(pinned_ty) = entry {
                return (Some(pinned_ty), locals.map(|l| self.update_parent(Some(l))).unwrap_or_else(|| self.clone()))
            }
        }

        (None, self.insert_var(name, LocalEntry::Bound(ty)))
    }

    pub fn refine(&self, name: &str, ty: &'ty Type<'ty, 'object>) -> Locals<'ty, 'object> {
        match self.get_var_direct(&name) {
            LocalEntry::Unbound => {
                // TODO - can't refine type of variable not in the immediate scope
                self.clone()
            }
            LocalEntry::Bound(_) => self.insert_var(name.to_owned(), LocalEntry::Bound(ty)),
            LocalEntry::Pinned(_) => self.clone(),
            LocalEntry::ConditionallyPinned(_) => self.clone(),
        }
    }

    pub fn merge<'env>(&self, other: Locals<'ty, 'object>, tyenv: &TypeEnv<'ty, 'env, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Locals<'ty, 'object> {
        assert!(self.0.autopin == other.0.autopin);

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
