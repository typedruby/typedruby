use errors::{Detail, ErrorSink};
use ast::Loc;

#[derive(Hash,Eq,PartialEq)]
pub enum Error {
    NoTypeParameters { loc: Loc },
    TooFewTypeParameters { msg: String, loc: Loc },
    TooManyTypeParameters { loc: Loc },
}

impl Error {
    pub fn emit(&self, sink: &mut ErrorSink) {
        match *self {
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
        }
    }
}
