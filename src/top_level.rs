use ast::{SourceFile, Id, Node, Loc};
use environment::Environment;
use object::{RubyObject, ObjectType, Scope};
use std::rc::Rc;

type EvalResult<'a, T> = Result<T, (&'a Node, &'static str)>;

struct Eval<'env, 'object: 'env> {
    pub env: &'env Environment<'object>,
    pub scope: Rc<Scope<'object>>,
    pub source_file: Rc<SourceFile>,
}

impl<'env, 'object> Eval<'env, 'object> {
    fn error(&self, message: &str, details: &[(&str, &SourceFile, &Loc)]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn warning(&self, message: &str, details: &[(&str, &SourceFile, &Loc)]) {
        self.env.error_sink.borrow_mut().warning(message, details)
    }

    fn resolve_cpath<'a>(&self, node: &'a Node) -> EvalResult<'a, &'object RubyObject<'object>> {
        match *node {
            Node::Cbase(_) =>
                Ok(Scope::root(&self.scope).module),

            Node::Const(_, Some(ref base), Id(_, ref name)) => {
                match self.resolve_cpath(base) {
                    Ok(base_ref) => match self.env.object.type_of(&base_ref) {
                        ObjectType::Object => Err((base, "not a class/module")),
                        _ => match self.env.object.get_const(&base_ref, name) {
                            Some(const_ref) => Ok(const_ref),
                            None => /* TODO autoload */ Err((node, "no such constant")),
                        }
                    },
                    error => error,
                }
            },

            Node::Const(_, None, Id(_, ref name)) => {
                for scope in Scope::ancestors(&self.scope) {
                    if let Some(obj) = self.env.object.get_const(&scope.module, name) {
                        return Ok(obj);
                    }
                }

                for scope in Scope::ancestors(&self.scope) {
                    // TODO autoload
                }

                Err((node, "no such constant"))
            }

            _ =>
                Err((node, "not a static cpath")),
        }
    }

    fn resolve_cbase<'a>(&self, cbase: &'a Option<Rc<Node>>) -> EvalResult<'a, &'object RubyObject<'object>> {
        match *cbase {
            None => Ok(self.scope.module.clone()),
            Some(ref cbase_node) => self.resolve_cpath(cbase_node),
        }
    }

    fn resolve_decl_ref<'a>(&self, name: &'a Node) -> EvalResult<'a, (&'object RubyObject<'object>, &'a str)> {
        if let Node::Const(_, ref base, Id(_, ref id)) = *name {
            match *base {
                Some(ref base_node) => self.resolve_cpath(base_node).map(|object_ref| (object_ref, id.as_str())),
                None => Ok((self.scope.module.clone(), id.as_str())),
            }
        } else {
            Err((name, "Class name is not a static constant"))
        }
    }

    fn resolve_static<'a>(&self, node: &'a Node) -> EvalResult<'a, &'object RubyObject<'object>> {
        match *node {
            Node::Self_(_) => Ok(self.scope.module.clone()),
            Node::Const(..) => return self.resolve_cpath(node),
            _ => Err((node, "unknown static node")),
        }
    }

    fn enter_scope(&self, module: &'object RubyObject<'object>, body: &Option<Rc<Node>>) {
        if let Some(ref node) = *body {
            let mut eval = Eval {
                env: self.env,
                scope: Scope::spawn(&self.scope, module),
                source_file: self.source_file.clone(),
            };

            eval.eval_node(node)
        }
    }

    fn decl_class(&self, name: &Node, genargs: &[Rc<Node>], superclass: &Option<Rc<Node>>, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let superclass = superclass.as_ref().map(|node| (node, self.resolve_cpath(node).unwrap() /* TODO handle error */));

        let class = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                match self.env.object.get_const_for_definition(&base, id) {
                    Some(ref const_value) =>
                        match self.env.object.type_of(const_value) {
                            ObjectType::Object |
                            ObjectType::Module => {
                                self.error(&format!("{} is not a class", id), &[
                                    ("here", &self.source_file, name.loc()),
                                    // TODO - show location of previous definition
                                ]);

                                const_value.clone()
                            },
                            ObjectType::Class |
                            ObjectType::Metaclass => {
                                // check superclass matches
                                if let Some((ref superclass_node, ref superclass)) = superclass {
                                    let existing_superclass = self.env.object.superclass(const_value);
                                    if Some(superclass.clone()) != existing_superclass {
                                        let existing_superclass_name =
                                            match existing_superclass {
                                                Some(existing_superclass) => existing_superclass.name(),
                                                None => "nil".to_owned(),
                                            };

                                        self.error(&format!("Superclass does not match existing superclass {}", existing_superclass_name), &[
                                            ("here", &self.source_file, superclass_node.loc()),
                                            // TODO - show location of previous definition
                                        ]);
                                    }
                                }

                                const_value.clone()
                            },
                        },
                    None => {
                        let class = self.env.object.new_class(
                            self.env.object.constant_path(&base, id),
                            match superclass {
                                Some((_, ref superclass)) => superclass,
                                None => &self.env.object.Object,
                            });

                        if !self.env.object.set_const(&base, id, self.source_file.clone(), name.loc().clone(), &class) {
                            panic!("internal error: would overwrite existing constant");
                        }

                        class
                    }
                }
            },
            Err((node, message)) => {
                self.error(&message, &[("here", &self.source_file, node.loc())]);
                return;
            },
        };

        self.enter_scope(class, body);
    }

    fn decl_module(&self, name: &Node, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let module = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                match self.env.object.get_const_for_definition(&base, id) {
                    Some(ref const_value) =>
                        match self.env.object.type_of(const_value) {
                            ObjectType::Object |
                            ObjectType::Class |
                            ObjectType::Metaclass => {
                                self.error(&format!("{} is not a module", id), &[
                                    ("here", &self.source_file, name.loc()),
                                    // TODO show location of previous definition
                                ]);

                                const_value.clone()
                            },
                            ObjectType::Module => const_value.clone(),
                        },
                    None => {
                        let module = self.env.object.new_module(
                            self.env.object.constant_path(&base, id));

                        if !self.env.object.set_const(&base, id, self.source_file.clone(), name.loc().clone(), &module) {
                            panic!("internal error: would overwrite existing constant");
                        }

                        module
                    }
                }
            },
            e@Err(..) => panic!("{:?}", e) /* TODO handle error */,
        };

        self.enter_scope(module, body);
    }

    fn decl_method(&self, target: &'object RubyObject<'object>, name: &str, def_node: &Rc<Node>) {
        self.env.object.define_method(target, name.to_owned(), self.source_file.clone(), def_node.clone())
    }

    fn eval_node(&self, node: &Rc<Node>) {
        match **node {
            Node::Begin(_, ref stmts) => {
                for stmt in stmts {
                    self.eval_node(stmt);
                }
            },
            Node::Class(_, ref declname, ref superclass, ref body) => {
                match **declname {
                    Node::TyGendecl(_, ref name, ref genargs) =>
                        self.decl_class(name, genargs.as_slice(), superclass, body),
                    Node::Const(..) =>
                        self.decl_class(declname, &[], superclass, body),
                    _ =>
                        panic!("bad node type in class declname position"),
                }
            },
            Node::Module(_, ref name, ref body) => {
                self.decl_module(name, body);
            },
            Node::SClass(_, ref expr, ref body) => {
                let singleton = match self.resolve_static(expr) {
                    Ok(singleton) => singleton,
                    Err((node, message)) => {
                        self.warning("Could not statically resolve singleton expression", &[
                            (message, &self.source_file, node.loc()),
                        ]);
                        return;
                    }
                };

                let metaclass = self.env.object.metaclass(&singleton);

                self.enter_scope(metaclass, body);
            },
            Node::Def(_, Id(_, ref name), ..) => {
                self.decl_method(&self.scope.module, name, node);
            },
            Node::Defs(_, ref singleton, Id(_, ref name), ..) => {
                match self.resolve_static(singleton) {
                    Ok(metaclass) => {
                        let metaclass = self.env.object.metaclass(&metaclass);
                        self.decl_method(&metaclass, name, node);
                    },
                    Err((node, message)) => {
                        self.error(message, &[("here", &self.source_file, node.loc())]);
                    },
                }
            },
            Node::Send(_, None, Id(ref id_loc, ref id), ref args) => {
                if id == "include" {
                    if args.is_empty() {
                        self.error("Wrong number of arguments to include", &[
                            ("here", &self.source_file, id_loc),
                        ]);
                    }

                    for arg in args {
                        match self.resolve_static(arg) {
                            Ok(obj) => {
                                if !self.env.object.include_module(&self.scope.module, &obj) {
                                    self.error("Cyclic include", &[
                                        ("here", &self.source_file, arg.loc()),
                                    ])
                                }
                            },
                            Err((node, message)) => {
                                self.warning("Could not statically resolve module reference in include", &[
                                    (message, &self.source_file, node.loc()),
                                ]);
                            }
                        }
                    }
                }
            },
            Node::Send(_, Some(ref recv), _, ref args) => {
                self.eval_node(recv);
                for arg in args {
                    self.eval_node(arg);
                }
            },
            Node::Casgn(_, ref base, Id(ref name_loc, ref name), ref expr) => {
                let loc = match *base {
                    Some(ref base_node) => base_node.loc().join(name_loc),
                    None => name_loc.clone(),
                };

                match self.resolve_cbase(base) {
                    Ok(cbase) => {
                        if self.env.object.has_own_const(&cbase, name) {
                            self.error("Constant reassignment", &[
                                ("here", &self.source_file, &loc),
                                // TODO show where constant was previously set
                            ]);
                            return;
                        }
                        match **expr {
                            Node::Const { .. } =>
                                if let Ok(value) = self.resolve_cpath(expr) {
                                    self.env.object.set_const(&cbase, name, self.source_file.clone(), loc, &value);
                                },
                            // TODO handle send
                            // TODO handle tr_cast
                            // TODO handle unresolved expressions
                            _ => {},
                        }
                    },
                    Err((node, message)) => {
                        self.warning("Could not statically resolve constant in assignment", &[
                            (message, &self.source_file, node.loc()),
                        ]);
                    }
                }
            },
            Node::Alias(_, ref from, ref to) => {
                // TODO
            }
            _ => panic!("unknown node: {:?}", node),
        }
    }
}

pub fn evaluate<'env, 'object: 'env>(env: &'env Environment<'object>, source_file: Rc<SourceFile>) {
    let scope = Rc::new(Scope { parent: None, module: env.object.Object });

    if let Some(ref node) = source_file.ast().node {
        Eval { env: env, scope: scope, source_file: source_file.clone() }.eval_node(node);
    }
}
