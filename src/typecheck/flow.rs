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
pub enum Computation<'ty, 'object: 'ty> {
    Result(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
    Return(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>),
    Divergent(Rc<Computation<'ty, 'object>>, Rc<Computation<'ty, 'object>>),
}

impl<'ty, 'object> Computation<'ty, 'object> {
    pub fn new_result(ty: &'ty Type<'ty, 'object>, locals: Rc<Locals<'ty, 'object>>) -> Rc<Computation<'ty, 'object>> {
        Rc::new(Computation::Result(ty, locals))
    }

    pub fn seq<F>(comp: Rc<Computation<'ty, 'object>>, f: &F) -> Rc<Computation<'ty, 'object>>
        where F: Fn(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>) -> Rc<Computation<'ty, 'object>>
    {
        match *comp {
            Computation::Result(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation::Return(ref ty, ref locals) => { f(ty.clone(), locals.clone()); comp.clone() },
            Computation::Divergent(ref a, ref b) => {
                Rc::new(Computation::Divergent(
                    Computation::seq(a.clone(), f),
                    Computation::seq(b.clone(), f)))
            },
        }
    }

    pub fn term<F>(comp: Rc<Computation<'ty, 'object>>, f: &F)
        where F: Fn(&'ty Type<'ty, 'object>, Rc<Locals<'ty, 'object>>)
    {
        match *comp {
            Computation::Result(ref ty, ref locals) |
            Computation::Return(ref ty, ref locals) => f(ty.clone(), locals.clone()),
            Computation::Divergent(ref a, ref b) => {
                Computation::term(a.clone(), f);
                Computation::term(b.clone(), f);
            },
        }
    }

    pub fn converge(comp: Rc<Computation<'ty, 'object>>) -> Rc<Computation<'ty, 'object>> {
        match *comp {
            Computation::Result(..) => comp.clone(),
            Computation::Return(..) => comp.clone(),
            Computation::Divergent(ref a, ref b) => panic!("unimplemented"),
        }
    }
}
