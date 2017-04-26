use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use typed_arena::Arena;
use ast::{Node, SourceFile, Loc};

// can become NonZero<u64> once NonZero for non-pointer types hits stable:
type ObjectId = u64;
// then we can use Option<ObjectId> rather than manually treating 0 as none-ish:
const NO_OBJECT_ID: ObjectId = 0;

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

struct AncestorIterator<'a> {
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

#[allow(non_snake_case)]
pub struct ObjectGraph<'a> {
    ids: GenId,
    arena: &'a Arena<RubyObject<'a>>,

    pub BasicObject: &'a RubyObject<'a>,
    pub Object: &'a RubyObject<'a>,
    pub Module: &'a RubyObject<'a>,
    pub Class: &'a RubyObject<'a>,

    constants: RefCell<HashMap<&'a RubyObject<'a>, HashMap<String, ConstantEntry<'a>>>>,
    methods: RefCell<HashMap<&'a RubyObject<'a>, HashMap<String, MethodEntry>>>,
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
        });

        let object = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Object".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(basic_object)),
        });

        let module = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Module".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(object)),
        });

        let class = arena.alloc(RubyObject::Class {
            id: ids.next(),
            name: "Class".to_owned(),
            class: Cell::new(unsafe_null_ref),
            superclass: Cell::new(Some(module)),
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

        ObjectGraph {
            ids: ids,
            arena: arena,

            BasicObject: basic_object,
            Object: object,
            Module: module,
            Class: class,

            constants: RefCell::new(HashMap::new()),
            methods: RefCell::new(HashMap::new()),
        }
    }

    fn alloc(&self, obj: RubyObject<'a>) -> &'a RubyObject<'a> {
        self.arena.alloc(obj)
    }

    pub fn new_class(&self, name: String, superclass: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
        self.alloc(RubyObject::Class {
            id: self.new_object_id(),
            name: name,
            class: Cell::new(self.Class),
            superclass: Cell::new(Some(superclass)),
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

    pub fn metaclass(&self, object_ref: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
        match *object_ref {
            RubyObject::Object { id, ref class, .. } |
            RubyObject::Module { id, ref class, .. } => {
                match class.get() {
                    metaclass_ref@&RubyObject::Metaclass { .. } =>
                        metaclass_ref,
                    class_ref@_ => {
                        let metaclass_id = self.new_object_id();

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
            RubyObject::Class { ref id, ref class, .. } |
            RubyObject::Metaclass { ref id, ref class, .. } => {
                match class.get() {
                    metaclass_ref@&RubyObject::Metaclass { .. } =>
                        metaclass_ref,
                    class_ref@_ => {
                        let metaclass_ref = self.arena.alloc(RubyObject::Metaclass {
                            id: self.new_object_id(),
                            of: object_ref,
                            class: class.clone(),
                            // no need to check for None superclass here - BasicObject's metaclass was already
                            // constructed in ObjectGraph::bootstrap:
                            // TODO - we do need to replace the direct superclass field get with something that
                            // ignores iclasses:
                            superclass: Cell::new(self.superclass(object_ref).map(|c| self.metaclass(c))),
                        });

                        class.set(metaclass_ref);

                        metaclass_ref
                    },
                }
            },
            RubyObject::IClass {..} => panic!("iclass has no metaclass"),
        }
    }

    pub fn name(&self, object: &'a RubyObject<'a>) -> String {
        match *object {
            RubyObject::Object { ref class, .. } =>
                format!("#<{}>", self.name(class.get())),
            RubyObject::Module { ref name, .. } =>
                name.clone(),
            RubyObject::Class { ref name, .. } =>
                name.clone(),
            RubyObject::Metaclass { of, .. } =>
                format!("Class::[{}]", self.name(of)),
            RubyObject::IClass { .. } =>
                panic!("iclass has no name"),
        }
    }

    fn ancestors(&self, object: &'a RubyObject<'a>) -> AncestorIterator<'a> {
        AncestorIterator { object: Some(object) }
    }

    // returns the next module, class, or metaclass in the ancestry chain
    // skips iclasses.
    pub fn superclass(&self, object: &'a RubyObject<'a>) -> Option<&'a RubyObject<'a>> {
        match *object {
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

    pub fn type_of(&self, object: &'a RubyObject<'a>) -> ObjectType {
        match *object {
            RubyObject::Object {..} => ObjectType::Object,
            RubyObject::Module {..} => ObjectType::Module,
            RubyObject::Class {..} => ObjectType::Class,
            RubyObject::Metaclass {..} => ObjectType::Metaclass,
            RubyObject::IClass {..} => panic!("iclass is hidden object type"),
        }
    }

    pub fn has_const(&self, object: &'a RubyObject<'a>, name: &str) -> bool {
        self.get_const(object, name).is_some()
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

    pub fn set_const(&self, object: &'a RubyObject<'a>, name: &str, source_file: Rc<SourceFile>, loc: Loc, value: &'a RubyObject<'a>) -> bool {
        match *object {
            RubyObject::Object { .. } => panic!("called put_const with RubyObject::Object!"),
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {},
            RubyObject::IClass { .. } => panic!("cannot set constant directly on iclass"),
        };

        let mut constants_ref = self.constants.borrow_mut();

        let object_constants = constants_ref.entry(object).or_insert_with(|| HashMap::new());

        if object_constants.contains_key(name) {
            false
        } else {
            object_constants.insert(name.to_owned(), ConstantEntry {
                source_file: source_file,
                loc: loc,
                value: value,
            });
            true
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
            self.name(object) + name
        }
    }

    pub fn define_method(&self, target: &'a RubyObject<'a>, name: String, source_file: Rc<SourceFile>, node: Rc<Node>) {
        match *target {
            RubyObject::Object { .. } => panic!("called define_method with RubyObject::Object!"),
            RubyObject::Module { .. } |
            RubyObject::Class { .. } |
            RubyObject::Metaclass { .. } => {}
            RubyObject::IClass {..} => panic!("called define_method with RubyObject::IClass!"),
        }

        let mut methods_ref = self.methods.borrow_mut();

        let methods = methods_ref.entry(target).or_insert_with(|| HashMap::new());

        methods.insert(name, MethodEntry {
            node: node,
            source_file: source_file,
        });
    }

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

        fn delegate<'a>(g: &ObjectGraph<'a>, obj: &'a RubyObject<'a>) -> &'a RubyObject<'a> {
            match *obj {
                RubyObject::Object {..} => panic!(),
                RubyObject::Module {..} |
                RubyObject::Class {..} |
                RubyObject::Metaclass {..} =>
                    obj,
                RubyObject::IClass { module, .. } =>
                    module,
            }
        }

        if target == delegate(self, module) {
            // cyclic include
            return false
        }

        let mut current_inclusion_point = method_location(target);

        'next_module: for next_module in self.ancestors(module) {
            if target == delegate(self, next_module) {
                // cyclic include
                return false
            }

            let mut superclass_seen = false;

            for next_class in self.ancestors(method_location(target)).skip(1) {
                if let RubyObject::IClass {..} = *next_class {
                    if delegate(self, next_class) == delegate(self, next_module) {
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
                module: delegate(self, next_module),
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

pub enum ObjectType {
    Object,
    Module,
    Class,
    Metaclass,
}

#[derive(Clone)]
pub struct MethodEntry {
    pub source_file: Rc<SourceFile>,
    pub node: Rc<Node>,
}

#[derive(Clone)]
pub struct ConstantEntry<'object> {
    pub source_file: Rc<SourceFile>,
    pub loc: Loc,
    pub value: &'object RubyObject<'object>,
}

#[derive(Debug)]
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
