use ast::{Loc};
use typecheck::types::{Arg, Type, TypeEnv};
use std::marker::Sized;
use std::ops::Deref;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum CallArg<'ty, 'object: 'ty> {
    Pass(Loc, &'ty Type<'ty, 'object>),
    Splat(Loc, &'ty Type<'ty, 'object>),
}

impl<'ty, 'object> CallArg<'ty, 'object> {
    pub fn loc(&self) -> &Loc {
        match *self {
            CallArg::Pass(ref loc, _) => loc,
            CallArg::Splat(ref loc, _) => loc,
        }
    }
}

#[derive(Debug)]
pub enum ArgError {
    TooFewArguments,
    TooManyArguments(Loc),
    MissingKeyword(String),
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
    prototype_arg_type: &'ty Type<'ty, 'object>,
    passed_arg_type: &'ty Type<'ty, 'object>,
    result: &mut MatchResult<'ty, 'object>)
{
    result.matches.push((prototype_arg_type, passed_arg_type));
}

fn consume_remaining_keywords<'a, 'ty: 'a, 'object: 'ty>(
    prototype_args: &mut View<'a, Arg<'ty, 'object>>,
    result: &mut MatchResult<'ty, 'object>
) {
    loop {
        match prototype_args.last() {
            Some(&Arg::Kwarg { ref name, .. }) => {
                prototype_args.consume_back();
                result.errors.push(ArgError::MissingKeyword(name.clone()));
            }
            Some(&Arg::Kwoptarg { .. }) => {
                prototype_args.consume_back();
            }
            _ => break
        }
    }
}

fn match_keyword_hash_argument<'a, 'ty: 'a, 'env, 'object: 'ty + 'env>(
    tyenv: &TypeEnv<'ty, 'env, 'object>,
    prototype_args: &mut View<'a, Arg<'ty, 'object>>,
    args: &mut View<'a, CallArg<'ty, 'object>>,
    result: &mut MatchResult<'ty, 'object>
) {
    let kw_loc = match prototype_args.last() {
        Some(&Arg::Kwarg { ref loc, .. }) |
        Some(&Arg::Kwoptarg { ref loc, .. }) => loc,
        _ => return,
    };

    if let Some(&CallArg::Pass(_, ty)) = args.last() {
        match *tyenv.prune(ty) {
            Type::KeywordHash { ref keywords, .. } => {
                args.consume_back();

                let mut keywords = keywords.iter().cloned().collect::<HashMap<_,_>>();

                loop {
                    match prototype_args.last() {
                        Some(&Arg::Kwarg { ref name, ty: proto_ty, .. }) => {
                            prototype_args.consume_back();

                            match keywords.remove(name) {
                                Some(passed_ty) => match_argument(proto_ty, passed_ty, result),
                                None => { result.errors.push(ArgError::MissingKeyword(name.clone())) }
                            }
                        }
                        Some(&Arg::Kwoptarg { ref name, ty: proto_ty, .. }) => {
                            prototype_args.consume_back();

                            match keywords.remove(name) {
                                Some(passed_ty) => match_argument(proto_ty, passed_ty, result),
                                None => { /* pass */ }
                            }
                        }
                        _ => break
                    }
                }
            }
            Type::Instance { ref class, .. } => {
                let hash_class = tyenv.object.get_const(tyenv.object.Object, "Hash").expect("expected Hash to be defined");

                if class.is_a(hash_class) {
                    args.consume_back();

                    let mut potential_keywords = Vec::new();
                    let mut keyword_hash_loc = kw_loc.clone();

                    loop {
                        match prototype_args.last() {
                            Some(&Arg::Kwarg { ref loc, ref name, ty: proto_ty }) |
                            Some(&Arg::Kwoptarg { ref loc, ref name, ty: proto_ty, .. }) => {
                                prototype_args.consume_back();

                                keyword_hash_loc = keyword_hash_loc.join(loc);

                                potential_keywords.push((name.clone(), proto_ty));
                            }
                            _ => break
                        }
                    }

                    let proto_hash_ty = tyenv.keyword_hash(keyword_hash_loc, potential_keywords);

                    result.matches.push((proto_hash_ty, ty));
                } else {
                    consume_remaining_keywords(prototype_args, result);
                }
            }
            _ => consume_remaining_keywords(prototype_args, result),
        }
    } else {
        consume_remaining_keywords(prototype_args, result);
    }
}

fn match_prototype_argument<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_arg_type: &'ty Type<'ty, 'object>,
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    result: &mut MatchResult<'ty, 'object>
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
        None => {}
    }
}

fn match_required_arguments<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    result: &mut MatchResult<'ty, 'object>
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
    result: &mut MatchResult<'ty, 'object>
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
    result: &mut MatchResult<'ty, 'object>
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
                }
                Some(&CallArg::Splat(_, splat_ty)) => {
                    args.consume();
                    match_argument(proto_ty, splat_ty, result);
                }
                None => break
            }
        }
    }
}

pub fn match_prototype_with_invocation<'ty, 'env, 'object: 'ty + 'env>(
    tyenv: &TypeEnv<'ty, 'env, 'object>,
    prototype_args: &[Arg<'ty, 'object>],
    call_args: &[CallArg<'ty, 'object>],
) -> MatchResult<'ty, 'object>
{
    let mut result = MatchResult {
        matches: Vec::new(),
        errors: Vec::new(),
    };

    let mut prototype_args = View(prototype_args);
    let mut call_args = View(call_args);

    let required_argc = prototype_args.iter().filter(|arg|
        match **arg {
            Arg::Required { .. } => true,
            _ => false,
        }
    ).count();

    if call_args.len() > required_argc {
        match_keyword_hash_argument(tyenv,
            &mut prototype_args,
            &mut call_args,
            &mut result);
    }

    match_required_arguments(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut call_args),
        &mut result);

    match_required_arguments(
        &mut ReverseConsumer(&mut prototype_args),
        &mut ReverseConsumer(&mut call_args),
        &mut result);

    match_optional_arguments(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut call_args),
        &mut result);

    match_rest_argument(
        &mut ForwardConsumer(&mut prototype_args),
        &mut ForwardConsumer(&mut call_args),
        &mut result);

    if !prototype_args.is_empty() {
        result.errors.push(ArgError::TooFewArguments);
    }

    if let Some(arg) = call_args.first() {
        result.errors.push(ArgError::TooManyArguments(arg.loc().clone()));
    }

    result
}
