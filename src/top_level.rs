use ast::{Id, Node};
use environment::Environment;
use object::{RubyObject, Scope, MethodEntry, IvarEntry};
use std::rc::Rc;
use errors::Detail;

type EvalResult<'a, T> = Result<T, (&'a Node, &'static str)>;

struct Eval<'env, 'object: 'env> {
    pub env: &'env Environment<'object>,
    pub scope: Rc<Scope<'object>>,
}

#[derive(Copy,Clone)]
enum AttrType {
    Reader,
    Writer,
    Accessor,
}

impl AttrType {
    fn reader(self) -> bool {
        match self {
            AttrType::Reader | AttrType::Accessor => true,
            AttrType::Writer => false,
        }
    }

    fn writer(self) -> bool {
        match self {
            AttrType::Writer | AttrType::Accessor => true,
            AttrType::Reader => false,
        }
    }
}

impl<'env, 'object> Eval<'env, 'object> {
    fn error(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().error(message, details)
    }

    fn warning(&self, message: &str, details: &[Detail]) {
        self.env.error_sink.borrow_mut().warning(message, details)
    }

    fn resolve_cpath<'a>(&self, node: &'a Node) -> EvalResult<'a, &'object RubyObject<'object>> {
        self.env.resolve_cpath(node, self.scope.clone())
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
            let eval = Eval {
                env: self.env,
                scope: Scope::spawn(&self.scope, module),
            };

            eval.eval_node(node)
        }
    }

    fn decl_class(&self, name: &Node, type_parameters: &[Rc<Node>], superclass: &Option<Rc<Node>>, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let superclass = superclass.as_ref().and_then(|node| {
            match self.resolve_cpath(node) {
                Ok(value) => Some((node, value)),
                Err((node, message)) => {
                    self.error(&message, &[Detail::Loc("here", node.loc())]);
                    None
                }
            }
        });

        let class = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                match self.env.object.get_const_for_definition(&base, id) {
                    Some(object_ref@&RubyObject::Object { .. }) => {
                        self.error(&format!("{} is not a class", id), &[
                            Detail::Loc("here", name.loc()),
                            // TODO - show location of previous definition
                        ]);

                        // open the object's metaclass instead as error recovery:
                        self.env.object.metaclass(object_ref)
                    }
                    Some(module_ref@&RubyObject::Module { .. }) => {
                        self.error(&format!("{} is not a class", id), &[
                            Detail::Loc("here", name.loc()),
                            // TODO - show location of previous definition
                        ]);

                        // open the module instead:
                        module_ref
                    }
                    Some(class_ref@&RubyObject::Class { .. }) |
                    Some(class_ref@&RubyObject::Metaclass { .. }) => {
                        // check superclass matches
                        if let Some((ref superclass_node, ref superclass)) = superclass {
                            let existing_superclass = class_ref.superclass();
                            if Some(superclass.clone()) != existing_superclass {
                                let existing_superclass_name =
                                    match existing_superclass {
                                        Some(existing_superclass) => existing_superclass.name(),
                                        None => "nil".to_owned(),
                                    };

                                self.error(&format!("Superclass does not match existing superclass {}", existing_superclass_name), &[
                                    Detail::Loc("here", superclass_node.loc()),
                                    // TODO - show location of previous definition
                                ]);
                            }
                        }

                        class_ref
                    }
                    Some(&RubyObject::IClass { .. }) => panic!(),
                    None => {
                        let superclass = match superclass {
                            Some((_, ref superclass)) => superclass,
                            None => &self.env.object.Object,
                        };

                        let type_parameters =
                            if superclass.type_parameters().is_empty() {
                                type_parameters.iter().map(|param|
                                    if let Node::TyGendeclarg(ref loc, ref name) = **param {
                                        Id(loc.clone(), name.to_owned())
                                    } else {
                                        panic!("expected TyGendeclarg in TyGendecl");
                                    }
                                ).collect()
                            } else if type_parameters.is_empty() {
                                Vec::new()
                            } else {
                                let loc = type_parameters.first().unwrap().loc().join(
                                            type_parameters.last().unwrap().loc());

                                self.error("Subclasses of generic classes may not specify type parameters", &[
                                    Detail::Loc("here", &loc),
                                ]);

                                Vec::new()
                            };

                        let class = self.env.object.new_class(
                            self.env.object.constant_path(&base, id),
                            superclass, type_parameters);

                        if !self.env.object.set_const(&base, id, Some(name.loc().clone()), &class) {
                            panic!("internal error: would overwrite existing constant");
                        }

                        class
                    }
                }
            }
            Err((node, message)) => {
                self.error(&message, &[Detail::Loc("here", node.loc())]);
                return;
            }
        };

        self.enter_scope(class, body);
    }

    fn decl_module(&self, name: &Node, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let module = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                match self.env.object.get_const_for_definition(&base, id) {
                    Some(const_value@&RubyObject::Object { .. }) |
                    Some(const_value@&RubyObject::Class { .. }) |
                    Some(const_value@&RubyObject::Metaclass { .. }) => {
                        self.error(&format!("{} is not a module", id), &[
                            Detail::Loc("here", name.loc()),
                            // TODO show location of previous definition
                        ]);

                        const_value.clone()
                    }
                    Some(&RubyObject::IClass { .. }) => panic!(),
                    Some(const_value@&RubyObject::Module { .. }) =>
                        const_value.clone(),
                    None => {
                        let module = self.env.object.new_module(
                            self.env.object.constant_path(&base, id));

                        if !self.env.object.set_const(&base, id, Some(name.loc().clone()), &module) {
                            panic!("internal error: would overwrite existing constant");
                        }

                        module
                    }
                }
            }
            e@Err(..) => panic!("{:?}", e) /* TODO handle error */,
        };

        self.enter_scope(module, body);
    }

    fn decl_method(&self, target: &'object RubyObject<'object>, name: &str, def_node: &Rc<Node>) {
        let method = Rc::new(MethodEntry::Ruby {
            owner: target,
            name: name.to_owned(),
            node: def_node.clone(),
            scope: self.scope.clone(),
        });

        self.env.object.define_method(target, name.to_owned(), method.clone());

        self.env.enqueue_method_for_type_check(method);
    }

    fn symbol_name<'node>(&self, node: &'node Rc<Node>, msg: &str) -> Option<&'node str> {
        match **node {
            Node::Symbol(_, ref sym) => Some(sym),
            _ => {
                self.warning(&format!("Dynamic symbol {}", msg), &[
                    Detail::Loc("here", node.loc()),
                ]);

                None
            }
        }
    }

    fn alias_method(&self, klass: &'object RubyObject<'object>, from: &Rc<Node>, to: &Rc<Node>) {
        let from_name = self.symbol_name(from, "in alias");
        let to_name = self.symbol_name(to, "in alias");

        if let Some(method) = from_name.and_then(|name| self.env.object.lookup_method(klass, name)) {
            if let Some(name) = to_name {
                self.env.object.define_method(klass, name.to_owned(), method.clone());
            }
        } else {
            if let Some(name) = from_name {
                // no need to check None case, symbol_name would have already emitted an error
                self.error("Could not resolve source method in alias", &[
                    Detail::Loc(&format!("{}#{}", klass.name(), name), from.loc()),
                ]);
            }

            if let Some(name) = to_name {
                // define alias target as untyped so that uses of it don't produce even more errors:
                self.env.object.define_method(klass, name.to_owned(), Rc::new(MethodEntry::Untyped));
            }
        }
    }

    fn process_attr(&self, attr_type: AttrType, args: &[Rc<Node>]) {
        // TODO need to decouple self from the current module in scope so we
        // can ignore errant attr_* calls at the top level.

        let class = self.scope.module;

        for arg in args {
            if let Some(sym) = self.symbol_name(arg, "in attribute name") {
                let ivar = format!("@{}", sym);

                if attr_type.reader() {
                    let method = MethodEntry::AttrReader {
                        ivar: ivar.clone(),
                        node: arg.clone(),
                    };
                    self.env.object.define_method(class, sym.to_owned(), Rc::new(method));
                }

                if attr_type.writer() {
                    let method = MethodEntry::AttrWriter {
                        ivar: ivar.clone(),
                        node: arg.clone(),
                    };
                    self.env.object.define_method(class, sym.to_owned() + "=", Rc::new(method));
                }
            }
        }
    }

    fn process_self_send(&self, id: &Id, args: &[Rc<Node>]) {
        match id.1.as_str() {
            "include" => {
                if args.is_empty() {
                    self.error("Wrong number of arguments to include", &[
                        Detail::Loc("here", &id.0),
                    ]);
                }

                for arg in args {
                    match self.resolve_static(arg) {
                        Ok(obj) => {
                            if !self.env.object.include_module(&self.scope.module, &obj) {
                                self.error("Cyclic include", &[
                                    Detail::Loc("here", arg.loc()),
                                ])
                            }
                        }
                        Err((node, message)) => {
                            self.warning("Could not statically resolve module reference in include", &[
                                Detail::Loc(message, node.loc()),
                            ]);
                        }
                    }
                }
            }
            "require" => {
                if args.len() == 0 {
                    self.error("Missing argument to require", &[
                        Detail::Loc("here", &id.0),
                    ]);
                    return;
                }

                if args.len() > 1 {
                    self.error("Too many arguments to require", &[
                        Detail::Loc("from here", args[1].loc()),
                    ]);
                    return;
                }

                match *args[0] {
                    Node::String(ref loc, ref string) => {
                        if let Some(path) = self.env.search_require_path(string) {
                            match self.env.require(&path) {
                                Ok(()) => {}
                                Err(e) => panic!("TODO: implement error handling for require errors: {:?}", e),
                            }
                        } else {
                            self.error("Could not resolve require", &[
                                Detail::Loc("here", loc),
                            ]);
                        }
                    }
                    _ => {
                        self.error("Could not resolve dynamic path in require", &[
                            Detail::Loc("here", args[0].loc()),
                        ]);
                    }
                }
            }
            "attr_reader" => self.process_attr(AttrType::Reader, args),
            "attr_writer" => self.process_attr(AttrType::Writer, args),
            "attr_accessor" => self.process_attr(AttrType::Accessor, args),
            _ => {}
        }
    }

    fn eval_maybe_node(&self, node: &Option<Rc<Node>>) {
        if let Some(ref node) = *node {
            self.eval_node(node);
        }
    }

    fn eval_node(&self, node: &Rc<Node>) {
        match **node {
            Node::Begin(_, ref stmts) => {
                for stmt in stmts {
                    self.eval_node(stmt);
                }
            }
            Node::Kwbegin(_, ref node) => {
                self.eval_maybe_node(node);
            }
            Node::Class(_, ref declname, ref superclass, ref body) => {
                match **declname {
                    Node::TyGendecl(_, ref name, ref genargs) =>
                        self.decl_class(name, genargs.as_slice(), superclass, body),
                    Node::Const(..) =>
                        self.decl_class(declname, &[], superclass, body),
                    _ =>
                        panic!("bad node type in class declname position"),
                }
            }
            Node::Module(_, ref name, ref body) => {
                self.decl_module(name, body);
            }
            Node::SClass(_, ref expr, ref body) => {
                let singleton = match self.resolve_static(expr) {
                    Ok(singleton) => singleton,
                    Err((node, message)) => {
                        self.warning("Could not statically resolve singleton expression", &[
                            Detail::Loc(message, node.loc()),
                        ]);
                        return;
                    }
                };

                let metaclass = self.env.object.metaclass(&singleton);

                self.enter_scope(metaclass, body);
            }
            Node::Def(_, Id(_, ref name), ..) => {
                self.decl_method(&self.scope.module, name, node);
            }
            Node::Defs(_, ref singleton, Id(_, ref name), ..) => {
                match self.resolve_static(singleton) {
                    Ok(metaclass) => {
                        let metaclass = self.env.object.metaclass(&metaclass);
                        self.decl_method(&metaclass, name, node);
                    }
                    Err((node, message)) => {
                        self.error(message, &[Detail::Loc("here", node.loc())]);
                    }
                }
            }
            Node::Undef(_, ref names) => {
                // TODO
            }
            Node::Send(_, None, ref id@Id(..), ref args) => {
                self.process_self_send(id, args.as_slice());
            }
            Node::Send(_, Some(ref recv), _, ref args) => {
                self.eval_node(recv);
                for arg in args {
                    self.eval_node(arg);
                }
            }
            Node::Casgn(_, ref base, Id(ref name_loc, ref name), ref expr) => {
                let loc = match *base {
                    Some(ref base_node) => base_node.loc().join(name_loc),
                    None => name_loc.clone(),
                };

                match self.resolve_cbase(base) {
                    Ok(cbase) => {
                        if self.env.object.has_own_const(&cbase, name) {
                            self.error("Constant reassignment", &[
                                Detail::Loc("here", &loc),
                                // TODO show where constant was previously set
                            ]);
                            return;
                        }
                        match **expr {
                            Node::Const { .. } => {
                                if let Ok(value) = self.resolve_cpath(expr) {
                                    self.env.object.set_const(&cbase, name, Some(loc), &value);
                                }
                            }
                            // TODO handle send
                            // TODO handle tr_cast
                            // TODO handle unresolved expressions
                            _ => {}
                        }
                    }
                    Err((node, message)) => {
                        self.warning("Could not statically resolve constant in assignment", &[
                            Detail::Loc(message, node.loc()),
                        ]);
                    }
                }
            }
            Node::Alias(_, ref to, ref from) => {
                self.alias_method(self.scope.module, from, to);
            }
            Node::TyIvardecl(_, Id(ref ivar_loc, ref ivar), ref type_node) => {
                if let Some(ivar_decl) = self.env.object.lookup_ivar(&self.scope.module, ivar) {
                    self.error("Duplicate instance variable type declaration", &[
                        Detail::Loc("here", ivar_loc),
                        Detail::Loc("previous declaration was here", &ivar_decl.ivar_loc),
                    ]);
                } else {
                    self.env.object.define_ivar(&self.scope.module, ivar.to_owned(), Rc::new(IvarEntry {
                        ivar_loc: ivar_loc.clone(),
                        type_node: type_node.clone(),
                        scope: self.scope.clone(),
                    }));
                }
            }
            Node::Block(_, ref send_node, ref block_args, ref block_body) => {
                self.eval_node(send_node);
                self.eval_node(block_args);
                self.eval_maybe_node(block_body);
            }
            Node::Const(..) => {
                // try to autoload this const, but ignore any errors
                let _ = self.resolve_cpath(node);
            }
            Node::Args(_, ref args) => {
                for arg in args {
                    self.eval_node(arg);
                }
            }
            Node::Procarg0(_, ref arg) => {
                self.eval_node(arg);
            }
            Node::Arg(..) => {}
            Node::If(_, ref cond, ref then, ref else_) => {
                self.eval_node(cond);
                self.eval_maybe_node(then);
                self.eval_maybe_node(else_);
            }
            Node::Lvar(..) => {}
            Node::Symbol(..) => {}
            Node::Defined(..) => {}
            Node::Rescue(_, ref body, ref rescues, ref else_) => {
                self.eval_maybe_node(body);
                for rescue in rescues {
                    self.eval_node(rescue);
                }
                self.eval_maybe_node(else_);
            }
            Node::Resbody(_, ref class, ref lvar, ref body) => {
                self.eval_maybe_node(class);
                self.eval_maybe_node(lvar);
                self.eval_maybe_node(body);
            }
            Node::Array(_, ref elements) => {
                for element in elements {
                    self.eval_node(element);
                }
            }
            Node::String(..) => {}
            Node::Or(_, ref left, ref right) |
            Node::And(_, ref left, ref right) => {
                self.eval_node(left);
                self.eval_node(right);
            }
            _ => panic!("unknown node: {:?}", node),
        }
    }
}

pub fn evaluate<'env, 'object: 'env>(env: &'env Environment<'object>, node: Rc<Node>) {
    let scope = Rc::new(Scope { parent: None, module: env.object.Object });

    Eval { env: env, scope: scope }.eval_node(&node);
}
