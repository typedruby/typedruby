use std::rc::Rc;
use ast::{Node, Loc};
use typecheck::types::{Arg, Type, Prototype};
use std::marker::Sized;
use std::ops::Deref;

#[derive(Debug,Clone)]
pub enum CallArg<'ty, 'object: 'ty> {
    Pass(Loc, &'ty Type<'ty, 'object>),
    Splat(Loc, &'ty Type<'ty, 'object>),
    Kwsplat(Loc, &'ty Type<'ty, 'object>),
    BlockPass(Loc, &'ty Type<'ty, 'object>),
    BlockLiteral(Loc, Rc<Node>, Option<Rc<Node>>),
}

impl<'ty, 'object> CallArg<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            CallArg::Pass(ref loc, _) => loc,
            CallArg::Splat(ref loc, _) => loc,
            CallArg::Kwsplat(ref loc, _) => loc,
            CallArg::BlockPass(ref loc, _) => loc,
            CallArg::BlockLiteral(ref loc, _, _) => loc,
        }
    }
}

#[derive(Debug)]
pub enum ArgError {
    TooFewArguments,
    TooManyArguments(Loc),
}

#[derive(Debug)]
pub struct MatchResult<'ty, 'object: 'ty> {
    pub matches: Vec<(&'ty Type<'ty, 'object>, &'ty Type<'ty, 'object>)>,
    pub errors: Vec<ArgError>,
}

struct View<'a, T: 'a + Sized>(&'a [T]);

impl<'a, T> View<'a, T> {
    // need to implement first and last manually rather than relying on Deref
    // because &T actually has lifetime 'a, not lifetime of self:
    pub fn first(&self) -> Option<&'a T> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&'a T> {
        self.0.last()
    }

    pub fn consume_front(&mut self) {
        self.0 = &self.0[1..self.0.len()]
    }

    pub fn consume_back(&mut self) {
        self.0 = &self.0[0..(self.0.len() - 1)]
    }
}

impl<'a, T> Deref for View<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

trait Consumer<'a, T: 'a> {
    fn peek(&self) -> Option<&'a T>;
    fn consume(&mut self);
}

struct ForwardConsumer<'v, 'a: 'v, T: 'a>(&'v mut View<'a, T>);

impl<'v, 'a, T> Consumer<'a, T> for ForwardConsumer<'v, 'a, T> {
    fn peek(&self) -> Option<&'a T> {
        self.0.first()
    }

    fn consume(&mut self) {
        self.0.consume_front();
    }
}

struct ReverseConsumer<'v, 'a: 'v, T: 'a>(&'v mut View<'a, T>);

impl<'v, 'a, T> Consumer<'a, T> for ReverseConsumer<'v, 'a, T> {
    fn peek(&self) -> Option<&'a T> {
        self.0.last()
    }

    fn consume(&mut self) {
        self.0.consume_back()
    }
}

fn match_argument<'ty, 'object: 'ty>(
    prototype_arg_type: Option<&'ty Type<'ty, 'object>>,
    passed_arg_type: &'ty Type<'ty, 'object>,
    result: &mut MatchResult<'ty, 'object>)
{
    if let Some(proto_ty) = prototype_arg_type {
        result.matches.push((proto_ty, passed_arg_type));
    }
}

fn match_prototype_argument<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_arg_type: Option<&'ty Type<'ty, 'object>>,
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    mut result: &mut MatchResult<'ty, 'object>
) where PrototypeConsumer : Consumer<'a, Arg<'ty, 'object>>,
        PassedConsumer : Consumer<'a, CallArg<'ty, 'object>>
{
    match args.peek() {
        Some(&CallArg::Pass(_, pass_ty)) => {
            prototype_args.consume();
            args.consume();

            match_argument(prototype_arg_type, pass_ty, result);
        }
        Some(&CallArg::Splat(_, pass_ty)) => {
            // consume the prototype arg but *not* this splat arg -
            // since we don't know ahead of time how many arguments this
            // splat will produce we need to match it against the remaining
            // required arguments
            prototype_args.consume();

            match_argument(prototype_arg_type, pass_ty, result);
        }
        Some(&CallArg::Kwsplat(..)) |
        Some(&CallArg::BlockPass(..)) |
        Some(&CallArg::BlockLiteral(..)) => panic!("unimplemented"),
        None => {},
    }
}

fn match_required_arguments<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    mut result: &mut MatchResult<'ty, 'object>
) where PrototypeConsumer : Consumer<'a, Arg<'ty, 'object>>,
        PassedConsumer : Consumer<'a, CallArg<'ty, 'object>>
{
    while let Some(..) = args.peek() {
        if let Some(&Arg::Required { ty: proto_ty, .. }) = prototype_args.peek() {
            match_prototype_argument(proto_ty, prototype_args, args, result)
        } else {
            break
        }
    }
}

fn match_optional_arguments<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    mut result: &mut MatchResult<'ty, 'object>
) where PrototypeConsumer : Consumer<'a, Arg<'ty, 'object>>,
        PassedConsumer : Consumer<'a, CallArg<'ty, 'object>>
{
    while let Some(..) = args.peek() {
        if let Some(&Arg::Optional { ty: proto_ty, .. }) = prototype_args.peek() {
            match_prototype_argument(proto_ty, prototype_args, args, result)
        } else {
            break
        }
    }
}

fn match_rest_argument<'a, 'ty: 'a, 'object, PrototypeConsumer, PassedConsumer>(
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    mut result: &mut MatchResult<'ty, 'object>
) where PrototypeConsumer : Consumer<'a, Arg<'ty, 'object>>,
        PassedConsumer : Consumer<'a, CallArg<'ty, 'object>>
{
    if let Some(&Arg::Rest { ty: proto_ty, .. }) = prototype_args.peek() {
        prototype_args.consume();

        loop {
            match args.peek() {
                Some(&CallArg::Pass(_, pass_ty)) => {
                    args.consume();
                    match_argument(proto_ty, pass_ty, result);
                },
                Some(&CallArg::Splat(_, splat_ty)) => {
                    args.consume();
                    match_argument(proto_ty, splat_ty, result);
                },
                Some(&CallArg::Kwsplat(..)) |
                Some(&CallArg::BlockPass(..)) |
                Some(&CallArg::BlockLiteral(..)) => panic!("unimplemented"),
                None => break,
            }
        }
    }
}

pub fn match_prototype_with_invocation<'ty, 'object: 'ty>(
    prototype: &Prototype<'ty, 'object>,
    args: &[CallArg<'ty, 'object>],
) -> MatchResult<'ty, 'object>
{
    let mut result = MatchResult {
        matches: Vec::new(),
        errors: Vec::new(),
    };

    let mut args = View(args);
    let mut prototype_args = View(prototype.args.as_slice());

    let required_argc = prototype_args.iter().filter(|arg|
        match **arg {
            Arg::Required { .. } => true,
            _ => false,
        }
    ).count();

    if args.len() < required_argc {
        result.errors.push(ArgError::TooFewArguments);
    }

    if args.len() > required_argc {
        // handle popping keyword args off the end
    }

    match_required_arguments(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut args),
        &mut result);

    match_required_arguments(
        &mut ReverseConsumer(&mut prototype_args),
        &mut ReverseConsumer(&mut args),
        &mut result);

    match_optional_arguments(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut args),
        &mut result);

    match_rest_argument(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut args),
        &mut result);

    assert!(prototype_args.is_empty());

    if let Some(arg) = args.first() {
        result.errors.push(ArgError::TooManyArguments(arg.loc().clone()));
    }

    result
}
