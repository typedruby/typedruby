use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use typed_arena::Arena;
use ast::{Node, Loc, Id};
use define::MethodVisibility;
use abstract_type::{TypeNodeRef, Prototype};

// can become NonZero<u64> once NonZero for non-pointer types hits stable:
type ObjectId = u64;

struct GenId {
    _next: Cell<ObjectId>,
}

impl GenId {
    fn new() -> GenId {
        GenId { _next: Cell::new(1) }
    }

    fn next(&self) -> ObjectId {
        let next = self._next.get();
        self._next.set(next + 1);
        next
    }
}

pub struct AncestorIterator<'a> {
    object: Option<&'a RubyObject<'a>>,
}

impl<'a> Iterator for AncestorIterator<'a> {
    type Item = &'a RubyObject<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.object {
            None => return None,
            Some(&RubyObject::Module { ref superclass, .. }) |
            Some(&RubyObject::Class { ref superclass, .. }) |
            Some(&RubyObject::Metaclass { ref superclass, .. }) |
            Some(&RubyObject::IClass { ref superclass, .. }) => {
                let cur = self.object;

                self.object = match superclass.get() {
                    None => None,
                    Some(ref c) => Some(c),
                };

                cur
            }
        }
    }
}

type ClassTable<'a, T> = RefCell<HashMap<&'a RubyObject<'a>, HashMap<String, Rc<T>>>>;

#[allow(non_snake_case)]
pub struct ObjectGraph<'a> {
    ids: GenId,
    arena: &'a Arena<RubyObject<'a>>,

    pub BasicObject: &'a RubyObject<'a>,
    pub Object: &'a RubyObject<'a>,
    pub Module: &'a RubyObject<'a>,
    pub Class: &'a RubyObject<'a>,
    pub Kernel: &'a RubyObject<'a>,
    pub Boolean: &'a RubyObject<'a>,
    pub TrueClass: &'a RubyObject<'a>,
    pub FalseClass: &'a RubyObject<'a>,
    pub NilClass: &'a RubyObject<'a>,
    pub Symbol: &'a RubyObject<'a>,
    pub String: &'a RubyObject<'a>,
    pub Numeric: &'a RubyObject<'a>,
    pub Integer: &'a RubyObject<'a>,
    pub Float: &'a RubyObject<'a>,
    pub Regexp: &'a RubyObject<'a>,
    pub Proc: &'a RubyObject<'a>,
    pub Exception: &'a RubyObject<'a>,
    pub StandardError: &'a RubyObject<'a>,

    constants: ClassTable<'a, ConstantEntry<'a>>,
    methods: ClassTable<'a, MethodEntry<'a>>,
    ivars: ClassTable<'a, IvarEntry<'a>>,
}

impl<'a> ObjectGraph<'a> {
    fn new_object_id(&self) -> ObjectId {
        self.ids.next()
    }

    pub fn new(arena: &'a Arena<RubyObject<'a>>) -> ObjectGraph<'a> {
        // manually bootstrap cyclic core of object graph:

        let unsafe_null_ref = unsafe { &*::std::ptr::null() };

        let ids = GenId::new();

        let basic_object = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "BasicObject".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(None),
            type_parameters: Vec::new(),
        });

        let object = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Object".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(basic_object)),
            type_parameters: Vec::new(),
        });

        let module = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Module".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(object)),
            type_parameters: Vec::new(),
        });

        let class = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Class".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(module)),
            type_parameters: Vec::new(),
        });

        let basic_object_metaclass = arena.alloc(RubyObject::Metaclass {
            id: ids.next(),
            of: basic_object,
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(class)),
        });

        fn set_class<'a>(object: &'a RubyObject<'a>, class: &'a RubyObject<'a>) {
            match *object {
                RubyObject::Class { class: ref class_, .. } |
                RubyObject::Module { class: ref class_, .. } |
                RubyObject::Metaclass { class: ref class_, .. } => class_.set(class),
                RubyObject::IClass { .. } => panic!(),
            }
        }

        set_class(basic_object, basic_object_metaclass);
        set_class(basic_object_metaclass, class);
        set_class(object, class);
        set_class(module, class);
        set_class(class, class);

        let mut o = ObjectGraph {
            ids: ids,
            arena: arena,

            BasicObject: basic_object,
            Object: object,
            Module: module,
            Class: class,
            // temporary values, will overwrite before returning:
            Kernel: object,
            Boolean: object,
            TrueClass: object,
            FalseClass: object,
            NilClass: object,
            Symbol: object,
            String: object,
            Numeric: object,
            Integer: object,
            Float: object,
            Regexp: object,
            Proc: object,
            Exception: object,
            StandardError: object,

            constants: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
            ivars: RefCell::new(HashMap::new()),
        };

        o.set_const(o.BasicObject, "BasicObject", Rc::new(ConstantEntry::Module { loc: None, value: o.BasicObject })).unwrap();
        o.set_const(o.Object, "Object", Rc::new(ConstantEntry::Module { loc: None, value: o.Object })).unwrap();
        o.set_const(o.Object, "Module", Rc::new(ConstantEntry::Module { loc: None, value: o.Module })).unwrap();
        o.set_const(o.Object, "Class", Rc::new(ConstantEntry::Module { loc: None, value: o.Class })).unwrap();

        o.Kernel = o.define_module(None, o.Object, "Kernel", vec![]);
        o.Boolean = o.define_class(None, o.Object, "Boolean", o.Object, Vec::new());
        o.TrueClass = o.define_class(None, o.Object, "TrueClass", o.Boolean, Vec::new());
        o.FalseClass = o.define_class(None, o.Object, "FalseClass", o.Boolean, Vec::new());
        o.NilClass = o.define_class(None, o.Object, "NilClass", o.Object, Vec::new());
        o.Symbol = o.define_class(None, o.Object, "Symbol", o.Object, Vec::new());
        o.String = o.define_class(None, o.Object, "String", o.Object, Vec::new());
        o.Numeric = o.define_class(None, o.Object, "Numeric", o.Object, Vec::new());
        o.Integer = o.define_class(None, o.Object, "Integer", o.Numeric, Vec::new());
        o.Float = o.define_class(None, o.Object, "Float", o.Numeric, Vec::new());
        o.Regexp = o.define_class(None, o.Object, "Regexp", o.Object, Vec::new());
        o.Proc = o.define_class(None, o.Object, "Proc", o.Object, Vec::new());
        o.Exception = o.define_class(None, o.Object, "Exception", o.Object, Vec::new());
        o.StandardError = o.define_class(None, o.Object, "StandardError", o.Exception, Vec::new());

        for (class, mid, impl_) in [
            (o.Class,  "new",   Rc::new(MethodImpl::IntrinsicClassNew)),
            (o.Proc,   "call",  Rc::new(MethodImpl::IntrinsicProcCall)),
            (o.Kernel, "raise", Rc::new(MethodImpl::IntrinsicKernelRaise)),
            (o.Kernel, "is_a?", Rc::new(MethodImpl::IntrinsicKernelIsA)),
            (o.Kernel, "reveal_type", Rc::new(MethodImpl::IntrinsicRevealType)),
        ].iter().cloned() {
            o.define_method(class, mid.to_owned(), Rc::new(MethodEntry {
                owner: class,
                visibility: Cell::new(MethodVisibility::Public),
                implementation: impl_,
            }))
        }

        o
    }

    fn expect_class(&self, name: &str) -> &'a RubyObject<'a> {
        self.get_const(self.Object, name).unwrap().expect_module()
    }

    pub fn array_class(&self) -> &'a RubyObject<'a> {
        self.expect_class("Array")
    }

    pub fn hash_class(&self) -> &'a RubyObject<'a> {
        self.expect_class("Hash")
    }

    pub fn range_class(&self) -> &'a RubyObject<'a> {
        self.expect_class("Range")
    }

    fn alloc(&self, obj: RubyObject<'a>) -> &'a RubyObject<'a> {
        self.arena.alloc(obj)
    }

    fn class_table_lookup<T>(table: &ClassTable<'a, T>, class: &'a RubyObject<'a>, key: &str) -> Option<Rc<T>> {
        match *class {
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {}
            RubyObject::IClass {..} => panic!("RubyObject::IClass has no associated class table"),
        }

        let table_ref = table.borrow();

        table_ref.get(class)
            .and_then(|t| t.get(key))
            .map(|rc| rc.clone())
    }

    fn class_table_insert<T>(table: &ClassTable<'a, T>, class: &'a RubyObject<'a>, key: String, value: Rc<T>) {
        match *class {
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {}
            RubyObject::IClass {..} => panic!("RubyObject::IClass has no associated class table"),
        }

        let mut table_ref = table.borrow_mut();

        table_ref.entry(class).or_insert_with(|| HashMap::new())
            .insert(key, value);
    }

    pub fn new_class(&self, name: String, superclass: &'a RubyObject<'a>, type_parameters: Vec<Id>) -> &'a RubyObject<'a> {
        self.alloc(RubyObject::Class {
            id: self.new_object_id(),
            name: name,
            class: Cell::new(self.Class),
            superclass: Cell::new(Some(superclass)),
            type_parameters: type_parameters,
        })
    }

    pub fn new_module(&self, name: String, type_parameters: Vec<Id>) -> &'a RubyObject<'a> {
        self.alloc(RubyObject::Module {
            id: self.new_object_id(),
            name: name,
            class: Cell::new(self.Module),
            superclass: Cell::new(None),
            type_parameters: type_parameters,
        })
    }

    pub fn define_class(&self, loc: Option<Loc>, owner: &'a RubyObject<'a>, name: &str, superclass: &'a RubyObject<'a>, type_parameters: Vec<Id>) -> &'a RubyObject<'a> {
        let class = self.new_class(self.constant_path(owner, name), superclass, type_parameters);

        self.set_const(owner, name, Rc::new(ConstantEntry::Module { loc: loc, value: class }))
            .expect("class to not already exist");

        class
    }

    pub fn define_module(&self, loc: Option<Loc>, owner: &'a RubyObject<'a>, name: &str, type_parameters: Vec<Id>) -> &'a RubyObject<'a> {
        let module = self.new_module(self.constant_path(owner, name), type_parameters);

        self.set_const(owner, name, Rc::new(ConstantEntry::Module { loc: loc, value: module }))
            .expect("module to not already exist");

        module
    }

    pub fn metaclass(&self, object_ref: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
        match *object_ref {
            RubyObject::Module { ref class, .. } => {
                match class.get() {
                    metaclass_ref@&RubyObject::Metaclass { .. } =>
                        metaclass_ref,
                    class_ref@_ => {
                        let metaclass_ref = self.alloc(RubyObject::Metaclass {
                            id: self.new_object_id(),
                            of: object_ref,
                            class: Cell::new(self.Class),
                            superclass: Cell::new(Some(class_ref)),
                        });

                        class.set(metaclass_ref);

                        metaclass_ref
                    }
                }
            },
            RubyObject::Class { ref class, .. } |
            RubyObject::Metaclass { ref class, .. } => {
                match class.get() {
                    metaclass_ref@&RubyObject::Metaclass { .. } =>
                        metaclass_ref,
                    _ => {
                        let metaclass_ref = self.arena.alloc(RubyObject::Metaclass {
                            id: self.new_object_id(),
                            of: object_ref,
                            class: class.clone(),
                            superclass: Cell::new(object_ref.superclass().map(|c| self.metaclass(c))),
                        });

                        class.set(metaclass_ref);

                        metaclass_ref
                    },
                }
            },
            RubyObject::IClass {..} => panic!("iclass has no metaclass"),
        }
    }

    pub fn get_const(&self, object: &'a RubyObject<'a>, name: &str) -> Option<Rc<ConstantEntry<'a>>> {
        let constants_ref = self.constants.borrow();

        let (superclass, constants) =
            match *object {
                RubyObject::Module { ref superclass, .. } |
                RubyObject::Class { ref superclass, .. } |
                RubyObject::Metaclass { ref superclass, .. } =>
                    (superclass, constants_ref.get(object)),
                RubyObject::IClass { ref superclass, ref site, .. } =>
                    (superclass, constants_ref.get(site.module))
            };

        match constants.and_then(|c| c.get(name)) {
            Some(ce) => Some(ce.clone()),
            None => match superclass.get() {
                None => None,
                Some(c) => self.get_const(c, name),
            }
        }
    }

    pub fn set_const(&self, object: &'a RubyObject<'a>, name: &str, entry: Rc<ConstantEntry<'a>>) -> Result<(), ()> {
        match Self::class_table_lookup(&self.constants, object, name) {
            Some(_) => Err(()),
            None => {
                Self::class_table_insert(&self.constants, object, name.to_owned(), entry);
                Ok(())
            },
        }
    }

    pub fn get_own_const(&self, object: &'a RubyObject<'a>, name: &str) -> Option<Rc<ConstantEntry<'a>>> {
        match *object {
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {},
            RubyObject::IClass { .. } => panic!("called get_own_const with RubyObject::IClass!"),
        };

        let constants_ref = self.constants.borrow();

        let constants = constants_ref.get(object);

        constants.and_then(|c| c.get(name)).map(|entry| entry.clone())
    }

    pub fn get_const_for_definition(&self, object: &'a RubyObject<'a>, name: &str) -> Option<Rc<ConstantEntry<'a>>> {
        let constants_ref = self.constants.borrow();

        let constants = constants_ref.get(object);

        match constants.and_then(|c| c.get(name)) {
            Some(ce) => Some(ce.clone()),
            None => {
                // vm_search_const_defined_class special cases constant lookups against
                // Object when used in a class/module definition context:
                if object == self.Object {
                    let superclass = match *object {
                        RubyObject::Module { ref superclass, .. } |
                        RubyObject::Class { ref superclass, .. } |
                        RubyObject::Metaclass { ref superclass, .. } => superclass.get(),
                        RubyObject::IClass { .. } => panic!("called get_const_for_definition with RubyObject::IClass"),
                    };

                    match superclass {
                        None => None,
                        Some(c) => self.get_const(c, name),
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn constant_path(&self, object: &'a RubyObject<'a>, name: &str) -> String {
        if object == self.Object {
            name.to_owned()
        } else {
            format!("{}::{}", object.name(), name)
        }
    }

    pub fn define_method(&self, target: &'a RubyObject<'a>, name: String, entry: Rc<MethodEntry<'a>>) {
        Self::class_table_insert(&self.methods, target, name, entry)
    }

    pub fn lookup_method_direct(&self, module: &'a RubyObject<'a>, name: &str) -> Option<Rc<MethodEntry<'a>>> {
        Self::class_table_lookup(&self.methods, module.delegate(), name)
    }

    pub fn lookup_method(&self, klass: &'a RubyObject<'a>, name: &str) -> Option<Rc<MethodEntry<'a>>> {
        klass.ancestors()
            .filter_map(|k| self.lookup_method_direct(k, name))
            .nth(0)
    }

    pub fn define_ivar(&self, target: &'a RubyObject<'a>, name: String, ivar: Rc<IvarEntry<'a>>) {
        Self::class_table_insert(&self.ivars, target, name, ivar)
    }

    pub fn lookup_ivar(&self, klass: &'a RubyObject<'a>, name: &str) -> Option<Rc<IvarEntry<'a>>> {
        for ancestor in klass.ancestors() {
            let delegate = ancestor.delegate();

            if let Some(ivar) = Self::class_table_lookup(&self.ivars, delegate, name) {
                return Some(ivar.clone())
            }
        }

        None
    }

    // TODO - check for instance variable name conflicts in superclasses and subclasses:
    pub fn include_module(&self, target: &'a RubyObject<'a>, module: &'a RubyObject<'a>, type_parameters: Vec<TypeNodeRef<'a>>, loc: Loc)
        -> Result<(), IncludeError<'a>>
    {
        // TODO - we'll need this to implement prepends later.
        // MRI's prepend implementation relies on changing the type of the object
        // at the module's address. We can't do that here, so instead let's go with
        // JRuby's algorithm involving keeping a reference to the real module.
        fn method_location<'a>(obj: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
            match *obj {
                RubyObject::Module {..} |
                RubyObject::Class {..} |
                RubyObject::Metaclass {..} |
                RubyObject::IClass {..} =>
                    obj
            }
        }

        if target == module.delegate() {
            return Err(IncludeError::CyclicInclude)
        }

        if module.type_parameters().len() != type_parameters.len() {
            panic!("type parameter count mismatch in module inclusion")
        }

        let include_site = Rc::new(IncludeSite {
            loc: loc,
            module: module.delegate(),
            type_parameters: type_parameters,
            reason: target
        });

        let mut current_inclusion_point = method_location(target);

        'next_module: for next_module in module.ancestors() {
            if target == next_module.delegate() {
                return Err(IncludeError::CyclicInclude)
            }

            let mut superclass_seen = false;

            for next_class in method_location(target).ancestors().skip(1) {
                if let RubyObject::IClass { ref site, .. } = *next_class {
                    if next_class.delegate() == next_module.delegate() {
                        if !superclass_seen {
                            current_inclusion_point = next_class;
                        }

                        if site.type_parameters.is_empty() {
                            // modules without type parameters retain their
                            // legacy behaviour of being ignored during inclusion:
                            continue 'next_module;
                        }

                        // duplicate inclusion of modules with type parameters
                        // are not supported:
                        return Err(IncludeError::DuplicateInclude(&site.loc));
                    }
                } else {
                    superclass_seen = true;
                }
            }

            let site = match *next_module {
                RubyObject::IClass { ref site, .. } => site.clone(),
                _ => include_site.clone(),
            };

            let iclass = self.alloc(RubyObject::IClass {
                id: self.new_object_id(),
                superclass: match *current_inclusion_point {
                    RubyObject::Module { ref superclass, .. } |
                    RubyObject::Class { ref superclass, .. } |
                    RubyObject::Metaclass { ref superclass, .. } |
                    RubyObject::IClass { ref superclass, .. } =>
                        superclass.clone(),
                },
                site: site,
            });

            match *current_inclusion_point {
                RubyObject::Module { ref superclass, .. } |
                RubyObject::Class { ref superclass, .. } |
                RubyObject::Metaclass { ref superclass, .. } |
                RubyObject::IClass { ref superclass, .. } =>
                    superclass.set(Some(iclass)),
            };

            current_inclusion_point = iclass;
        }

        Ok(())
    }

    pub fn is_hash(&self, class: &'a RubyObject<'a>) -> bool {
        class.is_a(self.hash_class())
    }

    pub fn is_array(&self, class: &'a RubyObject<'a>) -> bool {
        class.is_a(self.array_class())
    }
}

pub struct ScopeIter<'object> {
    scope: Option<Rc<Scope<'object>>>,
}

impl<'object> Iterator for ScopeIter<'object> {
    type Item = Rc<Scope<'object>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scope.clone().map(|scope| {
            self.scope = scope.parent.clone();
            scope
        })
    }
}

#[derive(Debug)]
pub struct Scope<'object> {
    pub parent: Option<Rc<Scope<'object>>>,
    pub module: &'object RubyObject<'object>,
}

impl<'object> Scope<'object> {
    pub fn root(scope: &Rc<Scope<'object>>) -> Rc<Scope<'object>> {
        match scope.parent {
            Some(ref parent) => Scope::root(parent),
            None => scope.clone(),
        }
    }

    pub fn ancestors(scope: &Rc<Scope<'object>>) -> ScopeIter<'object> {
        ScopeIter { scope: Some(scope.clone()) }
    }

    pub fn spawn(scope: &Rc<Scope<'object>>, module: &'object RubyObject<'object>) -> Rc<Scope<'object>> {
        Rc::new(Scope { parent: Some(scope.clone()), module: module })
    }
}

#[derive(Debug)]
pub struct MethodEntry<'object> {
    pub owner: &'object RubyObject<'object>,
    pub visibility: Cell<MethodVisibility>,
    pub implementation: Rc<MethodImpl<'object>>,
}

#[derive(Debug)]
pub enum MethodImpl<'object> {
    TypedRuby {
        name: String,
        proto: Prototype<'object>,
        body: Option<Rc<Node>>,
        scope: Rc<Scope<'object>>,
    },
    Ruby {
        name: String,
        proto: Prototype<'object>,
    },
    AttrReader {
        ivar: String,
        loc: Loc,
    },
    AttrWriter {
        ivar: String,
        loc: Loc,
    },
    Untyped,
    IntrinsicClassNew,
    IntrinsicProcCall,
    IntrinsicKernelRaise,
    IntrinsicKernelIsA,
    IntrinsicRevealType,
}

#[derive(Debug)]
pub enum ConstantEntry<'object> {
    Module {
        loc: Option<Loc>,
        value: &'object RubyObject<'object>,
    },
    Expression {
        loc: Loc,
        ty: TypeNodeRef<'object>,
        scope_self: &'object RubyObject<'object>,
    }
}

impl<'object> ConstantEntry<'object> {
    pub fn module(&self) -> Option<&'object RubyObject<'object>> {
        if let ConstantEntry::Module { value, .. } = *self {
            Some(value)
        } else {
            None
        }
    }

    pub fn expect_module(&self) -> &'object RubyObject<'object> {
        self.module().expect("ConstantEntry expected to be Module")
    }
}

pub struct IvarEntry<'object> {
    pub ivar_loc: Loc,
    pub ty: TypeNodeRef<'object>,
}

#[derive(Debug)]
pub struct IncludeSite<'object> {
    pub loc: Loc,
    pub module: &'object RubyObject<'object>,
    pub reason: &'object RubyObject<'object>,
    pub type_parameters: Vec<TypeNodeRef<'object>>,
}

#[derive(Debug)]
pub enum IncludeError<'object> {
    CyclicInclude,
    DuplicateInclude(&'object Loc),
}

pub enum RubyObject<'a> {
    Module {
        id: ObjectId,
        class: Cell<&'a RubyObject<'a>>,
        name: String,
        superclass: Cell<Option<&'a RubyObject<'a>>>,
        type_parameters: Vec<Id>,
    },
    Class {
        id: ObjectId,
        class: Cell<&'a RubyObject<'a>>,
        name: String,
        superclass: Cell<Option<&'a RubyObject<'a>>>,
        type_parameters: Vec<Id>,
    },
    Metaclass {
        id: ObjectId,
        class: Cell<&'a RubyObject<'a>>,
        superclass: Cell<Option<&'a RubyObject<'a>>>,
        of: &'a RubyObject<'a>,
    },
    IClass {
        id: ObjectId,
        superclass: Cell<Option<&'a RubyObject<'a>>>,
        site: Rc<IncludeSite<'a>>,
    }
}

impl<'a> RubyObject<'a> {
    fn id(&self) -> ObjectId {
        match *self {
            RubyObject::Module { id, .. } => id,
            RubyObject::Class { id, .. } => id,
            RubyObject::Metaclass { id, .. } => id,
            RubyObject::IClass { id, .. } => id,
        }
    }

    pub fn name(&self) -> String {
        match *self {
            RubyObject::Module { ref name, .. } =>
                name.clone(),
            RubyObject::Class { ref name, .. } =>
                name.clone(),
            RubyObject::Metaclass { of, .. } =>
                format!("Class::[{}]", of.name()),
            RubyObject::IClass { ref site, .. } => {
                format!("iclass for {} (included from {} at {})",
                    site.module.name(), site.reason.name(), site.loc)
            }
        }
    }

    pub fn delegate(&'a self) -> &'a RubyObject<'a> {
        match *self {
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } =>
                self,
            RubyObject::IClass { ref site, .. } =>
                &site.module,
        }
    }

    pub fn ancestors(&'a self) -> AncestorIterator<'a> {
        AncestorIterator { object: Some(self) }
    }

    pub fn include_chain(&'a self, module: &'a RubyObject<'a>) -> Vec<Rc<IncludeSite<'a>>> {
        let mut include_tree = HashMap::new();

        for ancestor in self.ancestors() {
            match *ancestor {
                RubyObject::IClass { ref site, .. } => {
                    include_tree.insert(site.module, site.clone());
                }
                _ => {
                    include_tree.clear();
                }
            }

            if ancestor.delegate() == module.delegate() {
                break
            }
        }

        let mut chain = Vec::new();
        let mut cur = module.delegate();

        loop {
            match include_tree.get(cur) {
                Some(site) => {
                    chain.push(site.clone());
                    cur = site.reason;
                }
                None => break
            }
        }

        chain.reverse();

        chain
    }

    pub fn is_a(&'a self, other: &'a RubyObject<'a>) -> bool {
        for k in self.ancestors() {
            if k.delegate() == other {
                return true
            }
        }

        false
    }

    // returns the next module, class, or metaclass in the ancestry chain.
    // skips iclasses.
    pub fn superclass(&'a self) -> Option<&'a RubyObject<'a>> {
        match *self {
            RubyObject::Module { ref superclass, .. } |
            RubyObject::Class { ref superclass, .. } |
            RubyObject::Metaclass { ref superclass, .. } => {
                let mut superclass = superclass;

                loop {
                    match superclass.get() {
                        None => return None,
                        Some(class@&RubyObject::Module { .. }) |
                        Some(class@&RubyObject::Class { .. }) |
                        Some(class@&RubyObject::Metaclass { .. }) => return Some(class),
                        Some(&RubyObject::IClass { superclass: ref superclass_, .. }) => superclass = superclass_,
                    }
                }
            },
            RubyObject::IClass { .. } =>
                panic!("should not get superclass of iclass directly"),
        }
    }

    pub fn type_parameters(&'a self) -> &'a [Id] {
        match *self {
            RubyObject::Metaclass { .. } => {
                &[]
            },
            RubyObject::Module { ref type_parameters, .. } |
            RubyObject::Class { ref type_parameters, .. } =>
                type_parameters,
            RubyObject::IClass { .. } =>
                panic!("called type_parameters on RubyObject::IClass!"),
        }
    }
}

impl<'a> fmt::Debug for RubyObject<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<'a> PartialEq for RubyObject<'a> {
    fn eq(&self, other: &RubyObject) -> bool {
        self.id() == other.id()
    }
}

impl<'a> Eq for RubyObject<'a> {}

impl<'a> Hash for RubyObject<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}
