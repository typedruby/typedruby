use errors::{Detail, ErrorSink};
use ast::Loc;

#[derive(Hash,Eq,PartialEq)]
pub enum Error {
    Arbitrary { msg: String, loc: Loc },
    NoTypeParameters { loc: Loc },
    TooFewTypeParameters { msg: String, loc: Loc },
    TooManyTypeParameters { loc: Loc },
    TooManyMetaclassParameters { loc: Loc },
    MetaclassParameterMustBeCpath { loc: Loc },
    ConstantNotClassOrModule { loc: Loc },
    TypeParametersInNonGenericType { loc: Loc },
    CannotInstantiateInstanceType { self_: String, loc: Loc },
}

impl Error {
    pub fn emit(&self, sink: &mut ErrorSink) {
        match *self {
            Error::Arbitrary { ref msg, ref loc } =>
                sink.error(msg, &[
                    Detail::Loc("here", loc),
                ]),

            Error::NoTypeParameters { ref loc } =>
                sink.error("Type referenced is generic but no type parameters were supplied", &[
                    Detail::Loc("here", loc),
                ]),

            Error::TooFewTypeParameters { ref msg, ref loc } =>
                sink.error("Too few type parameters supplied in instantiation of generic type", &[
                    Detail::Loc(msg, loc),
                ]),

            Error::TooManyTypeParameters { ref loc } =>
                sink.error("Too many type parameters supplied in instantiation of generic type", &[
                    Detail::Loc("from here", loc),
                ]),

            Error::TooManyMetaclassParameters { ref loc } =>
                sink.error("Too many type parameters supplied in instantiation of metaclass", &[
                    Detail::Loc("from here", loc),
                ]),

            Error::MetaclassParameterMustBeCpath { ref loc } =>
                sink.error("Type parameter in metaclass must be constant path", &[
                    Detail::Loc("here", loc),
                ]),

            Error::ConstantNotClassOrModule { ref loc } =>
                sink.error("Constant does not reference class/module", &[
                    Detail::Loc("here", loc),
                ]),

            Error::TypeParametersInNonGenericType { ref loc } =>
                sink.error("Type parameters were supplied but type mentioned does not take any", &[
                    Detail::Loc("here", loc),
                ]),

            Error::CannotInstantiateInstanceType { ref self_, ref loc } =>
                sink.error("Cannot instatiate instance type", &[
                    Detail::Loc(&format!("self here is {}, which is not a Class", self_), loc),
                ]),
        }
    }
}
