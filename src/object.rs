use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::fmt;
use typed_arena::Arena;
use ast::{Node, Loc, Id};

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
            Some(&RubyObject::Object { .. }) => panic!(),
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
                RubyObject::Object { class: ref class_, .. } |
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

            constants: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
            ivars: RefCell::new(HashMap::new()),
        };

        o.set_const(o.BasicObject, "BasicObject", None, o.BasicObject);
        o.set_const(o.Object, "Object", None, o.Object);
        o.set_const(o.Object, "Module", None, o.Module);
        o.set_const(o.Object, "Class", None, o.Class);

        o.Kernel = o.define_module(None, o.Object, "Kernel");
        o.include_module(o.Object, o.Kernel);
        o.Boolean = o.define_class(None, o.Object, "Boolean", o.Object, Vec::new());
        o.TrueClass = o.define_class(None, o.Object, "TrueClass", o.Boolean, Vec::new());
        o.FalseClass = o.define_class(None, o.Object, "FalseClass", o.Boolean, Vec::new());
        o.NilClass = o.define_class(None, o.Object, "NilClass", o.Object, Vec::new());
        o.Symbol = o.define_class(None, o.Object, "Symbol", o.Object, Vec::new());
        o.String = o.define_class(None, o.Object, "String", o.Object, Vec::new());
        o.Numeric = o.define_class(None, o.Object, "Numeric", o.Object, Vec::new());
        o.Integer = o.define_class(None, o.Object, "Integer", o.Numeric, Vec::new());
        o.Float = o.define_class(None, o.Object, "Float", o.Numeric, Vec::new());

        o
    }

    fn alloc(&self, obj: RubyObject<'a>) -> &'a RubyObject<'a> {
        self.arena.alloc(obj)
    }

    fn class_table_lookup<T>(table: &ClassTable<'a, T>, class: &'a RubyObject<'a>, key: &str) -> Option<Rc<T>> {
        match *class {
            RubyObject::Object { .. } => panic!("RubyObject::Object has no associated class table"),
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
            RubyObject::Object { .. } => panic!("RubyObject::Object has no associated class table"),
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

    pub fn new_module(&self, name: String) -> &'a RubyObject<'a> {
        self.alloc(RubyObject::Module {
            id: self.new_object_id(),
            name: name,
            class: Cell::new(self.Module),
            superclass: Cell::new(None),
        })
    }

    pub fn define_class(&self, loc: Option<Loc>, owner: &'a RubyObject<'a>, name: &str, superclass: &'a RubyObject<'a>, type_parameters: Vec<Id>) -> &'a RubyObject<'a> {
        let class = self.new_class(self.constant_path(owner, name), superclass, type_parameters);

        self.set_const(owner, name, loc, class);

        class
    }

    pub fn define_module(&self, loc: Option<Loc>, owner: &'a RubyObject<'a>, name: &str) -> &'a RubyObject<'a> {
        let module = self.new_module(self.constant_path(owner, name));

        self.set_const(owner, name, loc, module);

        module
    }

    pub fn metaclass(&self, object_ref: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
        match *object_ref {
            RubyObject::Object { ref class, .. } |
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
                            // no need to check for None superclass here - BasicObject's metaclass was already
                            // constructed in ObjectGraph::bootstrap:
                            // TODO - we do need to replace the direct superclass field get with something that
                            // ignores iclasses:
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

    pub fn get_const(&self, object: &'a RubyObject<'a>, name: &str) -> Option<&'a RubyObject<'a>> {
        let constants_ref = self.constants.borrow();

        let (superclass, constants) =
            match *object {
                RubyObject::Object { .. } => panic!("called get_const with RubyObject::Object!"),
                RubyObject::Module { ref superclass, .. } |
                RubyObject::Class { ref superclass, .. } |
                RubyObject::Metaclass { ref superclass, .. } =>
                    (superclass, constants_ref.get(object)),
                RubyObject::IClass { ref superclass, ref module, .. } =>
                    (superclass, constants_ref.get(module))
            };

        match constants.and_then(|c| c.get(name)) {
            Some(ce) => Some(ce.value),
            None => match superclass.get() {
                None => None,
                Some(c) => self.get_const(c, name),
            }
        }
    }

    pub fn set_const(&self, object: &'a RubyObject<'a>, name: &str, loc: Option<Loc>, value: &'a RubyObject<'a>) -> bool {
        match Self::class_table_lookup(&self.constants, object, name) {
            Some(_) => false,
            None => {
                Self::class_table_insert(&self.constants, object, name.to_owned(), Rc::new(ConstantEntry {
                    loc: loc,
                    value: value,
                }));
                true
            },
        }
    }

    pub fn has_own_const(&self, object: &'a RubyObject<'a>, name: &str) -> bool {
        match *object {
            RubyObject::Object { .. } => panic!("called has_own_const with RubyObject::Object!"),
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {},
            RubyObject::IClass { .. } => panic!("called has_own_const with RubyObject::IClass!"),
        };

        let constants_ref = self.constants.borrow();

        let constants = constants_ref.get(object);

        match constants.and_then(|c| c.get(name)) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_const_for_definition(&self, object: &'a RubyObject<'a>, name: &str) -> Option<&'a RubyObject<'a>> {
        let constants_ref = self.constants.borrow();

        let constants = constants_ref.get(object);

        match constants.and_then(|c| c.get(name)) {
            Some(ce) => Some(ce.value),
            None => {
                // vm_search_const_defined_class special cases constant lookups against
                // Object when used in a class/module definition context:
                if object == self.Object {
                    let superclass = match *object {
                        RubyObject::Object {..} => panic!("called get_const_for_definition with RubyObject::Object!"),
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

    pub fn lookup_method(&self, klass: &'a RubyObject<'a>, name: &str) -> Option<Rc<MethodEntry<'a>>> {
        for ancestor in klass.ancestors() {
            let delegate = ancestor.delegate();

            if let Some(method) = Self::class_table_lookup(&self.methods, delegate, name) {
                return Some(method.clone());
            }
        }

        None
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
    pub fn include_module(&self, target: &'a RubyObject<'a>, module: &'a RubyObject<'a>) -> bool {
        // TODO - we'll need this to implement prepends later.
        // MRI's prepend implementation relies on changing the type of the object
        // at the module's address. We can't do that here, so instead let's go with
        // JRuby's algorithm involving keeping a reference to the real module.
        fn method_location<'a>(obj: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
            match *obj {
                RubyObject::Object {..} => panic!(),
                RubyObject::Module {..} |
                RubyObject::Class {..} |
                RubyObject::Metaclass {..} |
                RubyObject::IClass {..} =>
                    obj
            }
        }

        if target == module.delegate() {
            // cyclic include
            return false
        }

        let mut current_inclusion_point = method_location(target);

        'next_module: for next_module in module.ancestors() {
            if target == next_module.delegate() {
                // cyclic include
                return false
            }

            let mut superclass_seen = false;

            for next_class in method_location(target).ancestors().skip(1) {
                if let RubyObject::IClass {..} = *next_class {
                    if next_class.delegate() == next_module.delegate() {
                        if !superclass_seen {
                            current_inclusion_point = next_class;
                        }

                        continue 'next_module;
                    }
                } else {
                    superclass_seen = true;
                }
            }

            let iclass = self.alloc(RubyObject::IClass {
                id: self.new_object_id(),
                superclass: match *current_inclusion_point {
                    RubyObject::Object { .. } =>
                        panic!("current_inclusion_point is object"),
                    RubyObject::Module { ref superclass, .. } |
                    RubyObject::Class { ref superclass, .. } |
                    RubyObject::Metaclass { ref superclass, .. } |
                    RubyObject::IClass { ref superclass, .. } =>
                        superclass.clone(),
                },
                module: next_module.delegate(),
            });

            match *current_inclusion_point {
                RubyObject::Object { .. } => panic!(),
                RubyObject::Module { ref superclass, .. } |
                RubyObject::Class { ref superclass, .. } |
                RubyObject::Metaclass { ref superclass, .. } |
                RubyObject::IClass { ref superclass, .. } =>
                    superclass.set(Some(iclass)),
            };

            current_inclusion_point = iclass;
        }

        true
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

pub enum MethodEntry<'object> {
    Ruby {
        owner: &'object RubyObject<'object>,
        name: String,
        node: Rc<Node>,
        scope: Rc<Scope<'object>>,
    },
    Untyped,
}

pub struct ConstantEntry<'object> {
    pub loc: Option<Loc>,
    pub value: &'object RubyObject<'object>,
}

pub struct IvarEntry<'object> {
    pub ivar_loc: Loc,
    pub type_node: Rc<Node>,
    pub scope: Rc<Scope<'object>>,
}

pub enum RubyObject<'a> {
    Object {
        id: ObjectId,
        class: Cell<&'a RubyObject<'a>>,
    },
    Module {
        id: ObjectId,
        class: Cell<&'a RubyObject<'a>>,
        name: String,
        superclass: Cell<Option<&'a RubyObject<'a>>>,
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
        module: &'a RubyObject<'a>,
    }
}

impl<'a> RubyObject<'a> {
    fn id(&self) -> ObjectId {
        match *self {
            RubyObject::Object { id, .. } => id,
            RubyObject::Module { id, .. } => id,
            RubyObject::Class { id, .. } => id,
            RubyObject::Metaclass { id, .. } => id,
            RubyObject::IClass { id, .. } => id,
        }
    }

    pub fn name(&self) -> String {
        match *self {
            RubyObject::Object { ref class, .. } =>
                format!("#<{}>", class.get().name()),
            RubyObject::Module { ref name, .. } =>
                name.clone(),
            RubyObject::Class { ref name, .. } =>
                name.clone(),
            RubyObject::Metaclass { of, .. } =>
                format!("Class::[{}]", of.name()),
            RubyObject::IClass { .. } =>
                panic!("iclass has no name"),
        }
    }

    pub fn delegate(&'a self) -> &'a RubyObject<'a> {
        match *self {
            RubyObject::Object { .. } => panic!(),
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } =>
                self,
            RubyObject::IClass { module, .. } =>
                module,
        }
    }

    pub fn ancestors(&'a self) -> AncestorIterator<'a> {
        AncestorIterator { object: Some(self) }
    }

    // returns the next module, class, or metaclass in the ancestry chain.
    // skips iclasses.
    pub fn superclass(&'a self) -> Option<&'a RubyObject<'a>> {
        match *self {
            RubyObject::Object { .. } =>
                panic!("called superclass with RubyObject::Object!"),
            RubyObject::Module { ref superclass, .. } |
            RubyObject::Class { ref superclass, .. } |
            RubyObject::Metaclass { ref superclass, .. } => {
                let mut superclass = superclass;

                loop {
                    match superclass.get() {
                        None => return None,
                        Some(&RubyObject::Object { .. }) => panic!(),
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
            RubyObject::Object { .. } =>
                panic!("called type_parameters on RubyObject::Object!"),
            RubyObject::Module { .. } |
            RubyObject::Metaclass { .. } => {
                &[]
            },
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
