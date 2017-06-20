use ast::{Id, Node, Loc, SourceFile};
use environment::Environment;
use errors::Detail;
use object::{RubyObject, Scope, MethodEntry, MethodVisibility, MethodImpl, IvarEntry, ConstantEntry};
use std::rc::Rc;
use std::cell::Cell;

type EvalResult<'a, T> = Result<T, (&'a Node, &'static str)>;

struct Eval<'env, 'object: 'env> {
    pub env: &'env Environment<'object>,
    pub scope: Rc<Scope<'object>>,
    source_file: Rc<SourceFile>,
    source_type: SourceType,
    in_def: bool,
    def_visibility: Cell<MethodVisibility>,
    module_function: Cell<bool>,
}

#[derive(Copy,Clone,Eq,PartialEq)]
pub enum SourceType {
    TypedRuby,
    Ruby,
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

enum RequireType {
    LoadPath,
    Relative,
}

impl<'env, 'object> Eval<'env, 'object> {
    fn new(env: &'env Environment<'object>, scope: Rc<Scope<'object>>, source_file: Rc<SourceFile>, source_type: SourceType, in_def: bool) -> Eval<'env, 'object> {
        Eval {
            env: env,
            scope: scope,
            source_file: source_file,
            source_type: source_type,
            in_def: in_def,
            def_visibility: Cell::new(MethodVisibility::Public),
            module_function: Cell::new(false),
        }
    }

    fn emit_errors(&self) -> bool {
        self.source_type == SourceType::TypedRuby &&
            self.env.should_emit_errors(self.source_file.filename())
    }

    fn error(&self, message: &str, details: &[Detail]) {
        if self.emit_errors() {
            self.env.error_sink.borrow_mut().error(message, details)
        }
    }

    fn warning(&self, message: &str, details: &[Detail]) {
        if self.emit_errors() {
            self.env.error_sink.borrow_mut().warning(message, details)
        }
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
            let eval = Eval::new(self.env, Scope::spawn(&self.scope, module), self.source_file.clone(), self.source_type, self.in_def);

            eval.eval_node(node)
        }
    }

    fn enter_def(&self, body: &Option<Rc<Node>>) {
        if let Some(ref node) = *body {
            let eval = Eval::new(self.env, self.scope.clone(), self.source_file.clone(), self.source_type, true);

            eval.eval_node(node)
        }
    }

    fn constant_definition_error(&self, message: &str, loc: &Loc, definition: &Option<Loc>) {
        let mut details = vec![
            Detail::Loc("here", loc),
        ];

        if let Some(ref loc) = *definition {
            details.push(Detail::Loc("previously defined here", loc));
        }

        self.error(message, details.as_slice());
    }

    fn decl_class(&self, name: &Node, type_parameters: &[Rc<Node>], superclass: &Option<Rc<Node>>, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let superclass = superclass.as_ref().and_then(|node| {
            match self.resolve_cpath(node) {
                Ok(value) => match *value {
                    RubyObject::Class { .. } |
                    RubyObject::Metaclass { .. } =>
                        Some((node, value)),
                    _ => {
                        self.error("Superclass is not a class", &[Detail::Loc("here", node.loc())]);
                        None
                    }
                },
                Err((node, message)) => {
                    self.warning(&message, &[Detail::Loc("here", node.loc())]);
                    None
                }
            }
        });

        let class = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                if let Some(constant_entry) = self.env.object.get_const_for_definition(&base, id) {
                    let value = constant_entry.value;
                    match *value {
                        RubyObject::Object { .. } => {
                            self.constant_definition_error(&format!("{} is not a class", id), name.loc(), &constant_entry.loc);

                            // open the object's metaclass instead as error recovery:
                            self.env.object.metaclass(value)
                        }
                        RubyObject::Module { .. } => {
                            self.constant_definition_error(&format!("{} is not a class", id), name.loc(), &constant_entry.loc);

                            // open the module instead:
                            value
                        }
                        RubyObject::Class { .. } |
                        RubyObject::Metaclass { .. } => {
                            // check superclass matches
                            if let Some((ref superclass_node, ref superclass)) = superclass {
                                let existing_superclass = value.superclass();
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

                            value
                        }
                        RubyObject::IClass { .. } => panic!(),
                    }
                } else {
                    let superclass = match superclass {
                        Some((_, ref superclass)) => superclass,
                        None => &self.env.object.Object,
                    };

                    let type_parameters =
                        if superclass.type_parameters().is_empty() {
                            type_parameters.iter().map(|param|
                                match **param {
                                    Node::TyGendeclarg(ref loc, ref name, None) =>
                                        Id(loc.clone(), name.to_owned()),
                                    Node::TyGendeclarg(ref loc, ref name, Some(ref constraint)) => {
                                        self.error("Type constraints not permitted on class type parameters", &[
                                            Detail::Loc("here", constraint.loc()),
                                        ]);
                                        Id(loc.clone(), name.to_owned())
                                    },
                                    _ => panic!("expected TyGendeclarg in TyGendecl"),
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

                    let constant = Rc::new(ConstantEntry {
                        loc: Some(name.loc().clone()),
                        value: class,
                    });

                    if !self.env.object.set_const(&base, id, constant) {
                        panic!("internal error: would overwrite existing constant");
                    }

                    class
                }
            }
            Err((node, message)) => {
                self.warning(&message, &[Detail::Loc("here", node.loc())]);
                return;
            }
        };

        self.enter_scope(class, body);
    }

    fn decl_module(&self, name: &Node, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let module = match self.resolve_decl_ref(name) {
            Ok((base, id)) => {
                if let Some(constant_entry) = self.env.object.get_const_for_definition(&base, id) {
                    match constant_entry.value {
                        value@&RubyObject::Object { .. } |
                        value@&RubyObject::Class { .. } |
                        value@&RubyObject::Metaclass { .. } => {
                            self.constant_definition_error(&format!("{} is not a module", id), name.loc(), &constant_entry.loc);

                            value
                        }
                        &RubyObject::IClass { .. } => panic!(),
                        value@&RubyObject::Module { .. } => value,
                    }
                } else {
                    let module = self.env.object.new_module(
                        self.env.object.constant_path(&base, id));

                    let constant = Rc::new(ConstantEntry {
                        loc: Some(name.loc().clone()),
                        value: module,
                    });

                    if !self.env.object.set_const(&base, id, constant) {
                        panic!("internal error: would overwrite existing constant");
                    }

                    module
                }
            }
            Err((node, msg)) => {
                self.error(msg, &[Detail::Loc("here", node.loc())]);
                return
            }
        };

        self.enter_scope(module, body);
    }

    fn decl_method(&self, target: &'object RubyObject<'object>, name: &str, def_node: &Rc<Node>, visi: MethodVisibility) {
        let method = Rc::new(MethodEntry {
            owner: target,
            visibility: Cell::new(visi),
            implementation: Rc::new(MethodImpl::Ruby {
                name: name.to_owned(),
                node: def_node.clone(),
                scope: self.scope.clone(),
            })
        });

        self.env.object.define_method(target, name.to_owned(), method.clone());

        self.env.enqueue_method_for_type_check(method);
    }

    fn symbol_name<'node>(&self, node: &'node Rc<Node>, msg: &str) -> Option<&'node str> {
        match **node {
            Node::Symbol(_, ref sym) => Some(sym),
            Node::Def(_, Id(_, ref sym), _, _) |
            Node::Defs(_, _, Id(_, ref sym), _, _) => {
                self.eval_node(node);
                Some(sym)
            }
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
                self.env.object.define_method(klass, name.to_owned(), Rc::new(MethodEntry {
                    owner: klass,
                    visibility: self.def_visibility.clone(),
                    implementation: Rc::new(MethodImpl::Untyped),
                }));
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
                    let method = MethodEntry {
                        owner: class,
                        visibility: self.def_visibility.clone(),
                        implementation: Rc::new(MethodImpl::AttrReader {
                            ivar: ivar.clone(),
                            node: arg.clone(),
                        }),
                    };
                    self.env.object.define_method(class, sym.to_owned(), Rc::new(method));
                }

                if attr_type.writer() {
                    let method = MethodEntry {
                        owner: class,
                        visibility: self.def_visibility.clone(),
                        implementation: Rc::new(MethodImpl::AttrWriter {
                            ivar: ivar.clone(),
                            node: arg.clone(),
                        }),
                    };
                    self.env.object.define_method(class, sym.to_owned() + "=", Rc::new(method));
                }
            }
        }
    }

    fn lookup_method_for_visi(&self, mid: &str) -> Option<Rc<MethodEntry<'object>>> {
        if let Some(me) = self.env.object.lookup_method(self.scope.module, mid) {
            return Some(me);
        }

        if let RubyObject::Module { .. } = *self.scope.module {
            self.env.object.lookup_method(self.env.object.Object, mid)
        } else {
            None
        }
    }

    fn process_module_function(&self, args: &[Rc<Node>]) {
        if args.is_empty() {
            self.def_visibility.set(MethodVisibility::Private);
            self.module_function.set(true);
        } else {
            for arg in args {
                if let Some(mid) = self.symbol_name(arg, "in method name") {
                    if let Some(method) = self.lookup_method_for_visi(mid) {
                        let target = self.env.object.metaclass(self.scope.module);
                        self.env.object.define_method(target, mid.to_owned(), method.clone())
                    } else {
                        self.error("Could not resolve method in module_function", &[
                            Detail::Loc(&format!("{}#{}", self.scope.module.name(), mid), arg.loc()),
                        ]);
                    }
                }
            }
        }
    }

    fn process_visibility(&self, visi: MethodVisibility, args: &[Rc<Node>]) {
        let self_ = self.scope.module;

        if args.is_empty() {
            self.def_visibility.set(visi);
        } else {
            for arg in args {
                if let Some(mid) = self.symbol_name(arg, "in method name") {
                    if let Some(method) = self.lookup_method_for_visi(mid) {
                        if self_ == method.owner {
                            method.visibility.set(visi);
                        } else {
                            self.env.object.define_method(self_, mid.to_owned(), Rc::new(MethodEntry {
                                owner: self_,
                                visibility: Cell::new(visi),
                                implementation: method.implementation.clone(),
                            }));
                        }
                    }
                }
            }
        }
    }

    fn process_module_inclusion(&self, id: &Id, target: &'object RubyObject<'object>, args: &[Rc<Node>]) {
        if args.is_empty() {
            self.error(&format!("Wrong number of arguments to {}", id.1), &[
                Detail::Loc("here", &id.0),
            ]);
        }

        for arg in args {
            match self.resolve_static(arg) {
                Ok(obj) => {
                    if !self.env.object.include_module(target, &obj) {
                        self.error("Cyclic include", &[
                            Detail::Loc("here", arg.loc()),
                        ])
                    }
                }
                Err((node, message)) => {
                    self.warning(&format!("Could not statically resolve module reference in {}", id.1), &[
                        Detail::Loc(message, node.loc()),
                    ]);
                }
            }
        }
    }

    fn process_require(&self, id: &Id, args: &[Rc<Node>], require_type: RequireType) {
        if args.len() == 0 {
            self.error(&format!("Missing argument to {}", id.1), &[
                Detail::Loc("here", &id.0),
            ]);
            return;
        }

        if args.len() > 1 {
            self.error(&format!("Too many arguments to {}", id.1), &[
                Detail::Loc("from here", args[1].loc()),
            ]);
            return;
        }

        let (loc, string) = match *args[0] {
            Node::String(ref loc, ref string) => (loc, string),
            _ => {
                self.warning("Could not resolve dynamic path in require", &[
                    Detail::Loc("here", args[0].loc()),
                ]);
                return;
            }
        };

        let path = match require_type {
            RequireType::LoadPath => self.env.search_require_path(string),
            RequireType::Relative => self.env.search_relative_path(string, &args[0].loc().file),
        };

        if let Some(path) = path {
            match self.env.require(&path) {
                Ok(()) => {}
                Err(e) => panic!("TODO: implement error handling for require errors: {:?}", e),
            }
        } else {
            self.warning("Could not resolve require", &[
                Detail::Loc("here", loc),
            ]);
        }
    }

    fn process_self_send(&self, id: &Id, args: &[Rc<Node>]) {
        match id.1.as_str() {
            "include" => self.process_module_inclusion(id, self.scope.module, args),
            "extend" => self.process_module_inclusion(id, self.env.object.metaclass(self.scope.module), args),
            "require" => self.process_require(id, args, RequireType::LoadPath),
            "require_relative" => self.process_require(id, args, RequireType::Relative),
            "attr_reader" => self.process_attr(AttrType::Reader, args),
            "attr_writer" => self.process_attr(AttrType::Writer, args),
            "attr_accessor" => self.process_attr(AttrType::Accessor, args),
            "module_function" => self.process_module_function(args),
            "public" => self.process_visibility(MethodVisibility::Public, args),
            "private" => self.process_visibility(MethodVisibility::Private, args),
            "protected" => self.process_visibility(MethodVisibility::Protected, args),
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
            Node::Kwbegin(_, ref nodes) => {
                for node in nodes {
                    self.eval_node(node);
                }
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
            Node::SClass(_, ref expr, ref body) if self.in_def => {
                self.eval_node(expr);
                self.eval_maybe_node(body);
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

                let metaclass = self.env.object.metaclass(singleton);

                self.enter_scope(metaclass, body);
            }
            Node::Def(_, _, ref proto, ref body) if self.in_def => {
                self.eval_maybe_node(proto);
                self.enter_def(body);
            }
            Node::Def(_, Id(_, ref name), ref proto, ref body) => {
                self.eval_maybe_node(proto);

                self.decl_method(&self.scope.module, name, node, self.def_visibility.get());

                if self.module_function.get() {
                    let meta = self.env.object.metaclass(self.scope.module);
                    self.decl_method(meta, name, node, MethodVisibility::Public);
                }

                self.enter_def(body);
            }
            Node::Defs(_, ref singleton, _, ref proto, ref body) if self.in_def => {
                self.eval_node(singleton);
                self.eval_maybe_node(proto);
                self.enter_def(body);
            }
            Node::Defs(_, ref singleton, Id(_, ref name), ref proto, ref body) => {
                match self.resolve_static(singleton) {
                    Ok(&RubyObject::Object { .. }) => {
                        self.error("Defs on Object", &[
                            Detail::Loc("here", singleton.loc()),
                        ]);
                        // panic!("Defs on Object");
                    }
                    Ok(metaclass) => {
                        let metaclass = self.env.object.metaclass(metaclass);
                        self.decl_method(metaclass, name, node, MethodVisibility::Public);
                    }
                    Err((node, message)) => {
                        self.warning(message, &[Detail::Loc("here", node.loc())]);
                    }
                }

                self.eval_maybe_node(proto);
                self.enter_def(body);
            }
            Node::Undef(_, ref names) => {
                // TODO
                let _ = names;
            }
            Node::Send(_, None, ref id, ref args) => {
                if self.in_def {
                    for arg in args {
                        self.eval_node(arg)
                    }
                } else {
                    self.process_self_send(id, args.as_slice());
                }
            }
            Node::CSend(_, ref recv, _, ref args) |
            Node::Send(_, ref recv, _, ref args) => {
                self.eval_maybe_node(recv);
                for arg in args {
                    self.eval_node(arg);
                }
            }
            Node::ConstAsgn(_, ref base, _, ref expr) if self.in_def => {
                self.eval_maybe_node(base);
                self.eval_node(expr);
            }
            Node::ConstAsgn(_, ref base, Id(ref name_loc, ref name), ref expr) => {
                let loc = match *base {
                    Some(ref base_node) => base_node.loc().join(name_loc),
                    None => name_loc.clone(),
                };

                match self.resolve_cbase(base) {
                    Ok(cbase) => {
                        if let Some(constant_entry) = self.env.object.get_own_const(&cbase, name) {
                            self.constant_definition_error("Duplicate constant definition", &loc, &constant_entry.loc);
                            return;
                        }

                        let value = match **expr {
                            Node::Const(..) => self.resolve_cpath(expr),
                            Node::TyCast(_, _, ref tynode) => {
                                Ok(self.env.object.new_typed_object(tynode.clone(), self.scope.clone()))
                            }
                            // TODO special case things like Struct.new and Class.new here
                            _ => {
                                let tynode = Rc::new(Node::TyAny(expr.loc().clone()));
                                Ok(self.env.object.new_typed_object(tynode, self.scope.clone()))
                            }
                        };

                        match value {
                            Ok(value) => {
                                let constant = Rc::new(ConstantEntry {
                                    loc: Some(loc),
                                    value: value,
                                });

                                self.env.object.set_const(&cbase, name, constant);
                            }
                            Err((node, message)) => {
                                self.warning("Could not statically resolve expression in constant assignment", &[
                                    Detail::Loc(message, node.loc()),
                                ]);
                            }
                        }
                    }
                    Err((node, message)) => {
                        self.warning("Could not statically resolve constant in assignment", &[
                            Detail::Loc(message, node.loc()),
                        ]);
                    }
                }
            }
            Node::Alias(_, _, _) if self.in_def => {
                // pass
            }
            Node::Alias(_, ref to, ref from) => {
                self.alias_method(self.scope.module, from, to);
            }
            Node::TyIvardecl(..) if self.in_def => {
                self.error("Invalid instance variable type declaration", &[
                    Detail::Loc("here", node.loc()),
                ]);
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
                self.eval_maybe_node(block_args);
                self.eval_maybe_node(block_body);
            }
            Node::Const(..) => {
                // try to autoload this const, but ignore any errors
                let _ = self.resolve_cpath(node);
            }
            Node::ConstLhs(_, ref base, _) => {
                if let Some(ref base) = *base {
                    // try to autoload this const, but ignore any errors
                    let _ = self.resolve_cpath(base);
                }
            }
            Node::Args(_, ref args) => {
                for arg in args {
                    self.eval_node(arg);
                }
            }
            Node::Prototype(_, ref genargs, ref args, ref retn) => {
                self.eval_maybe_node(genargs);
                self.eval_maybe_node(args);
                self.eval_maybe_node(retn);
            }
            Node::TypedArg(_, ref ty, ref arg) => {
                self.eval_node(ty);
                self.eval_node(arg);
            }
            Node::Procarg0(_, ref arg) => {
                self.eval_node(arg);
            }
            Node::Arg(..) => {}
            Node::Restarg(..) => {}
            Node::Kwrestarg(..) => {}
            Node::Blockarg(..) => {}
            Node::Kwarg(..) => {}
            Node::Optarg(_, _, ref expr) |
            Node::Kwoptarg(_, _, ref expr) => {
                self.eval_node(expr);
            }
            Node::If(_, ref cond, ref then, ref else_) => {
                self.eval_node(cond);
                self.eval_maybe_node(then);
                self.eval_maybe_node(else_);
            }
            Node::While(_, ref cond, ref body) |
            Node::Until(_, ref cond, ref body) => {
                self.eval_node(cond);
                self.eval_maybe_node(body);
            }
            Node::WhilePost(_, ref body, ref cond) |
            Node::UntilPost(_, ref body, ref cond) => {
                self.eval_node(cond);
                self.eval_node(body);
            }
            Node::For(_, ref lval, ref expr, ref body) => {
                self.eval_node(lval);
                self.eval_node(expr);
                self.eval_maybe_node(body);
            }
            Node::Backref(..) |
            Node::Cvar(..) |
            Node::Defined(..) |
            Node::False(..) |
            Node::FileLiteral(..) |
            Node::Float(..) |
            Node::Gvar(..) |
            Node::Integer(..) |
            Node::Ivar(..) |
            Node::Lambda(..) |
            Node::LineLiteral(..) |
            Node::Lvar(..) |
            Node::Nil(..) |
            Node::NthRef(..) |
            Node::Regexp(..) |
            Node::Retry(..) |
            Node::Self_(..) |
            Node::String(..) |
            Node::Symbol(..) |
            Node::True(..) |
            Node::ZSuper(..) => {}
            Node::Splat(_, ref expr) => {
                self.eval_maybe_node(expr);
            }
            Node::Kwsplat(_, ref expr) => {
                self.eval_node(expr);
            }
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
            Node::Ensure(_, ref body, ref ensure) => {
                self.eval_maybe_node(body);
                self.eval_maybe_node(ensure);
            }
            Node::Array(_, ref exprs) |
            Node::Break(_, ref exprs) |
            Node::DString(_, ref exprs) |
            Node::DSymbol(_, ref exprs) |
            Node::Hash(_, ref exprs) |
            Node::Mlhs(_, ref exprs) |
            Node::Next(_, ref exprs) |
            Node::Return(_, ref exprs) |
            Node::Super(_, ref exprs) |
            Node::XString(_, ref exprs) |
            Node::Yield(_, ref exprs) => {
                for expr in exprs {
                    self.eval_node(expr);
                }
            }
            Node::Or(_, ref left, ref right) |
            Node::And(_, ref left, ref right) => {
                self.eval_node(left);
                self.eval_node(right);
            }
            Node::LvarLhs(_, _) |
            Node::IvarLhs(_, _) |
            Node::CvarLhs(_, _) |
            Node::GvarLhs(_, _) => {}
            Node::LvarAsgn(_, _, ref expr) |
            Node::IvarAsgn(_, _, ref expr) |
            Node::CvarAsgn(_, _, ref expr) |
            Node::GvarAsgn(_, _, ref expr) => {
                self.eval_node(expr);
            }
            Node::Pair(_, ref key, ref value) => {
                self.eval_node(key);
                self.eval_node(value);
            }
            Node::IRange(_, ref a, ref b) |
            Node::ERange(_, ref a, ref b) => {
                self.eval_node(a);
                self.eval_node(b);
            }
            Node::BlockPass(_, ref expr) => {
                self.eval_node(expr);
            }
            Node::OrAsgn(_, ref lhs, ref rhs) |
            Node::AndAsgn(_, ref lhs, ref rhs) |
            Node::OpAsgn(_, ref lhs, _, ref rhs) |
            Node::Masgn(_, ref lhs, ref rhs) |
            Node::MatchAsgn(_, ref lhs, ref rhs) => {
                self.eval_node(lhs);
                self.eval_node(rhs);
            }
            Node::Case(_, ref cond, ref cases, ref else_) => {
                self.eval_maybe_node(cond);

                for case in cases {
                    self.eval_node(case);
                }

                self.eval_maybe_node(else_);
            }
            Node::When(_, ref conds, ref expr) => {
                for cond in conds {
                    self.eval_node(cond);
                }

                self.eval_maybe_node(expr);
            }
            Node::TyCast(_, ref expr, ref ty) => {
                self.eval_node(expr);
                self.eval_node(ty);
            }
            Node::TyCpath(_, ref cpath) => {
                self.eval_node(cpath);
            }
            Node::TyGeninst(_, ref cpath, ref args) => {
                self.eval_node(cpath);
                for arg in args {
                    self.eval_node(arg);
                }
            }
            Node::TyArray(_, ref ty) |
            Node::TyNillable(_, ref ty) => {
                self.eval_node(ty);
            }
            Node::TyTuple(_, ref tys) => {
                for ty in tys {
                    self.eval_node(ty);
                }
            }
            Node::TyHash(_, ref key, ref value) => {
                self.eval_node(key);
                self.eval_node(value);
            }
            Node::TyNil(..) |
            Node::TySelf(..) |
            Node::TyClass(..) |
            Node::TyInstance(..) |
            Node::TyAny(..) => {}
            Node::TyGenargs(_, ref genargs) => {
                for genarg in genargs {
                    self.eval_node(genarg);
                }
            }
            Node::TyGendeclarg(_, _, ref constraint) => {
                self.eval_maybe_node(constraint);
            }
            Node::TyConUnify(_, ref a, ref b) |
            Node::TyConSubtype(_, ref a, ref b) |
            Node::TyOr(_, ref a, ref b) => {
                self.eval_node(a);
                self.eval_node(b);
            }
            Node::TyProc(_, ref proto) => {
                self.eval_node(proto);
            }
            _ => panic!("unknown node: {:?}", node),
        }
    }
}

fn source_type_for_file(source_file: &SourceFile) -> SourceType {
    let is_typedruby = source_file.source()
        .lines()
        .take_while(|line| line.starts_with("#"))
        .any(|line| line.contains("@typedruby"));

    if is_typedruby {
        SourceType::TypedRuby
    } else {
        SourceType::Ruby
    }
}

pub fn evaluate<'env, 'object: 'env>(env: &'env Environment<'object>, node: Rc<Node>) {
    let scope = Rc::new(Scope { parent: None, module: env.object.Object });

    let source_file = node.loc().file.clone();

    let source_type = source_type_for_file(&source_file);

    Eval::new(env, scope, source_file, source_type, false).eval_node(&node);
}
