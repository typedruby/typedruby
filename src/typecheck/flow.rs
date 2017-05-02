use std::fmt;
use std::rc::Rc;
use typecheck::types::Type;

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
}

#[derive(Debug)]
enum Computation_<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
    Return(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
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
            Computation_::Return(ref ty, ref locals) => { f(ty.clone(), locals.clone()); self.clone() },
            Computation_::Divergent(ref a, ref b) => Self::divergent(a.seq(f), b.seq(f)),
        }
    }

    pub fn term<F>(&self, f: &F)
        where F: Fn(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>)
    {
        match *self.0 {
            Computation_::Result(ref ty, ref locals) |
            Computation_::Return(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation_::Divergent(ref a, ref b) => {
                a.term(f);
                b.term(f);
            },
        }
    }

    pub fn converge(&self) -> Computation<'ty, 'object> {
        match *self.0 {
            Computation_::Result(..) => self.clone(),
            Computation_::Return(..) => self.clone(),
            Computation_::Divergent(ref a, ref b) => panic!("unimplemented"),
        }
    }
}
