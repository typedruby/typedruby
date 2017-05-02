use std::fmt;
use std::rc::Rc;
use typecheck::types::{TypeEnv, Type};

#[derive(Debug)]
pub enum Locals<'ty, 'object: 'ty> {
    None,
    Var {
        parent: Rc<Locals<'ty, 'object>>,
        name: String,
        ty: &'ty Type<'ty, 'object>,
    },
}

impl<'ty, 'object> Locals<'ty, 'object> {
    pub fn assign(locals: Rc<Locals<'ty, 'object>>, name: String, ty: &'ty Type<'ty, 'object>) -> Rc<Locals<'ty, 'object>> {
        Rc::new(Locals::Var {
            parent: locals,
            name: name,
            ty: ty,
        })
    }

    pub fn merge(a: Rc<Locals<'ty, 'object>>, b: Rc<Locals<'ty, 'object>>) -> Rc<Locals<'ty, 'object>> {
        panic!("unimplemented")
    }
}

#[derive(Debug)]
enum Computation_<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
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
    pub fn result(ty: &'ty Type<'ty, 'object>, locals: Rc<Locals<'ty, 'object>>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Result(ty, locals)))
    }

    pub fn divergent(a: Computation<'ty, 'object>, b: Computation<'ty, 'object>) -> Computation<'ty, 'object> {
        Computation(Rc::new(Computation_::Divergent(a, b)))
    }

    pub fn seq<F>(&self, f: &F) -> Computation<'ty, 'object>
        where F: Fn(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>) -> Computation<'ty, 'object>
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
                        return Computation::result(tyenv.union(a_ty, b_ty), Locals::merge(a_l.clone(), b_l.clone()));
                    }

                    if let Computation_::Divergent(ref ba, ref bb) = *b.0 {
                        if let Computation_::Result(ba_ty, ref ba_l) = *ba.0 {
                            return Computation::divergent(
                                Computation::result(tyenv.union(a_ty, ba_ty), Locals::merge(a_l.clone(), ba_l.clone())),
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
