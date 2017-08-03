use std::fmt;
use std::rc::Rc;
use typecheck::types::{TypeEnv, Type, TypeRef};
use typecheck::locals::{Locals, LocalEntryMerge};
use ast::Loc;
use util::Or;

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

    pub fn seq_falsy<F>(self, f: F) -> ComputationPredicate<'ty, 'object>
        where F: FnOnce(Computation<'ty, 'object>) -> ComputationPredicate<'ty, 'object>
    {
        if let Some(falsy) = self.falsy {
            ComputationPredicate { truthy: self.truthy, falsy: None, non_result: self.non_result }
                .append(f(falsy))
        } else {
            self
        }
    }
}

#[derive(Debug)]
enum Computation_<'ty, 'object: 'ty> {
    Result(TypeRef<'ty, 'object>, Locals<'ty, 'object>),
    Return(TypeRef<'ty, 'object>),
    Raise(Locals<'ty, 'object>),
    Redo,
    Retry,
    Next(TypeRef<'ty, 'object>, Locals<'ty, 'object>),
    Break(TypeRef<'ty, 'object>, Locals<'ty, 'object>),
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
    pub fn result(ty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Result(ty, locals)))
    }

    pub fn return_(ty: TypeRef<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Return(ty)))
    }

    pub fn raise(locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Raise(locals)))
    }

    pub fn redo() -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Redo))
    }

    pub fn retry() -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Retry))
    }

    pub fn next(ty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Next(ty, locals)))
    }

    pub fn break_(ty: TypeRef<'ty, 'object>, locals: Locals<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Break(ty, locals)))
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
        where F: Fn(TypeRef<'ty, 'object>, Locals<'ty, 'object>) -> Computation<'ty, 'object>
    {
        match *self.0 {
            Computation_::Result(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation_::Return(_) |
            Computation_::Raise(_) |
            Computation_::Redo |
            Computation_::Retry |
            Computation_::Next(..) |
            Computation_::Break(..) => self.clone(),
            Computation_::Divergent(ref a, ref b) => Self::divergent(a.seq(f), b.seq(f)),
        }
    }

    pub fn map_locals<F>(&self, f: &F) -> Computation<'ty, 'object>
        where F: Fn(Locals<'ty, 'object>) -> Locals<'ty, 'object>
    {
        match *self.0 {
            Computation_::Result(ty, ref locals) => Self::result(ty, f(locals.clone())),
            Computation_::Raise(ref locals) => Self::raise(f(locals.clone())),
            Computation_::Next(ty, ref locals) => Self::next(ty, f(locals.clone())),
            Computation_::Break(ty, ref locals) => Self::break_(ty, f(locals.clone())),
            Computation_::Return(_) |
            Computation_::Redo |
            Computation_::Retry => self.clone(),
            Computation_::Divergent(ref a, ref b) => Self::divergent(a.map_locals(f), b.map_locals(f)),
        }
    }

    pub fn terminate<F>(&self, f: &F)
        where F: Fn(TypeRef<'ty, 'object>)
    {
        match *self.0 {
            Computation_::Result(ref ty, _) |
            Computation_::Return(ref ty) => f(ty.clone()),
            Computation_::Raise(_) => {},
            Computation_::Redo |
            Computation_::Retry |
            Computation_::Next(..) |
            Computation_::Break(..) => {
                // illegal redo/retry/next/break!
                // TODO - we should error on this in eval
            },
            Computation_::Divergent(ref a, ref b) => {
                a.terminate(f);
                b.terminate(f);
            },
        }
    }

    pub fn terminate_next_scope(&self) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Next(ty, ref locals) =>
                Computation::result(ty, locals.clone()),
            Computation_::Divergent(ref a, ref b) =>
                Computation::divergent(a.terminate_next_scope(), b.terminate_next_scope()),
            _ =>
                self.clone()
        }
    }

    pub fn terminate_break_scope(&self) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Break(ty, ref locals) =>
                Computation::result(ty, locals.clone()),
            Computation_::Divergent(ref a, ref b) =>
                Computation::divergent(a.terminate_break_scope(), b.terminate_break_scope()),
            _ =>
                self.clone()
        }
    }

    pub fn converge_results(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Result(..) |
            Computation_::Return(..) |
            Computation_::Raise(..) |
            Computation_::Redo |
            Computation_::Retry |
            Computation_::Next(..) |
            Computation_::Break(..) => self.clone(),

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

    pub fn extract_results(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'object>, merges: &mut Vec<LocalEntryMerge<'ty, 'object>>)
        -> EvalResult<'ty, 'object, TypeRef<'ty, 'object>>
    {
        let converged = self.converge_results(loc, tyenv, merges);

        match *converged.0 {
            Computation_::Result(ty, ref locals) => EvalResult::Ok(ty, locals.clone()),

            Computation_::Raise(..) |
            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry |
            Computation_::Next(..) |
            Computation_::Break(..) => EvalResult::NonResult(converged.clone()),

            Computation_::Divergent(ref a, ref b) => {
                // if there were any result computations, converge_results
                // guarantees that they will have been collapsed into the
                // left hand side of the divergent computation it returns.
                if let Computation_::Result(ty, ref locals) = *a.0 {
                    EvalResult::Both(ty, locals.clone(), b.clone())
                } else {
                    EvalResult::NonResult(converged.clone())
                }
            }
        }
    }

    pub fn predicate(&self, loc: &Loc, tyenv: &TypeEnv<'ty, 'object>) -> ComputationPredicate<'ty, 'object> {
        fn refine_computation<'ty, 'object: 'ty>(ty: TypeRef<'ty, 'object>, refined_ty: TypeRef<'ty, 'object>, locals: &Locals<'ty, 'object>) -> Computation<'ty, 'object> {
            let locals = if let Type::LocalVariable { ref name, .. } = *ty {
                locals.refine(name, refined_ty)
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
            Computation_::Raise(..) |
            Computation_::Return(..) |
            Computation_::Redo |
            Computation_::Retry |
            Computation_::Next(..) |
            Computation_::Break(..) => {
                ComputationPredicate::non_result(self.clone())
            }
        }
    }
}

pub enum EvalResult<'ty, 'object: 'ty, T> {
    Ok(T, Locals<'ty, 'object>),
    Both(T, Locals<'ty, 'object>, Computation<'ty, 'object>),
    NonResult(Computation<'ty, 'object>)
}

impl<'ty, 'object, T> EvalResult<'ty, 'object, T> {
    pub fn map<F, U>(self, f: F) -> EvalResult<'ty, 'object, U>
        where F : FnOnce(T) -> U
    {
        match self {
            EvalResult::Ok(val, locals) => EvalResult::Ok(f(val), locals),
            EvalResult::Both(val, locals, non_result) => EvalResult::Both(f(val), locals, non_result),
            EvalResult::NonResult(non_result) => EvalResult::NonResult(non_result),
        }
    }

    pub fn and_then<F, U>(self, f: F) -> EvalResult<'ty, 'object, U>
        where F : FnOnce(T, Locals<'ty, 'object>) -> EvalResult<'ty, 'object, U>
    {
        match self {
            EvalResult::Ok(val, locals) => f(val, locals),
            EvalResult::Both(val, locals, non_result) => {
                match f(val, locals) {
                    EvalResult::Ok(val, locals) =>
                        EvalResult::Both(val, locals, non_result),
                    EvalResult::Both(val, locals, other_non_result) =>
                        EvalResult::Both(val, locals,
                            Computation::divergent(non_result, other_non_result)),
                    EvalResult::NonResult(other_non_result) =>
                        EvalResult::NonResult(
                            Computation::divergent(non_result, other_non_result)),
                }
            }
            EvalResult::NonResult(non_result) => EvalResult::NonResult(non_result),
        }
    }

    pub fn and_then_comp<F>(self, f: F) -> Computation<'ty, 'object>
        where F : FnOnce(T, Locals<'ty, 'object>) -> Computation<'ty, 'object>
    {
        match self {
            EvalResult::Ok(val, locals) => f(val, locals),
            EvalResult::Both(val, locals, non_result) =>
                Computation::divergent(non_result, f(val, locals)),
            EvalResult::NonResult(non_result) => non_result,
        }
    }

    pub fn if_not<F>(self, mut f: F) -> EvalResult<'ty, 'object, T>
        where F : FnMut()
    {
        if let EvalResult::NonResult(_) = self {
            f();
        }

        self
    }
}

impl<'ty, 'object> EvalResult<'ty, 'object, TypeRef<'ty, 'object>> {
    pub fn into_computation(self) -> Computation<'ty, 'object> {
        match self {
            EvalResult::Ok(ty, locals) => Computation::result(ty, locals),
            EvalResult::Both(ty, locals, comp) => Computation::divergent(Computation::result(ty, locals), comp),
            EvalResult::NonResult(comp) => comp,
        }
    }
}
