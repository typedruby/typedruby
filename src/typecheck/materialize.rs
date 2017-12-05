use std::rc::Rc;
use itertools::Itertools;

use abstract_type;
use abstract_type::TypeNode;
use ast::Id;
use typecheck::types::{TypeEnv, TypeContext, TypeRef, Type, Prototype, Arg, TypeConstraint};
use typecheck::locals::Locals;
use object::RubyObject;
use errors::Detail;
use environment::Environment;

pub struct Materialize<'a, 'ty: 'a, 'object: 'ty> {
    env: &'a Environment<'object>,
    tyenv: &'a TypeEnv<'ty, 'object>,
}

impl<'a, 'ty, 'object> Materialize<'a, 'ty, 'object> {
    pub fn new(env: &'a Environment<'object>, tyenv: &'a TypeEnv<'ty, 'object>) -> Self {
        Materialize { env, tyenv }
    }

    pub fn materialize_type(&self, type_node: &TypeNode<'object>, context: &TypeContext<'ty, 'object>) -> TypeRef<'ty, 'object> {
        match *type_node {
            TypeNode::Instance { ref loc, class, ref type_parameters } =>
                self.tyenv.instance(loc.clone(), class,
                    type_parameters.iter().map(|node|
                        self.materialize_type(node, context)).collect()),
            TypeNode::Tuple { ref loc, ref lead, ref splat, ref post } =>
                self.tyenv.tuple(loc.clone(),
                    lead.iter().map(|node| self.materialize_type(node, context)).collect(),
                    splat.as_ref().map(|node| self.materialize_type(node, context)),
                    post.iter().map(|node| self.materialize_type(node, context)).collect()),
            TypeNode::Union { ref loc, ref types } =>
                types.iter()
                    .map(|node| self.materialize_type(node, context))
                    .fold1(|a, b| self.tyenv.union(loc, a, b))
                    .unwrap(),
            TypeNode::Any { ref loc } =>
                self.tyenv.any(loc.clone()),
            TypeNode::TypeParameter { ref loc, ref name } =>
                self.tyenv.update_loc(context.type_names[name], loc.clone()),
            TypeNode::Proc { ref loc, ref proto } => {
                let mut context = context.clone();

                let (proto, _) = self.materialize_prototype(proto, Locals::new(), &mut context);

                self.tyenv.alloc(Type::Proc { loc: loc.clone(), proto: proto })
            }
            TypeNode::SpecialSelf { ref loc } =>
                context.self_type(self.tyenv, loc.clone()),
            TypeNode::SpecialInstance { ref loc } =>
                match *context.self_class(self.tyenv) {
                    RubyObject::Metaclass { of, .. } => {
                        // if the class we're trying to instantiate has type parameters just fill them with new
                        // type variables. TODO revisit this logic and see if there's something better we could do?
                        let type_parameters = of.type_parameters().iter().map(|_| self.tyenv.new_var(loc.clone())).collect();
                        self.tyenv.instance(loc.clone(), of, type_parameters)
                    },
                    ref class => {
                        // special case to allow the Class#allocate definition in the stdlib:
                        if class != self.env.object.Class {
                            // TODO: we need to move this check out to abstract_type
                            // we should not ever error while materializing a type!
                            let mut sink = self.env.error_sink.borrow_mut();
                            sink.error("Cannot instatiate instance type", &[
                                Detail::Loc(&format!("Self here is {}, which is not a Class", class.name()), loc),
                            ]);
                        }

                        self.tyenv.new_var(loc.clone())
                    },
                },
            TypeNode::SpecialClass { ref loc } => {
                let metaclass = self.env.object.metaclass(context.self_class(self.tyenv));
                // metaclasses never have type parameters:
                self.tyenv.instance(loc.clone(), metaclass, Vec::new())
            }
            TypeNode::Error { ref loc } =>
                // an error was already printed, just make a fresh type var:
                self.tyenv.new_var(loc.clone()),
        }
    }

    fn materialize_arg_lhs(&self, lhs: &abstract_type::ArgLhs, locals: Locals<'ty, 'object>)
        -> (TypeRef<'ty, 'object>, Locals<'ty, 'object>)
    {
        use abstract_type::ArgLhs;

        match *lhs {
            ArgLhs::Lvar { name: Id(ref loc, ref name) } => {
                let ty = self.tyenv.new_var(loc.clone());
                (ty, locals.assign_shadow(name.clone(), ty, loc))
            }
            ArgLhs::Mlhs { ref loc, ref items } => {
                let (tys, locals) = items.iter().fold((Vec::new(), locals), |(mut tys, locals), item| {
                    let (ty, locals) = self.materialize_arg_lhs(item, locals);
                    tys.push(ty);
                    (tys, locals)
                });

                let ty = self.tyenv.tuple(loc.clone(), tys, None, vec![]);

                (ty, locals)
            }
        }
    }

    fn materialize_arg(&self, arg: &abstract_type::ArgNode<'object>, locals: Locals<'ty, 'object>, context: &TypeContext<'ty, 'object>)
        -> (Arg<'ty, 'object>, Locals<'ty, 'object>)
    {
        use abstract_type::ArgNode;

        let (ty, loc) = match *arg {
            ArgNode::Required { ref ty, ref loc, .. } |
            ArgNode::Optional { ref ty, ref loc, .. } |
            ArgNode::Rest { ref ty, ref loc, .. } |
            ArgNode::Kwarg { ref ty, ref loc, .. } |
            ArgNode::Kwoptarg { ref ty, ref loc, .. } |
            ArgNode::Kwrest { ref ty, ref loc, .. } |
            ArgNode::Block { ref ty, ref loc, .. } =>
                (ty.as_ref(), loc),
            ArgNode::Procarg0 { ref arg, ref loc } => {
                let (arg, locals) = self.materialize_arg(arg, locals, context);
                return (Arg::Procarg0 { loc: loc.clone(), arg: Box::new(arg) }, locals);
            }
        };

        let ty = match ty {
            Some(ty) => self.materialize_type(ty, context),
            None => self.tyenv.new_var(loc.clone()),
        };

        let (arg, locals) = match *arg {
            ArgNode::Required { ref lhs, .. } => {
                let (tyvar, locals) = self.materialize_arg_lhs(lhs, locals);

                // we assert compatibility in the direction of external
                // interface (the ty attached to this ArgNode) -> the arg's
                // internal bindings (from tyvar, may be an mlhs tuple):
                self.tyenv.compatible(tyvar, ty)
                    .expect("abstract_type ensures that this can not fail");

                (Arg::Required { loc: loc.clone(), ty: ty }, locals)
            }
            ArgNode::Optional { name: Id(_, ref name), ref default, .. } =>
                (Arg::Optional { loc: loc.clone(), ty: ty, expr: default.expr.clone() },
                    locals.assign_shadow(name.clone(), ty, loc)),
            ArgNode::Rest { name: None, .. } =>
                (Arg::Rest { loc: loc.clone(), ty: ty }, locals),
            ArgNode::Rest { name: Some(Id(_, ref name)), .. } =>
                (Arg::Rest { loc: loc.clone(), ty: ty },
                    locals.assign_shadow(name.clone(), self.tyenv.array(loc.clone(), ty), loc)),
            ArgNode::Kwarg { name: Id(_, ref name), .. } =>
                (Arg::Kwarg { loc: loc.clone(), name: name.clone(), ty: ty },
                    locals.assign_shadow(name.clone(), ty, loc)),
            ArgNode::Kwoptarg { name: Id(_, ref name), ref default, .. } =>
                (Arg::Kwoptarg { loc: loc.clone(), name: name.clone(), ty: ty, expr: default.expr.clone() },
                    locals.assign_shadow(name.clone(), ty, loc)),
            ArgNode::Kwrest { name: None, .. } =>
                (Arg::Kwrest { loc: loc.clone(), ty: ty }, locals),
            ArgNode::Kwrest { name: Some(Id(_, ref name)), .. } => {
                let hash_ty = self.tyenv.hash(loc.clone(),
                    self.tyenv.instance0(loc.clone(), self.env.object.Symbol), ty);
                (Arg::Kwrest { loc: loc.clone(), ty: ty },
                    locals.assign_shadow(name.clone(), hash_ty, loc))
            }
            ArgNode::Block { name: None, .. } =>
                (Arg::Block { loc: loc.clone(), ty: ty }, locals),
            ArgNode::Block { name: Some(Id(_, ref name)), .. } =>
                (Arg::Block { loc: loc.clone(), ty: ty },
                    locals.assign_shadow(name.clone(), ty, loc)),
            ArgNode::Procarg0 { .. } =>
                panic!("impossible")
        };

        (arg, locals)
    }

    pub fn materialize_prototype(&self, prototype: &abstract_type::Prototype<'object>, locals: Locals<'ty, 'object>, context: &mut TypeContext<'ty, 'object>)
        -> (Rc<Prototype<'ty, 'object>>, Locals<'ty, 'object>)
    {
        for &Id(ref loc, ref name) in &prototype.type_vars {
            context.type_names.insert(name.clone(),
                self.tyenv.new_var(loc.clone()));
        }

        let constraints = prototype.type_constraints.iter().map(|constraint| {
            match constraint {
                &abstract_type::TypeConstraint::Compatible { ref loc, ref sub, ref super_ } =>
                    TypeConstraint::Compatible {
                        loc: loc.clone(),
                        sub: self.materialize_type(sub, context),
                        super_: self.materialize_type(super_, context),
                    },
            }
        }).collect();

        let (args, locals) = prototype.args.iter().fold((Vec::new(), locals),
            |(mut args, locals), arg| {
                let (arg, locals_) = self.materialize_arg(arg, locals, context);
                args.push(arg);
                (args, locals_)
            });

        let retn = match prototype.retn.as_ref() {
            Some(retn) => self.materialize_type(retn, context),
            None => self.tyenv.new_var(prototype.loc.clone()),
        };

        let proto = Rc::new(Prototype {
            loc: prototype.loc.clone(),
            constraints: constraints,
            args: args,
            retn: retn,
        });

        (proto, locals)
    }
}
