use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ast::{Id, Node};
use errors::{Detail, ErrorSink};
use environment::Environment;
use object::{RubyObject, Scope, MethodEntry, MethodImpl, ObjectGraph, IvarEntry};
use abstract_type::{TypeNode, TypeScope};

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
        name: String,
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
    }
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
            if let Some(entry) = define_method(&env.object, method, env.error_sink.borrow_mut().as_mut()) {
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

fn define_method<'o>(object: &ObjectGraph<'o>, method: MethodDef<'o>, errors: &mut ErrorSink)
    -> Option<Rc<MethodEntry<'o>>>
{
    match method {
        MethodDef::Def { module, visi, name, node, scope } => {
            let method = Rc::new(MethodEntry {
                owner: module,
                visibility: Cell::new(visi),
                implementation: Rc::new(MethodImpl::Ruby {
                    name: name.clone(),
                    node: node,
                    scope: scope,
                }),
            });

            object.define_method(module, name, method.clone());

            return Some(method);
        }
        MethodDef::Alias { module, to, from, emit_error } => {
            if let Some(method) = object.lookup_method(module, &from.1) {
                object.define_method(module, to.1, method.clone());
            } else {
                if emit_error {
                    errors.error("Could not resolve source method in alias", &[
                        Detail::Loc(&format!("{}#{}", module.name(), from.1), &from.0),
                    ]);
                }

                // define alias target as untyped so that uses of it don't produce even more errors:
                object.define_method(module, to.1, Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(MethodVisibility::Public),
                    implementation: Rc::new(MethodImpl::Untyped),
                }));
            }
        }
        MethodDef::AttrReader { module, visi, name: Id(loc, name) } => {
            object.define_method(module, name.clone(),
                Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(visi),
                    implementation: Rc::new(MethodImpl::AttrReader {
                        ivar: format!("@{}", name),
                        loc: loc,
                    })
                }))
        }
        MethodDef::AttrWriter { module, visi, name: Id(loc, name) } => {
            object.define_method(module, format!("{}=", name),
                Rc::new(MethodEntry {
                    owner: module,
                    visibility: Cell::new(visi),
                    implementation: Rc::new(MethodImpl::AttrWriter {
                        ivar: format!("@{}", name),
                        loc: loc,
                    })
                }))
        }
        MethodDef::SetVisi { module, visi, name: Id(loc, name), emit_error } => {
            if let Some(method) = lookup_visi(module, &name, object) {
                if module == method.owner {
                    method.visibility.set(visi)
                } else {
                    object.define_method(module, name, Rc::new(MethodEntry {
                        owner: module,
                        visibility: Cell::new(visi),
                        implementation: method.implementation.clone(),
                    }))
                }
            } else {
                if emit_error {
                    errors.error("Could not resolve method name in visibility declaration", &[
                        Detail::Loc("here", &loc),
                    ]);
                }
            }
        }
        MethodDef::ModuleFunc { module, name: Id(loc, name), emit_error } => {
            if let Some(method) = lookup_visi(module, &name, object) {
                let target = object.metaclass(module);
                object.define_method(target, name, method);
            } else {
                if emit_error {
                    errors.error("Could not resolve method name in module_function", &[
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
        let ty = TypeNode::resolve(&type_node, env, TypeScope::new(scope));

        env.object.define_ivar(module, ivar, Rc::new(IvarEntry {
            ivar_loc: ivar_loc,
            ty: ty,
        }));
    }
}
