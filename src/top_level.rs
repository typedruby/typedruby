use ast::{SourceFile, Id, Node};
use environment::Environment;
use object::{RubyObjectRef, ObjectType};
use std::rc::Rc;

struct Scope {
    pub parent: Option<Rc<Scope>>,
    pub module: RubyObjectRef,
}

impl Scope {
    pub fn root(scope: &Rc<Scope>) -> Rc<Scope> {
        match scope.parent {
            Some(ref parent) => Scope::root(parent),
            None => scope.clone(),
        }
    }

    pub fn ancestors(scope: &Rc<Scope>) -> ScopeIter {
        ScopeIter { scope: Some(scope.clone()) }
    }

    pub fn spawn(scope: &Rc<Scope>, module: RubyObjectRef) -> Rc<Scope> {
        Rc::new(Scope { parent: Some(scope.clone()), module: module })
    }
}

struct ScopeIter {
    scope: Option<Rc<Scope>>,
}

impl Iterator for ScopeIter {
    type Item = Rc<Scope>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scope.clone().map(|scope| {
            self.scope = scope.parent.clone();
            scope
        })
    }
}

type EvalResult<'a, T> = Result<T, (&'a Node, &'static str)>;

struct Eval<'ev, 'evinterior: 'ev> {
    pub env: &'ev Environment<'evinterior>,
    pub scope: &'ev Rc<Scope>,
}

impl<'ev, 'evinterior> Eval<'ev, 'evinterior> {
    fn resolve_cpath<'a>(&self, node: &'a Node) -> EvalResult<'a, RubyObjectRef> {
        match *node {
            Node::Cbase(_) =>
                Ok(Scope::root(self.scope).module.clone()),

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
                for scope in Scope::ancestors(self.scope) {
                    if let Some(obj) = self.env.object.get_const(&scope.module, name) {
                        return Ok(obj);
                    }
                }

                for scope in Scope::ancestors(self.scope) {
                    // TODO autoload
                }

                Err((node, "no such constant"))
            }

            _ =>
                Err((node, "not a static cpath")),
        }
    }

    fn resolve_cbase<'a>(&self, cbase: &'a Option<Rc<Node>>) -> EvalResult<'a, RubyObjectRef> {
        match *cbase {
            None => Ok(self.scope.module.clone()),
            Some(ref cbase_node) => self.resolve_cpath(cbase_node),
        }
    }

    fn resolve_decl_ref<'a>(&self, name: &'a Node) -> EvalResult<'a, (RubyObjectRef, &'a str)> {
        if let Node::Const(_, ref base, Id(_, ref id)) = *name {
            match *base {
                Some(ref base_node) => self.resolve_cpath(base_node).map(|object_ref| (object_ref, id.as_str())),
                None => Ok((self.scope.module.clone(), id.as_str())),
            }
        } else {
            Err((name, "Class name is not a static constant"))
        }
    }

    fn resolve_static<'a>(&self, node: &'a Node) -> EvalResult<'a, RubyObjectRef> {
        let target = match *node {
            Node::Self_(_) => &self.scope.module,
            Node::Const(..) => return self.resolve_cpath(node),
            _ => panic!("unknown static node {:?}", node),
        };

        Ok(target.clone())
    }

    fn enter_scope(&self, module: RubyObjectRef, body: &Option<Rc<Node>>) {
        if let Some(ref node) = *body {
            Eval { env: self.env, scope: &Scope::spawn(self.scope, module) }.eval_node(node)
        }
    }

    fn decl_class(&self, name: &Node, genargs: &[Rc<Node>], superclass: &Option<Rc<Node>>, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let superclass = superclass.as_ref().map(|node| self.resolve_cpath(node).unwrap() /* TODO handle error */);

        let class = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                match self.env.object.get_const_for_definition(&base, id) {
                    Some(ref const_value) =>
                        match self.env.object.type_of(const_value) {
                            ObjectType::Object |
                            ObjectType::Module => {
                                // TODO handle error
                                panic!("not a class!");
                            },
                            ObjectType::Class |
                            ObjectType::Metaclass => {
                                if let Some(ref superclass) = superclass {
                                    if Some(superclass.clone()) != self.env.object.superclass(const_value) {
                                        // TODO handle error
                                        panic!("superclass mismatch!");
                                    }
                                }

                                const_value.clone()
                            },
                        },
                    None => {
                        let class = self.env.object.new_class(
                            self.env.object.constant_path(&base, id),
                            &superclass.unwrap_or(self.env.object.Object.clone()));

                        if !self.env.object.set_const(&base, id, &class) {
                            panic!("internal error: would overwrite existing constant");
                        }

                        class
                    }
                }
            },
            e@Err(..) => panic!("{:?}", e) /* TODO handle error */,
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
                                // TODO handle error
                                panic!("not a module!");
                            },
                            ObjectType::Module => const_value.clone(),
                        },
                    None => {
                        let module = self.env.object.new_module(
                            self.env.object.constant_path(&base, id));

                        if !self.env.object.set_const(&base, id, &module) {
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

    fn decl_method(&self, target: &RubyObjectRef, name: &str, def_node: &Rc<Node>) {
        self.env.object.define_method(target, name.to_owned(), def_node.clone())
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
            Node::Def(_, Id(_, ref name), ..) => {
                self.decl_method(&self.scope.module, name, node);
            },
            Node::Defs(_, ref singleton, Id(_, ref name), ..) => {
                match self.resolve_static(singleton) {
                    Ok(metaclass) => {
                        let metaclass = self.env.object.metaclass(&metaclass);
                        self.decl_method(&metaclass, name, node);
                    },
                    e@Err(..) =>
                        // TODO handle error
                        panic!("{:?}", e),
                }
            },
            Node::Send(_, None, Id(_, ref name), ref args) => {
                if name == "include" {
                    if args.is_empty() {
                        // TODO handle error
                        panic!("useless include!");
                    }

                    for arg in args {
                        match self.resolve_static(arg) {
                            Ok(obj) => {
                                if !self.env.object.include_module(&self.scope.module, &obj) {
                                    // cyclic include
                                    // TODO handle error
                                }
                            },
                            e@Err(..) =>
                                // TODO handle error
                                panic!("{:?}", e),
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
            Node::Casgn(_, ref base, Id(_, ref name), ref expr) => {
                match self.resolve_cbase(base) {
                    Ok(cbase) => {
                        match **expr {
                            Node::Const { .. } =>
                                match self.resolve_cpath(expr) {
                                    Ok(value) =>
                                        if !self.env.object.set_const(&cbase, name, &value) {
                                            // TODO error on reassigning constants
                                            panic!("reassigning constant that's already set");
                                        },
                                    e@Err(..) =>
                                        // TODO handle error
                                        panic!("{:?}", e),
                                },
                            // TODO handle send
                            // TODO handle tr_cast
                            // TODO handle unresolved expressions
                            _ => {},
                        }
                    },
                    e@Err(..) =>
                        // TODO handle error
                        panic!("{:?}", e),
                }
            },
            Node::Alias(_, ref from, ref to) => {
                // TODO
            }
            _ => panic!("unknown node: {:?}", node),
        }
    }
}

pub fn evaluate(env: &Environment, source_file: &SourceFile) {
    let ast = source_file.parse();
    let scope = Rc::new(Scope { parent: None, module: env.object.Object.clone() });

    if let Some(ref node) = ast.node {
        Eval { env: env, scope: &scope }.eval_node(node);
    }
}
