use std::fmt;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use std::cmp::PartialEq;
use std::hash::{Hash, Hasher};
use std::iter::{Iterator, IntoIterator};

use typecheck::types::{TypeEnv, TypeRef};

#[derive(Debug,Clone)]
pub enum LocalEntry<'ty, 'object: 'ty> {
    Unbound,
    Bound(TypeRef<'ty, 'object>),
    Pinned(TypeRef<'ty, 'object>),
    ConditionallyPinned(TypeRef<'ty, 'object>),
}

#[derive(Debug,Clone)]
pub enum LocalEntryMerge<'ty, 'object: 'ty> {
    Ok(LocalEntry<'ty, 'object>),
    MustMatch(LocalEntry<'ty, 'object>, TypeRef<'ty, 'object>, TypeRef<'ty, 'object>)
}

impl<'ty, 'object> LocalEntry<'ty, 'object> {
    pub fn merge(self, other: LocalEntry<'ty, 'object>, tyenv: &TypeEnv<'ty, 'object>) -> LocalEntryMerge<'ty, 'object> {
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
struct LocalNode<'ty, 'object: 'ty> {
    name: String,
    entry: LocalEntry<'ty, 'object>,
    next: LocalTable<'ty, 'object>,
}

#[derive(Debug,Clone)]
struct LocalTable<'ty, 'object: 'ty> {
    node: Option<Rc<LocalNode<'ty, 'object>>>,
}

impl<'ty, 'object> LocalTable<'ty, 'object> {
    pub fn new() -> Self {
        LocalTable { node: None }
    }

    fn extend(&self, node: LocalNode<'ty, 'object>) -> Self {
        LocalTable { node: Some(Rc::new(node)) }
    }

    pub fn insert(&self, name: String, entry: LocalEntry<'ty, 'object>) -> Self {
        self.extend(LocalNode { name: name, entry: entry, next: self.clone() })
    }

    pub fn get(&self, name: &str) -> Option<LocalEntry<'ty, 'object>> {
        let mut tbl = self;

        while let Some(ref node) = tbl.node {
            if node.name == name {
                return Some(node.entry.clone());
            }

            tbl = &node.next;
        }

        None
    }

    pub fn bindings_since(&self, since: &LocalTable<'ty, 'object>) -> HashMap<String, LocalEntry<'ty, 'object>> {
        self.iter()
            .take_while(|tbl| tbl != since)
            .map(|tbl| {
                let node = tbl.node.as_ref().expect("node to be Some given we're not yet at LCA");
                (node.name.clone(), node.entry.clone())
            })
            .fold(HashMap::new(), |mut map, (name, entry)| {
                map.entry(name).or_insert(entry);
                map
            })
    }

    pub fn identity_key(&self) -> Option<*const LocalNode<'ty, 'object>> {
        self.node.as_ref().map(|rc| Rc::as_ref(rc) as *const _)
    }

    pub fn ref_eq(&self, other: &LocalTable<'ty, 'object>) -> bool {
        self.identity_key() == other.identity_key()
    }

    pub fn iter(&self) -> LocalTableIterator<'ty, 'object> {
        self.clone().into_iter()
    }
}

impl<'ty, 'object> Hash for LocalTable<'ty, 'object> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.identity_key().hash(state)
    }
}

impl<'ty, 'object> PartialEq for LocalTable<'ty, 'object> {
    fn eq(&self, other: &Self) -> bool {
        self.identity_key() == other.identity_key()
    }
}

impl<'ty, 'object> Eq for LocalTable<'ty, 'object> {}

struct LocalTableIterator<'ty, 'object: 'ty> {
    tbl: Option<LocalTable<'ty, 'object>>,
}

impl<'ty, 'object> Iterator for LocalTableIterator<'ty, 'object> {
    type Item = LocalTable<'ty, 'object>;

    fn next(&mut self) -> Option<Self::Item> {
        let tbl = self.tbl.clone();
        self.tbl = tbl.as_ref().and_then(|tbl| tbl.node.as_ref().map(|node| node.next.clone()));
        tbl
    }
}

impl<'ty, 'object> IntoIterator for LocalTable<'ty, 'object> {
    type IntoIter = LocalTableIterator<'ty, 'object>;
    type Item = LocalTable<'ty, 'object>;

    fn into_iter(self) -> Self::IntoIter {
        LocalTableIterator { tbl: Some(self) }
    }
}

#[derive(Debug)]
struct LocalScope<'ty, 'object: 'ty> {
    parent: Option<Locals<'ty, 'object>>,
    vars: LocalTable<'ty, 'object>,
    autopin: usize,
}

#[derive(Clone)]
pub struct Locals<'ty, 'object: 'ty> {
    sc: Rc<LocalScope<'ty, 'object>>,
}

impl<'ty, 'object> fmt::Debug for Locals<'ty, 'object> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.sc.fmt(f)
    }
}

impl<'ty, 'object> Locals<'ty, 'object> {
    fn new_(l: LocalScope<'ty, 'object>) -> Locals<'ty, 'object>{
        Locals { sc: Rc::new(l) }
    }

    pub fn new() -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: None, vars: LocalTable::new(), autopin: 0 })
    }

    pub fn extend(&self) -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: Some(self.clone()), vars: LocalTable::new(), autopin: 0 })
    }

    pub fn unextend(&self) -> Locals<'ty, 'object> {
        self.sc.parent.as_ref().expect("unbalanced extend/unextend (parent is None)").clone()
    }

    pub fn autopin(&self) -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: self.sc.parent.clone(), vars: self.sc.vars.clone(), autopin: self.sc.autopin + 1 })
    }

    pub fn unautopin(&self) -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: self.sc.parent.clone(), vars: self.sc.vars.clone(), autopin: self.sc.autopin - 1 })
    }

    fn update_parent(&self, parent: Option<Locals<'ty, 'object>>) -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: parent, vars: self.sc.vars.clone(), autopin: self.sc.autopin })
    }

    fn update_vars(&self, vars: LocalTable<'ty, 'object>) -> Locals<'ty, 'object> {
        Self::new_(LocalScope { parent: self.sc.parent.clone(), vars: vars, autopin: self.sc.autopin })
    }

    fn get_var_direct(&self, name: &str) -> LocalEntry<'ty, 'object> {
        self.sc.vars.get(name).unwrap_or(LocalEntry::Unbound)
    }

    fn insert_var(&self, name: String, entry: LocalEntry<'ty, 'object>) -> Locals<'ty, 'object> {
        self.update_vars(self.sc.vars.insert(name, entry))
    }

    fn update_upvar<F>(&self, name: &str, f: &F) -> (LocalEntry<'ty, 'object>, Option<Locals<'ty, 'object>>)
        where F: Fn(LocalEntry<'ty, 'object>) -> (LocalEntry<'ty, 'object>)
    {
        if let Some(local) = self.sc.vars.get(name) {
            let new_local = f(local);

            (new_local.clone(), Some(self.insert_var(name.to_owned(), new_local)))
        } else if let Some(ref parent) = self.sc.parent {
            let (x, parent) = parent.update_upvar(name, f);

            (x, parent.map(|parent| self.update_parent(Some(parent))))
        } else {
            (LocalEntry::Unbound, None)
        }
    }

    pub fn lookup(&self, name: &str) -> (LocalEntry<'ty, 'object>, Locals<'ty, 'object>) {
        if let Some(local) = self.sc.vars.get(name) {
            (local.clone(), self.clone())
        } else {
            let updated = self.update_upvar(name, &|local|
                match local {
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

    pub fn assign_shadow(&self, name: String, ty: TypeRef<'ty, 'object>) -> Locals<'ty, 'object> {
        self.insert_var(name, LocalEntry::Bound(ty))
    }

    pub fn assign(&self, name: String, ty: TypeRef<'ty, 'object>) -> (Option<TypeRef<'ty, 'object>>, Locals<'ty, 'object>) {
        if let Some(local) = self.sc.vars.get(&name) {
            return match local {
                LocalEntry::Bound(_) if self.sc.autopin == 0 => (None, self.insert_var(name, LocalEntry::Bound(ty))),
                LocalEntry::Bound(ty) => (Some(ty), self.insert_var(name, LocalEntry::Pinned(ty))),
                LocalEntry::Pinned(ty) => (Some(ty), self.clone()),
                LocalEntry::ConditionallyPinned(ty) => (Some(ty), self.clone()),
                LocalEntry::Unbound => panic!("should not happen"),
            }
        }

        if let Some(ref parent) = self.sc.parent {
            let (entry, locals) = parent.update_upvar(&name, &|local| {
                match local {
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

    pub fn refine(&self, name: &str, ty: TypeRef<'ty, 'object>) -> Locals<'ty, 'object> {
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

    pub fn merge(&self, other: Locals<'ty, 'object>, tyenv: &TypeEnv<'ty, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Locals<'ty, 'object> {
        assert!(self.sc.autopin == other.sc.autopin);

        let children = self.sc.vars.iter()
            .filter_map(|tbl| tbl.node.as_ref().map(|node| (node.next.clone(), tbl.clone())))
            .collect::<HashMap<_, _>>();

        let (lca, other_entries) = {
            let mut lca = None;
            let mut other_entries = Vec::new();

            for tbl in other.sc.vars.iter() {
                if let Some(ref node) = tbl.node {
                    other_entries.push((node.name.clone(), node.entry.clone()));
                }

                if children.contains_key(&tbl) {
                    lca = Some(tbl);
                    break;
                }
            }

            other_entries.reverse();
            (lca.expect("lca to be Some"), other_entries)
        };

        let self_map = self.sc.vars.bindings_since(&lca);
        let other_map = self.sc.vars.bindings_since(&lca);

        let mut names = HashSet::new();
        names.extend(self_map.keys());
        names.extend(other_map.keys());

        let vars = names.into_iter().map(|name| {
            let merge = self.get_var_direct(name).merge(other.get_var_direct(name), tyenv);

            merges.push(merge.clone());

            match merge {
                LocalEntryMerge::Ok(entry) |
                LocalEntryMerge::MustMatch(entry, ..) =>
                    (name.clone(), entry)
            }
        });

        let vars = other_entries.into_iter().chain(vars)
            .fold(self.sc.vars.clone(), |tbl, (name, entry)|
                tbl.insert(name, entry));

        self.update_vars(vars)
    }

    pub fn uncertain(&self, since: Locals<'ty, 'object>, tyenv: &TypeEnv<'ty, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Locals<'ty, 'object> {
        let mut bindings = Vec::new();

        let mut tbl = &self.sc.vars;

        loop {
            if tbl.ref_eq(&since.sc.vars) {
                break;
            }

            let binding = tbl.node.as_ref().expect("node to be Some because we have not hit 'since' yet");
            bindings.push((binding.name.clone(), binding.entry.clone()));

            tbl = &binding.next;
        }

        bindings.into_iter().rev().fold(since, |locals, (name, entry)| {
            let before_entry = locals.get_var_direct(&name);
            let merge = before_entry.merge(entry, tyenv);

            merges.push(merge.clone());

            match merge {
                LocalEntryMerge::Ok(entry) |
                LocalEntryMerge::MustMatch(entry, ..) =>
                    locals.update_vars(locals.sc.vars.insert(name, entry))
            }
        })
    }
}
