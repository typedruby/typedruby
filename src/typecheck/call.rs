use ast::{Loc};
use typecheck::types::{Arg, Type, TypeEnv};
use slice_util::{View, Consumer, ForwardConsumer, ReverseConsumer};
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

enum KeywordHashArgument<'a, 'ty: 'a, 'object: 'ty> {
    Keywords(&'a [(String, &'ty Type<'ty, 'object>)]),
    Hash(&'ty Type<'ty, 'object>),
    None
}

fn keyword_hash_argument<'a, 'ty: 'a, 'env, 'object: 'ty + 'env>(
    tyenv: &TypeEnv<'ty, 'env, 'object>,
    prototype_args: &mut View<'a, Arg<'ty, 'object>>,
    args: &mut View<'a, CallArg<'ty, 'object>>,
) -> KeywordHashArgument<'a, 'ty, 'object>
{
    let required_argc = prototype_args.iter().filter(|arg|
        match **arg {
            Arg::Required { .. } => true,
            _ => false,
        }
    ).count();

    if args.len() <= required_argc {
        return KeywordHashArgument::None;
    }

    if let Some(&CallArg::Pass(_, ty)) = args.last() {
        match *tyenv.prune(ty) {
            Type::KeywordHash { ref keywords, .. } => {
                args.consume_back();
                KeywordHashArgument::Keywords(keywords)
            }
            Type::Instance { ref class, .. } => {
                if class.is_a(tyenv.object.hash_class()) {
                    args.consume_back();
                    KeywordHashArgument::Hash(ty)
                } else {
                    KeywordHashArgument::None
                }
            }
            _ => KeywordHashArgument::None
        }
    } else {
        KeywordHashArgument::None
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

    match keyword_hash_argument(tyenv, prototype_args, args) {
        KeywordHashArgument::Keywords(ref keywords) => {
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
        KeywordHashArgument::Hash(ref hash_ty) => {
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

            result.matches.push((proto_hash_ty, hash_ty));
        }
        KeywordHashArgument::None => consume_remaining_keywords(prototype_args, result),
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

    // consume remaining optional arguments:
    while let Some(&Arg::Optional { .. }) = prototype_args.peek() {
        prototype_args.consume();
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

    match_keyword_hash_argument(tyenv,
        &mut prototype_args,
        &mut call_args,
        &mut result);

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
