use ast::{Id, Node, Loc, SourceFile};
use environment::Environment;
use errors::Detail;
use define::{Definitions, MethodVisibility, MethodDef, IvarDef};
use object::{RubyObject, Scope, ConstantEntry, IncludeError};
use std::rc::Rc;
use std::cell::Cell;
use abstract_type::{TypeNode, TypeScope, TypeNodeRef};

type EvalResult<'a, T> = Result<T, (&'a Node, &'static str)>;

struct Eval<'env, 'object: 'env> {
    pub env: &'env Environment<'object>,
    pub scope: Rc<Scope<'object>>,
    source_file: Rc<SourceFile>,
    source_type: SourceType,
    in_def: bool,
    def_visibility: Cell<MethodVisibility>,
    module_function: Cell<bool>,
    defs: &'env Definitions<'object>,
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

struct DeclRef<'object> {
    base: &'object RubyObject<'object>,
    name: String,
    type_parameters: Vec<Id>,
}

enum RequireType {
    LoadPath,
    Relative,
}

impl<'env, 'object> Eval<'env, 'object> {
    fn new(
        env: &'env Environment<'object>,
        scope: Rc<Scope<'object>>,
        source_file: Rc<SourceFile>,
        source_type: SourceType,
        in_def: bool,
    ) -> Eval<'env, 'object> {
        Eval {
            env: env,
            scope: scope,
            source_file: source_file,
            source_type: source_type,
            in_def: in_def,
            def_visibility: Cell::new(MethodVisibility::Public),
            module_function: Cell::new(false),
            defs: &env.defs,
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

    fn resolve_cpath<'a>(&self, node: &'a Node) -> EvalResult<'a, Rc<ConstantEntry<'object>>> {
        self.env.resolve_cpath(node, self.scope.clone())
    }

    fn resolve_cbase<'a>(&self, cbase: &'a Option<Rc<Node>>) -> EvalResult<'a, &'object RubyObject<'object>> {
        match *cbase {
            None => Ok(self.scope.module),
            Some(ref cbase_node) => self.env.resolve_cbase(cbase_node, self.scope.clone()),
        }
    }

    fn resolve_decl_ref<'a>(&self, node: &'a Node) -> EvalResult<'a, DeclRef<'object>> {
        let (name, params) = match *node {
            Node::TyGendecl(_, ref name, ref params, ref constraints) => {
                let params = params.iter().map(|param|
                    match **param {
                        Node::TyGendeclarg(_, ref name, None) =>
                            name.clone(),
                        Node::TyGendeclarg(_, ref name, Some(ref constraint)) => {
                            self.error("Type constraints not permitted on classes/modules", &[
                                Detail::Loc("here", constraint.loc()),
                            ]);
                            name.clone()
                        },
                        _ => panic!("expected TyGendeclarg in TyGendecl"),
                    }
                ).collect();

                if !constraints.is_empty() {
                    self.error("Type constraints not permitted on classes/modules", &[
                        Detail::Loc("here", constraints[0].loc()),
                    ]);
                }

                (Rc::as_ref(name), params)
            }
            _ => {
                (node, vec![])
            }
        };

        let ref_ = if let Node::Const(_, ref base, ref id) = *name {
            match *base {
                Some(ref base_node) =>
                    self.resolve_cpath(base_node).and_then(|constant|
                        constant.module()
                            .map(|constant| (constant, id))
                            .ok_or((base_node, "Not a static class/module"))),
                None => Ok((self.scope.module, id)),
            }
        } else {
            Err((name, "Class name is not a static constant"))
        };

        ref_.map(|(base, name)| {
            DeclRef { base, name: name.1.clone(), type_parameters: params }
        })
    }

    fn resolve_static<'a>(&self, node: &'a Node) -> EvalResult<'a, &'object RubyObject<'object>> {
        match *node {
            Node::Self_(_) => Ok(self.scope.module),
            Node::Const(..) =>
                self.resolve_cpath(node).and_then(|constant|
                    constant.module()
                        .ok_or((node, "Not a static class/module"))),
            Node::TyConstInstance(..) =>
                Err((node, "Generic instance not valid here")),
            _ =>
                Err((node, "unknown static node")),
        }
    }

    fn enter_scope(&self, constant: &'object RubyObject<'object>, body: &Option<Rc<Node>>) {
        if let Some(ref node) = *body {
            let eval = Eval::new(
                self.env,
                Scope::spawn(&self.scope, constant),
                self.source_file.clone(),
                self.source_type,
                self.in_def,
            );

            eval.eval_node(node)
        }
    }

    fn enter_def(&self, body: &Option<Rc<Node>>) {
        if let Some(ref node) = *body {
            let eval = Eval::new(
                self.env,
                self.scope.clone(),
                self.source_file.clone(),
                self.source_type,
                true,
            );

            eval.eval_node(node)
        }
    }

    fn constant_definition_error(&self, message: &str, loc: &Loc, definition: Option<&Loc>) {
        let mut details = vec![
            Detail::Loc("here", loc),
        ];

        if let Some(ref loc) = definition {
            details.push(Detail::Loc("previously defined here", loc));
        }

        self.error(message, details.as_slice());
    }

    fn decl_class(&self, name: &Node, superclass: &Option<Rc<Node>>, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let superclass = superclass.as_ref().and_then(|node| {
            match self.resolve_static(node) {
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
            Ok(decl) => {
                if let Some(constant_entry) = self.env.object.get_const_for_definition(decl.base, &decl.name) {
                    match *constant_entry {
                        ConstantEntry::Expression { ref loc, .. } => {
                            self.constant_definition_error(&format!("{} is not a static class", decl.name), name.loc(), Some(loc));

                            // do nothing with the class body
                            None
                        }
                        ConstantEntry::Module { ref loc, value: &RubyObject::Module { .. } } => {
                            self.constant_definition_error(&format!("{} is not a static class", decl.name), name.loc(), loc.as_ref());

                            // do nothing with the class body
                            None
                        }
                        ConstantEntry::Module { value: value@&RubyObject::Class { .. }, .. } |
                        ConstantEntry::Module { value: value@&RubyObject::Metaclass { .. }, .. } => {
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

                            Some(value)
                        }
                        ConstantEntry::Module { value: &RubyObject::IClass { .. }, .. } =>
                            panic!(),
                    }
                } else {
                    let superclass = match superclass {
                        Some((_, ref superclass)) => superclass,
                        None => &self.env.object.Object,
                    };

                    let type_parameters =
                        if superclass.type_parameters().is_empty() {
                            decl.type_parameters
                        } else if decl.type_parameters.is_empty() {
                            superclass.type_parameters().to_vec()
                        } else {
                            let loc = decl.type_parameters.first().unwrap().0.join(
                                        &decl.type_parameters.last().unwrap().0);

                            self.error("Subclasses of generic classes may not specify type parameters", &[
                                Detail::Loc("here", &loc),
                            ]);

                            superclass.type_parameters().to_vec()
                        };

                    let class = self.env.object.new_class(
                        self.env.object.constant_path(decl.base, &decl.name),
                        superclass, type_parameters);

                    let constant = Rc::new(ConstantEntry::Module {
                        loc: Some(name.loc().clone()),
                        value: class,
                    });

                    self.env.object.set_const(decl.base, &decl.name, constant)
                        .expect("constant to not already exist");

                    Some(class)
                }
            }
            Err((node, message)) => {
                self.warning(&message, &[Detail::Loc("here", node.loc())]);
                return;
            }
        };

        if let Some(class) = class {
            self.enter_scope(class, body);
        }
    }

    fn decl_module(&self, name: &Node, body: &Option<Rc<Node>>) {
        // TODO need to autoload

        let module = match self.resolve_decl_ref(name) {
            Ok(decl) => {
                if let Some(constant_entry) = self.env.object.get_const_for_definition(decl.base, &decl.name) {
                    match *constant_entry {
                        ConstantEntry::Expression { ref loc, .. } => {
                            self.constant_definition_error(&format!("{} is not a static module", decl.name), name.loc(), Some(loc));
                            None
                        }
                        ConstantEntry::Module { value: value@&RubyObject::Module { .. }, .. } => {
                            Some(value)
                        }
                        ConstantEntry::Module { value: value@&RubyObject::Class { .. }, loc: ref expr_loc } |
                        ConstantEntry::Module { value: value@&RubyObject::Metaclass { .. }, loc: ref expr_loc } => {
                            self.constant_definition_error(&format!("{} is not a module", decl.name), name.loc(), expr_loc.as_ref());
                            Some(value)
                        }
                        ConstantEntry::Module { value: &RubyObject::IClass { .. }, .. } =>
                            panic!(),
                    }
                } else {
                    let module = self.env.object.new_module(
                        self.env.object.constant_path(decl.base, &decl.name), decl.type_parameters);

                    let constant = Rc::new(ConstantEntry::Module {
                        loc: Some(name.loc().clone()),
                        value: module,
                    });

                    self.env.object.set_const(decl.base, &decl.name, constant)
                        .expect("constant to not already exist");

                    Some(module)
                }
            }
            Err((node, msg)) => {
                self.error(msg, &[Detail::Loc("here", node.loc())]);
                return
            }
        };

        if let Some(module) = module {
            self.enter_scope(module, body);
        }
    }

    fn decl_method(&self, target: &'object RubyObject<'object>, name: Id, def_node: &Rc<Node>, visi: MethodVisibility) {
        self.defs.add_method(MethodDef::Def {
            module: target,
            visi: visi,
            name: name,
            node: def_node.clone(),
            scope: self.scope.clone(),
        });
    }

    fn symbol_name<'node>(&self, node: &'node Rc<Node>, msg: &str) -> Option<Id> {
        match **node {
            Node::Symbol(ref loc, ref sym) => Some(Id(loc.clone(), sym.clone())),
            Node::Def(_, ref id, _, _) |
            Node::Defs(_, _, ref id, _, _) => {
                self.eval_node(node);
                Some(id.clone())
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

        if let (Some(from), Some(to)) = (from_name, to_name) {
            self.defs.add_method(MethodDef::Alias {
                module: klass,
                from: from,
                to: to,
                emit_error: self.emit_errors(),
            });
        }
    }

    fn process_attr(&self, attr_type: AttrType, args: &[Rc<Node>]) {
        // TODO need to decouple self from the current module in scope so we
        // can ignore errant attr_* calls at the top level.

        let class = self.scope.module;

        for arg in args {
            if let Some(sym) = self.symbol_name(arg, "in attribute name") {
                if attr_type.reader() {
                    self.defs.add_method(MethodDef::AttrReader {
                        module: class,
                        visi: self.def_visibility.get(),
                        name: sym.clone(),
                    });
                }

                if attr_type.writer() {
                    self.defs.add_method(MethodDef::AttrWriter {
                        module: class,
                        visi: self.def_visibility.get(),
                        name: sym.clone(),
                    });
                }
            }
        }
    }

    fn process_module_function(&self, args: &[Rc<Node>]) {
        if args.is_empty() {
            self.def_visibility.set(MethodVisibility::Private);
            self.module_function.set(true);
        } else {
            for arg in args {
                if let Some(mid) = self.symbol_name(arg, "in method name") {
                    self.defs.add_method(MethodDef::ModuleFunc {
                        module: self.scope.module,
                        name: mid,
                        emit_error: self.emit_errors(),
                    });
                }
            }
        }
    }

    fn process_visibility(&self, visi: MethodVisibility, args: &[Rc<Node>]) {
        if args.is_empty() {
            self.def_visibility.set(visi);
        } else {
            for arg in args {
                if let Some(mid) = self.symbol_name(arg, "in method name") {
                    self.defs.add_method(MethodDef::SetVisi {
                        module: self.scope.module,
                        visi: visi,
                        name: mid,
                        emit_error: self.emit_errors(),
                    });
                }
            }
        }
    }

    fn resolve_module_ref<'a>(&self, node: &'a Node)
        -> EvalResult<'a, (&'object RubyObject<'object>, Vec<TypeNodeRef<'object>>)>
    {
        let (node, params) = match *node {
            Node::TyConstInstance(_, ref cpath, ref params) =>
                (cpath.as_ref(), params.as_slice()),
            _ =>
                (node, &[] as &[Rc<Node>]),
        };

        self.resolve_static(node).map(|obj| {
            let sc = TypeScope::new(self.scope.clone(), self.scope.module);

            (obj, params.iter().map(|param|
                TypeNode::resolve(param, &self.env, sc.clone())).collect())
        })
    }

    fn process_module_inclusion(&self, id: &Id, target: &'object RubyObject<'object>, args: &[Rc<Node>]) {
        if args.is_empty() {
            self.error(&format!("Wrong number of arguments to {}", id.1), &[
                Detail::Loc("here", &id.0),
            ]);
        }

        for arg in args {
            match self.resolve_module_ref(arg) {
                Ok((obj, mut params)) => {
                    if obj.type_parameters().len() != params.len() {
                        let any = Rc::new(TypeNode::Any { loc: arg.loc().clone() });

                        if params.is_empty() {
                            // allow untyped inclusions of parameterised modules
                            // for legacy compatibility
                        } else if obj.type_parameters().len() > params.len() {
                            self.error("Too few type parameters supplied in instantiation of generic type", &[
                                Detail::Loc("here", params.last().unwrap().loc())
                            ]);
                        } else if obj.type_parameters().len() < params.len() {
                            self.error("Too many type parameters supplied in instantiation of generic type", &[
                                Detail::Loc("here", params.last().unwrap().loc())
                            ]);
                        }

                        params.resize(obj.type_parameters().len(), any);
                    }

                    match self.env.object.include_module(target, obj, params, Some(arg.loc().clone())) {
                        Ok(()) => (),
                        Err(IncludeError::CyclicInclude) => {
                            self.error("Cyclic include", &[
                                Detail::Loc("here", arg.loc()),
                            ])
                        }
                        Err(IncludeError::DuplicateInclude(Some(loc))) => {
                            self.error("Duplicate include", &[
                                Detail::Loc("here", arg.loc()),
                                Detail::Loc("previous inclusion was here", loc),
                            ])
                        }
                        Err(IncludeError::DuplicateInclude(None)) => {
                            self.error("Duplicate include", &[
                                Detail::Loc("here", arg.loc()),
                            ])
                        }
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

        if let Some(pathstr) = string.string() {
            let path = match require_type {
                RequireType::LoadPath => self.env.search_require_path(&pathstr),
                RequireType::Relative => self.env.search_relative_path(&pathstr, &args[0].loc().file()),
            };

            if let Some(path) = path {
                self.env.require(&path).unwrap_or_else(|e|
                    self.error(&format!("Could not load file: {}", e), &[
                        Detail::Loc("in require", args[0].loc()),
                    ]))
            } else {
                self.warning("Could not resolve require", &[
                    Detail::Loc("here", loc),
                ]);
            }
        } else {
            self.error("Invalid UTF-8 in require path", &[
                Detail::Loc("here", args[0].loc()),
            ]);
            return
        };
    }

    fn process_alias_method(&self, id: &Id, args: &[Rc<Node>]) {
        if args.len() != 2 {
            self.error(&format!("Wrong number of arguments to {}", id.1), &[
                Detail::Loc("here", &id.0),
            ]);
            return;
        }

        self.alias_method(self.scope.module, &args[1], &args[0]);
    }

    fn process_self_send(&self, id: &Id, args: &[Rc<Node>]) {
        match id.1.as_str() {
            "include" => self.process_module_inclusion(id, self.scope.module, args),
            "extend" => self.process_module_inclusion(id, self.env.object.metaclass(self.scope.module), args),
            "require" => self.process_require(id, args, RequireType::LoadPath),
            // TODO guard require_dependency behind a rails-mode flag:
            "require_dependency" => self.process_require(id, args, RequireType::LoadPath),
            "require_relative" => self.process_require(id, args, RequireType::Relative),
            "alias_method" => self.process_alias_method(id, args),
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
                self.decl_class(declname, superclass, body);
            }
            Node::Module(_, ref name, ref body) => {
                self.decl_module(name, body);
            }
            Node::SClass(_, ref expr, ref body) if self.in_def => {
                self.eval_node(expr);
                self.eval_maybe_node(body);
            }
            Node::SClass(_, ref expr, ref body) => {
                match self.resolve_static(expr) {
                    Ok(singleton) => {
                        let metaclass = self.env.object.metaclass(singleton);
                        self.enter_scope(metaclass, body);
                    }
                    Err((node, message)) => {
                        self.warning("Could not statically resolve singleton expression", &[
                            Detail::Loc(message, node.loc()),
                        ]);
                        return;
                    }
                };
            }
            Node::Def(_, _, ref proto, ref body) if self.in_def => {
                self.eval_maybe_node(proto);
                self.enter_def(body);
            }
            Node::Def(_, ref name, ref proto, ref body) => {
                self.eval_maybe_node(proto);

                self.decl_method(&self.scope.module, name.clone(), node, self.def_visibility.get());

                if self.module_function.get() {
                    let meta = self.env.object.metaclass(self.scope.module);
                    self.decl_method(meta, name.clone(), node, MethodVisibility::Public);
                }

                self.enter_def(body);
            }
            Node::Defs(_, ref singleton, _, ref proto, ref body) if self.in_def => {
                self.eval_node(singleton);
                self.eval_maybe_node(proto);
                self.enter_def(body);
            }
            Node::Defs(_, ref singleton, ref name, ref proto, ref body) => {
                match self.resolve_static(singleton) {
                    Ok(metaclass) => {
                        let metaclass = self.env.object.metaclass(metaclass);
                        self.decl_method(metaclass, name.clone(), node, MethodVisibility::Public);
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
                            let existing_loc = match *constant_entry {
                                ConstantEntry::Expression { ref loc, .. } => Some(loc),
                                ConstantEntry::Module { ref loc, .. } => loc.as_ref(),
                            };

                            self.constant_definition_error("Duplicate constant definition", &loc, existing_loc);
                            return;
                        }

                        let constant = match **expr {
                            Node::Const(..) => {
                                self.resolve_cpath(expr).map(|constant| match *constant {
                                    ConstantEntry::Expression { ref ty, scope_self, .. } =>
                                        ConstantEntry::Expression { ty: ty.clone(), scope_self, loc },
                                    ConstantEntry::Module { value, .. } =>
                                        ConstantEntry::Module { value: value, loc: Some(loc) },
                                })
                            },
                            Node::TyCast(_, _, ref type_node) => {
                                let scope = TypeScope::new(self.scope.clone(),
                                    self.env.object.metaclass(self.scope.module));

                                let ty = TypeNode::resolve(type_node, self.env, scope);

                                Ok(ConstantEntry::Expression {
                                    loc: loc,
                                    ty: ty,
                                    scope_self: self.scope.module,
                                })
                            }
                            // TODO special case things like Struct.new and Class.new here
                            _ => {
                                Ok(ConstantEntry::Expression {
                                    loc: loc.clone(),
                                    ty: Rc::new(TypeNode::Any { loc: loc.clone() }),
                                    scope_self: self.scope.module,
                                })
                            }
                        };

                        match constant {
                            Ok(constant) => {
                                self.env.object.set_const(&cbase, name, Rc::new(constant))
                                    .expect("constant to not already exist");
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
            Node::TyIvardecl(_, ref ivar, ref type_node) => {
                self.defs.add_ivar(IvarDef {
                    module: self.scope.module,
                    name: ivar.to_owned(),
                    type_node: type_node.clone(),
                    scope: self.scope.clone(),
                });
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
            Node::Redo(..) |
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
            Node::TyGenargs(_, ref vars, ref constraints) => {
                for var in vars {
                    self.eval_node(var);
                }

                for constraint in constraints {
                    self.eval_node(constraint);
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
    let source_file = node.loc().file().clone();
    let source_type = source_type_for_file(&source_file);

    Eval::new(env, scope, source_file, source_type, false).eval_node(&node);
}
