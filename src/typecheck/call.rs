use ast::{Loc};
use typecheck::types::{Arg, Type, TypeEnv, TypeRef, SplatArg};
use slice_util::{View, Consumer, ForwardConsumer, ReverseConsumer};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgError {
    TooFewArguments,
    TooManyArguments(Loc),
    MissingKeyword(String),
    UnknownKeyword(String),
    UnexpectedSplat(Loc),
}

#[derive(Debug)]
pub struct MatchResult<'ty, 'object: 'ty> {
    pub matches: Vec<(TypeRef<'ty, 'object>, TypeRef<'ty, 'object>)>,
    pub errors: Vec<ArgError>,
}

fn match_argument<'ty, 'object: 'ty>(
    prototype_arg_type: TypeRef<'ty, 'object>,
    passed_arg_type: TypeRef<'ty, 'object>,
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
            Some(&Arg::Kwoptarg { .. }) |
            Some(&Arg::Kwrest { .. }) => {
                prototype_args.consume_back();
            }
            _ => break
        }
    }
}

enum KeywordHashArgument<'a, 'ty: 'a, 'object: 'ty> {
    Keywords(&'a [(String, TypeRef<'ty, 'object>)], Option<TypeRef<'ty, 'object>>),
    Hash(TypeRef<'ty, 'object>),
    None
}

fn keyword_hash_argument<'a, 'ty: 'a, 'object: 'ty>(
    tyenv: &TypeEnv<'ty, 'object>,
    prototype_args: &mut View<'a, Arg<'ty, 'object>>,
    args: &mut View<'a, SplatArg<'ty, 'object>>,
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

    if let Some(&SplatArg::Value(ty)) = args.last() {
        match *tyenv.prune(ty).deref() {
            Type::KeywordHash { ref keywords, splat, .. } => {
                args.consume_back();
                KeywordHashArgument::Keywords(keywords, splat)
            }
            Type::Instance { ref class, .. } => {
                if tyenv.is_hash(class) {
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

fn match_keyword_hash_argument<'a, 'ty: 'a, 'object: 'ty>(
    tyenv: &TypeEnv<'ty, 'object>,
    prototype_args: &mut View<'a, Arg<'ty, 'object>>,
    args: &mut View<'a, SplatArg<'ty, 'object>>,
    result: &mut MatchResult<'ty, 'object>
) {
    let kw_loc = match prototype_args.last() {
        Some(&Arg::Kwarg { ref loc, .. }) |
        Some(&Arg::Kwoptarg { ref loc, .. }) => loc,
        Some(&Arg::Kwrest { ref loc, .. }) => loc,
        _ => return,
    };

    match keyword_hash_argument(tyenv, prototype_args, args) {
        KeywordHashArgument::Keywords(keywords, splat) => {
            let mut keywords = keywords.iter().cloned().collect::<HashMap<_,_>>();
            let mut kwrest_ty = None;

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
                    Some(&Arg::Kwrest { ty: proto_ty, .. }) => {
                        prototype_args.consume_back();
                        kwrest_ty = Some(proto_ty);
                    }
                    _ => break
                }
            }

            if let Some(kwrest_ty) = kwrest_ty {
                for (_, passed_ty) in keywords {
                    match_argument(kwrest_ty, passed_ty, result);
                }

                if let Some(splat_ty) = splat {
                    match_argument(kwrest_ty, splat_ty, result);
                }
            } else {
                let unknown_keywords = keywords.iter().map(|(name,_)| ArgError::UnknownKeyword(name.clone()));
                result.errors.extend(unknown_keywords);

                if let Some(splat_ty) = splat {
                    result.errors.push(ArgError::UnexpectedSplat(splat_ty.loc().clone()));
                }
            }
        }
        KeywordHashArgument::Hash(hash_ty) => {
            let mut potential_keywords = Vec::new();
            let mut keyword_hash_loc = kw_loc.clone();
            let mut kwrest_ty = None;

            loop {
                match prototype_args.last() {
                    Some(&Arg::Kwarg { ref loc, ref name, ty: proto_ty }) |
                    Some(&Arg::Kwoptarg { ref loc, ref name, ty: proto_ty, .. }) => {
                        prototype_args.consume_back();

                        keyword_hash_loc = keyword_hash_loc.join(loc);

                        potential_keywords.push((name.clone(), proto_ty));
                    }
                    Some(&Arg::Kwrest { ty: proto_ty, .. }) => {
                        prototype_args.consume_back();
                        kwrest_ty = Some(proto_ty);
                    }
                    _ => break
                }
            }

            let proto_hash_ty = tyenv.keyword_hash(keyword_hash_loc, potential_keywords, kwrest_ty);

            result.matches.push((proto_hash_ty, hash_ty));
        }
        KeywordHashArgument::None => consume_remaining_keywords(prototype_args, result),
    }
}

fn match_prototype_argument<'a, 'ty: 'a, 'object: 'ty, PrototypeConsumer, PassedConsumer>(
    prototype_arg_type: TypeRef<'ty, 'object>,
    prototype_args: &mut PrototypeConsumer,
    args: &mut PassedConsumer,
    result: &mut MatchResult<'ty, 'object>
) where PrototypeConsumer : Consumer<'a, Arg<'ty, 'object>>,
        PassedConsumer : Consumer<'a, SplatArg<'ty, 'object>>
{
    match args.peek() {
        Some(&SplatArg::Value(pass_ty)) => {
            prototype_args.consume();
            args.consume();

            match_argument(prototype_arg_type, pass_ty, result);
        }
        Some(&SplatArg::Splat(pass_ty)) => {
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
        PassedConsumer : Consumer<'a, SplatArg<'ty, 'object>>
{
    while let Some(..) = args.peek() {
        let proto_arg = prototype_args.peek().map(|a| a.unwrap_procarg0());

        if let Some(&Arg::Required { ty: proto_ty, .. }) = proto_arg {
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
        PassedConsumer : Consumer<'a, SplatArg<'ty, 'object>>
{
    while let Some(..) = args.peek() {
        let proto_arg = prototype_args.peek().map(|a| a.unwrap_procarg0());

        if let Some(&Arg::Optional { ty: proto_ty, .. }) = proto_arg {
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
        PassedConsumer : Consumer<'a, SplatArg<'ty, 'object>>
{
    if let Some(&Arg::Rest { ty: proto_ty, .. }) = prototype_args.peek() {
        prototype_args.consume();

        loop {
            match args.peek() {
                Some(&SplatArg::Value(pass_ty)) => {
                    args.consume();
                    match_argument(proto_ty, pass_ty, result);
                }
                Some(&SplatArg::Splat(splat_ty)) => {
                    args.consume();
                    match_argument(proto_ty, splat_ty, result);
                }
                None => break
            }
        }
    }
}

pub fn match_prototype_with_invocation<'ty, 'object: 'ty>(
    tyenv: &TypeEnv<'ty, 'object>,
    prototype_args: &[Arg<'ty, 'object>],
    call_args: &[SplatArg<'ty, 'object>],
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
