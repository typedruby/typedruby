use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ast::{Id, Node, Loc};
use errors::Detail;
use environment::Environment;
use object::{RubyObject, Scope, MethodEntry, MethodImpl, ObjectGraph, IvarEntry};
use abstract_type::{TypeNode, TypeScope, Prototype, AnnotationStatus};

#[derive(Copy,Clone,Debug)]
pub enum MethodVisibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug)]
pub enum MethodDef<'object> {
    Def {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
        node: Rc<Node>,
        scope: Rc<Scope<'object>>,
    },
    Prototype {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
        node: Rc<Node>,
        scope: Rc<Scope<'object>>,
    },
    Alias {
        module: &'object RubyObject<'object>,
        to: Id,
        from: Id,
        emit_error: bool,
    },
    AttrReader {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
    },
    AttrWriter {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
    },
    SetVisi {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
        emit_error: bool,
    },
    ModuleFunc {
        module: &'object RubyObject<'object>,
        name: Id,
        emit_error: bool,
    },
}

#[derive(Debug)]
pub struct IvarDef<'object> {
    pub module: &'object RubyObject<'object>,
    pub name: Id,
    pub type_node: Rc<Node>,
    pub scope: Rc<Scope<'object>>,
}

#[derive(Debug)]
pub struct Definitions<'object> {
    methods: RefCell<Vec<MethodDef<'object>>>,
    ivars: RefCell<Vec<IvarDef<'object>>>,
}

impl<'object> Definitions<'object> {
    pub fn new() -> Self {
        Definitions {
            methods: RefCell::new(Vec::new()),
            ivars: RefCell::new(Vec::new()),
        }
    }

    pub fn add_method(&self, method: MethodDef<'object>) {
        self.methods.borrow_mut().push(method);
    }

    pub fn add_ivar(&self, ivar: IvarDef<'object>) {
        self.ivars.borrow_mut().push(ivar);
    }

    pub fn define(&self, env: &Environment<'object>) -> Vec<Rc<MethodEntry<'object>>> {
        let mut methods = self.methods.borrow_mut();
        let mut ivars = self.ivars.borrow_mut();

        for ivar in ivars.drain(0..) {
            define_ivar(env, ivar)
        }

        let mut method_entries = Vec::new();

        for method in methods.drain(0..) {
            if let Some(entry) = define_method(env, method) {
                method_entries.push(entry)
            }
        }

        method_entries
    }
}

fn lookup_visi<'o>(module: &'o RubyObject<'o>, name: &str, object: &ObjectGraph<'o>)
    -> Option<Rc<MethodEntry<'o>>>
{
    object.lookup_method(module, name).or_else(|| {
        if let RubyObject::Module { .. } = *module {
            object.lookup_method(object.Object, name)
        } else {
            None
        }
    })
}

fn stub_location(method: &MethodEntry) -> Option<Loc> {
    let proto = match *method.implementation {
        MethodImpl::TypedRuby{ref proto, ..} |
        MethodImpl::Ruby{ref proto, ..} => proto,
        _ => return None
    };
    if proto.is_stub {
        Some(proto.loc.clone())
    } else {
        None
    }
}

fn define_method<'o>(env: &Environment<'o>, method: MethodDef<'o>)
    -> Option<Rc<MethodEntry<'o>>>
{
    match method {
        MethodDef::Def { module, visi, name: Id(name_loc, name), node, scope } => {
            if let Some(ref meth) = env.object.lookup_method(module, &name) {
                if let Some(ref loc) = stub_location(meth) {
                    env.error_sink.borrow_mut().error(
                        "Can't redefine a method that was defined as a stub.", &[
                            Detail::Loc("re-defined here", &node.loc()),
                            Detail::Loc("stub defined here", &loc),
                        ]);
                    return None;
                }
            }

            let (proto, body) = match *node {
                Node::Def(_, _, ref proto, ref body) => (proto, body),
                Node::Defs(_, _, _, ref proto, ref body) => (proto, body),
                _ => panic!("unexpected node type"),
            };

            let type_scope = TypeScope::new(scope.clone(), module);

            let (anno, proto) = Prototype::resolve(&name_loc, proto.as_ref().map(Rc::as_ref), env, type_scope);

            if anno == AnnotationStatus::Partial {
                env.error_sink.borrow_mut().error("Partial type signatures are not permitted in method definitions", &[
                    Detail::Loc("all arguments and return value must be annotated", &proto.loc),
                ]);
            }

            let impl_ = if anno == AnnotationStatus::Typed {
                MethodImpl::TypedRuby {
                    name: name.clone(),
                    body: body.clone(),
                    proto: proto,
                    scope: scope,
                }
            } else {
                MethodImpl::Ruby {
                    name: name.clone(),
                    proto: proto,
                    body: body.clone(),
                }
            };

            let method = Rc::new(MethodEntry {
                owner: module,
                visibility: Cell::new(visi),
                implementation: Rc::new(impl_),
            });

            env.object.define_method(module, name, method.clone());

            return Some(method);
        }
        MethodDef::Prototype { module, visi, name: Id(name_loc, name), node, scope } => {
            let (proto, body) = match *node {
                Node::Def(_, _, ref proto, ref body) => (proto, body),
                Node::Defs(_, _, _, ref proto, ref body) => (proto, body),
                _ => panic!("unexpected node type"),
            };
            if let Some(ref bnode) = *body {
                env.error_sink.borrow_mut().error(
                    "Methods in .rbi files may not define bodies", &[
                        Detail::Loc("here", bnode.loc()),
                    ]);
            }

            let type_scope = TypeScope::new(scope.clone(), module);
            let (anno, mut proto) = Prototype::resolve(&name_loc, proto.as_ref().map(Rc::as_ref), env, type_scope);
            proto.is_stub = true;

            match anno {
                AnnotationStatus::Untyped | AnnotationStatus::Empty => {
                    env.error_sink.borrow_mut().error(
                        "Definitions in .rbi files must contain type annotations", &[
                            Detail::Loc("here", &proto.loc),
                        ]);
                }
                AnnotationStatus::Partial => {
                    env.error_sink.borrow_mut().error(
                        "Partial type signatures are not allowed in .rbi files.", &[
                            Detail::Loc("All arguments and return values must be annotated", &proto.loc),
                        ]);
                }
                _ => ()
            }

            let method = if let Some(meth) = env.object.lookup_method(module, &name) {
                let impl_ = match *meth.implementation {
                    MethodImpl::TypedRuby{ref name, ref body, ref scope, ..} => {
                        Rc::new(MethodImpl::TypedRuby{
                            name: name.to_owned(),
                            proto: proto,
                            body: body.clone(),
                            scope: scope.clone(),
                        })
                    },
                    MethodImpl::Ruby{ref name, ref body, ..} => {
                        Rc::new(MethodImpl::TypedRuby{
                            name: name.to_owned(),
                            proto: proto,
                            body: body.clone(),
                            scope: scope.clone(),
                        })
                    }
                    _ => meth.implementation.clone(),
                };

                Rc::new(MethodEntry {
                    owner: meth.owner,
                    visibility: meth.visibility.clone(),
                    implementation: impl_,
                })
            } else {
                Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(visi),
                    implementation: Rc::new(MethodImpl::TypedRuby {
                        name: name.clone(),
                        proto: proto,
                        scope: scope,
                        body: None,
                    }),
                })
            };

            env.object.define_method(module, name.to_owned(), method.clone());
            return Some(method);
        }
        MethodDef::Alias { module, to, from, emit_error } => {
            if let Some(method) = env.object.lookup_method(module, &from.1) {
                env.object.define_method(module, to.1, method.clone());
            } else {
                if emit_error {
                    env.error_sink.borrow_mut().error("Could not resolve source method in alias", &[
                        Detail::Loc(&format!("{}#{}", module.name(), from.1), &from.0),
                    ]);
                }

                // define alias target as untyped so that uses of it don't produce even more errors:
                env.object.define_method(module, to.1, Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(MethodVisibility::Public),
                    implementation: Rc::new(MethodImpl::Untyped),
                }));
            }
        }
        MethodDef::AttrReader { module, visi, name: Id(loc, name) } => {
            env.object.define_method(module, name.clone(),
                Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(visi),
                    implementation: Rc::new(MethodImpl::AttrReader {
                        ivar: format!("@{}", name),
                        loc: loc,
                    }),
                }))
        }
        MethodDef::AttrWriter { module, visi, name: Id(loc, name) } => {
            env.object.define_method(module, format!("{}=", name),
                Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(visi),
                    implementation: Rc::new(MethodImpl::AttrWriter {
                        ivar: format!("@{}", name),
                        loc: loc,
                    }),
                }))
        }
        MethodDef::SetVisi { module, visi, name: Id(loc, name), emit_error } => {
            if let Some(method) = lookup_visi(module, &name, &env.object) {
                if module == method.owner {
                    method.visibility.set(visi)
                } else {
                    env.object.define_method(module, name, Rc::new(MethodEntry {
                        owner: module,
                        visibility: Cell::new(visi),
                        implementation: method.implementation.clone(),
                    }))
                }
            } else {
                if emit_error {
                    env.error_sink.borrow_mut().error("Could not resolve method name in visibility declaration", &[
                        Detail::Loc("here", &loc),
                    ]);
                }
            }
        }
        MethodDef::ModuleFunc { module, name: Id(loc, name), emit_error } => {
            if let Some(method) = lookup_visi(module, &name, &env.object) {
                let target = env.object.metaclass(module);
                env.object.define_method(target, name, method);
            } else {
                if emit_error {
                    env.error_sink.borrow_mut().error("Could not resolve method name in module_function", &[
                        Detail::Loc("here", &loc),
                    ]);
                }
            }
        }
    }

    None
}

fn define_ivar<'o>(env: &Environment<'o>, ivar: IvarDef<'o>) {
    let IvarDef { module, name: Id(ivar_loc, ivar), type_node, scope } = ivar;

    if let Some(ivar_entry) = env.object.lookup_ivar(module, &ivar) {
        env.error_sink.borrow_mut().error("Duplicate instance variable type declaration", &[
            Detail::Loc("here", &ivar_loc),
            Detail::Loc("previous declaration was here", &ivar_entry.ivar_loc),
        ]);
    } else {
        let ty = TypeNode::resolve(&type_node, env, TypeScope::new(scope, module));

        env.object.define_ivar(module, ivar, Rc::new(IvarEntry {
            ivar_loc: ivar_loc,
            ty: ty,
        }));
    }
}
