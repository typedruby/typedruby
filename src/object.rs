use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::ops::DerefMut;
use std::rc::Rc;
use ast::Node;

// can become NonZero<u64> once NonZero for non-pointer types hits stable:
type ObjectId = u64;
// then we can use Option<ObjectId> rather than manually treating 0 as none-ish:
const NO_OBJECT_ID: ObjectId = 0;

type ObjectMap = HashMap<ObjectId, Box<RubyObject>>;

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

#[derive(Debug,Clone,Eq,PartialEq)]
pub struct RubyObjectRef {
    id: ObjectId
}

impl RubyObjectRef {
    fn to_cell(&self) -> Cell<ObjectId> {
        Cell::new(self.id)
    }
}

struct AncestorIterator<'a> {
    graph: &'a ObjectGraph,
    object: Option<&'a RubyObject>,
}

impl<'a> Iterator for AncestorIterator<'a> {
    type Item = &'a RubyObject;

    fn next(&mut self) -> Option<Self::Item> {
        match self.object {
            None => return None,
            Some(&RubyObject::Object { .. }) => panic!(),
            Some(&RubyObject::Module { ref superclass, .. }) |
            Some(&RubyObject::Class { ref superclass, .. }) |
            Some(&RubyObject::Metaclass { ref superclass, .. }) |
            Some(&RubyObject::IClass { ref superclass, .. }) => {
                let superclass_id = superclass.get();

                let cur = self.object;

                self.object =
                    if superclass_id == NO_OBJECT_ID {
                        None
                    } else {
                        Some(self.graph.get_object(superclass_id))
                    };

                cur
            }
        }
    }
}

#[allow(non_snake_case)]
pub struct ObjectGraph {
    pub BasicObject: RubyObjectRef,
    pub Object: RubyObjectRef,
    pub Module: RubyObjectRef,
    pub Class: RubyObjectRef,

    _objects: RefCell<ObjectMap>,
    ids: GenId,
}

impl ObjectGraph {
    fn new_object_id(&self) -> ObjectId {
        self.ids.next()
    }

    pub fn new() -> ObjectGraph {
        let mut objects = ObjectMap::new();

        // manually bootstrap cyclic core of object graph:

        let ids = GenId::new();
        let basic_object_ref = RubyObjectRef { id: ids.next() };
        let basic_object_metaclass_ref = RubyObjectRef { id: ids.next() };
        let object_ref = RubyObjectRef { id: ids.next() };
        let module_ref = RubyObjectRef { id: ids.next() };
        let class_ref = RubyObjectRef { id: ids.next() };

        objects.insert(basic_object_ref.id, Box::new(RubyObject::Class {
            id: basic_object_ref.id,
            name: "BasicObject".to_owned(),
            class: basic_object_metaclass_ref.to_cell(),
            superclass: Cell::new(NO_OBJECT_ID),
            constants: HashMap::new(),
            methods: HashMap::new(),
        }));

        objects.insert(basic_object_metaclass_ref.id, Box::new(RubyObject::Metaclass {
            id: basic_object_metaclass_ref.id,
            of: basic_object_ref.id,
            class: class_ref.to_cell(),
            superclass: class_ref.to_cell(),
            constants: HashMap::new(),
            methods: HashMap::new(),
        }));

        objects.insert(object_ref.id, Box::new(RubyObject::Class {
            id: object_ref.id,
            name: "Object".to_owned(),
            class: class_ref.to_cell(),
            superclass: basic_object_ref.to_cell(),
            constants: HashMap::new(),
            methods: HashMap::new(),
        }));

        objects.insert(module_ref.id, Box::new(RubyObject::Class {
            id: module_ref.id,
            name: "Module".to_owned(),
            class: class_ref.to_cell(),
            superclass: object_ref.to_cell(),
            constants: HashMap::new(),
            methods: HashMap::new(),
        }));

        objects.insert(class_ref.id, Box::new(RubyObject::Class {
            id: class_ref.id,
            name: "Class".to_owned(),
            class: class_ref.to_cell(),
            superclass: module_ref.to_cell(),
            constants: HashMap::new(),
            methods: HashMap::new(),
        }));

        ObjectGraph {
            BasicObject: basic_object_ref,
            Object: object_ref,
            Module: module_ref,
            Class: class_ref,

            _objects: RefCell::new(objects),
            ids: ids,
        }
    }

    fn get_object(&self, id: ObjectId) -> &mut RubyObject {
        let mut objects = self._objects.borrow_mut();
        let ref_ = objects.get_mut(&id).expect("dangling ObjectId").deref_mut();

        // extend lifetime of &RubyObject to that of self.
        // WARNING: potentially unsafe - these references *must not* be
        // retained across GCs of the object graph.
        // MORE WARNING: this can potentially create aliasing mutable
        // references! use with care.
        unsafe { ::std::mem::transmute::<&mut RubyObject, &mut RubyObject>(ref_) }
    }

    fn put_object(&self, id: ObjectId, object: RubyObject) -> RubyObjectRef {
        self._objects.borrow_mut().insert(id, Box::new(object));
        RubyObjectRef { id: id }
    }

    pub fn new_class(&self, name: String, superclass: &RubyObjectRef) -> RubyObjectRef {
        let id = self.new_object_id();

        self.put_object(id, RubyObject::Class {
            id: id,
            name: name,
            class: self.Class.to_cell(),
            superclass: superclass.to_cell(),
            constants: HashMap::new(),
            methods: HashMap::new(),
        })
    }

    pub fn new_module(&self, name: String) -> RubyObjectRef {
        let id = self.new_object_id();

        self.put_object(id, RubyObject::Module {
            id: id,
            name: name,
            class: self.Module.to_cell(),
            superclass: Cell::new(NO_OBJECT_ID),
            constants: HashMap::new(),
            methods: HashMap::new(),
        })
    }

    pub fn metaclass(&self, object_ref: &RubyObjectRef) -> RubyObjectRef {
        match *self.get_object(object_ref.id) {
            RubyObject::Object { id, ref class, .. } |
            RubyObject::Module { id, ref class, .. } => {
                match *self.get_object(class.get()) {
                    RubyObject::Metaclass { id: metaclass_id, .. } =>
                        RubyObjectRef { id: metaclass_id },
                    _ => {
                        let metaclass_id = self.new_object_id();

                        let metaclass_ref = self.put_object(metaclass_id, RubyObject::Metaclass {
                            id: metaclass_id,
                            of: id,
                            class: self.Class.to_cell(),
                            superclass: object_ref.to_cell(),
                            constants: HashMap::new(),
                            methods: HashMap::new(),
                        });

                        class.set(metaclass_id);

                        metaclass_ref
                    }
                }
            },
            RubyObject::Class { ref id, ref class, ref superclass, .. } |
            RubyObject::Metaclass { ref id, ref class, ref superclass, .. } => {
                match *self.get_object(class.get()) {
                    RubyObject::Metaclass { id, .. } =>
                        RubyObjectRef { id: id },
                    _ => {
                        let metaclass_id = self.new_object_id();

                        let metaclass_ref = self.put_object(metaclass_id, RubyObject::Metaclass {
                            id: metaclass_id,
                            of: *id,
                            class: class.clone(),
                            // no need to check for None superclass here - BasicObject's metaclass was already
                            // constructed in ObjectGraph::bootstrap:
                            // TODO - we do need to replace the direct superclass field get with something that
                            // ignores iclasses:
                            superclass: self.metaclass(&RubyObjectRef { id: superclass.get() }).to_cell(),
                            constants: HashMap::new(),
                            methods: HashMap::new(),
                        });

                        class.set(metaclass_id);

                        metaclass_ref
                    },
                }
            },
            RubyObject::IClass {..} => panic!("iclass has no metaclass"),
        }
    }

    pub fn name(&self, object: &RubyObjectRef) -> String {
        match *self.get_object(object.id) {
            RubyObject::Object { ref class, .. } =>
                format!("#<{}>", self.name(&RubyObjectRef { id: class.get() })),
            RubyObject::Module { ref name, .. } =>
                name.clone(),
            RubyObject::Class { ref name, .. } =>
                name.clone(),
            RubyObject::Metaclass { of, .. } =>
                format!("Class::[{}]", self.name(&RubyObjectRef { id: of })),
            RubyObject::IClass { .. } =>
                panic!("iclass has no name"),
        }
    }

    fn ancestors<'a>(&'a self, object: &'a RubyObject) -> AncestorIterator<'a> {
        AncestorIterator { graph: self, object: Some(object) }
    }

    // returns the next module, class, or metaclass in the ancestry chain
    // skips iclasses.
    pub fn superclass(&self, object: &RubyObjectRef) -> Option<RubyObjectRef> {
        match *self.get_object(object.id) {
            RubyObject::Object { .. } =>
                panic!("called superclass with RubyObject::Object!"),
            RubyObject::Module { ref superclass, .. } |
            RubyObject::Class { ref superclass, .. } |
            RubyObject::Metaclass { ref superclass, .. } =>
                // TODO - need to skip iclasses here:
                if superclass.get() == NO_OBJECT_ID {
                    None
                } else {
                    Some(RubyObjectRef { id: superclass.get() })
                },
            RubyObject::IClass { .. } =>
                panic!("should not get superclass of iclass directly"),
        }
    }

    pub fn type_of(&self, object: &RubyObjectRef) -> ObjectType {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => ObjectType::Object,
            RubyObject::Module {..} => ObjectType::Module,
            RubyObject::Class {..} => ObjectType::Class,
            RubyObject::Metaclass {..} => ObjectType::Metaclass,
            RubyObject::IClass {..} => panic!("iclass is hidden object type"),
        }
    }

    pub fn has_const(&self, object: &RubyObjectRef, name: &str) -> bool {
        self.get_const(object, name).is_some()
    }

    pub fn get_const(&self, object: &RubyObjectRef, name: &str) -> Option<RubyObjectRef> {
        let (superclass, constants) =
            match *self.get_object(object.id) {
                RubyObject::Object {..} => panic!("called get_const with RubyObject::Object!"),
                RubyObject::Module { ref superclass, ref constants, .. } |
                RubyObject::Class { ref superclass, ref constants, .. } |
                RubyObject::Metaclass { ref superclass, ref constants, .. } =>
                    (superclass, constants),
                RubyObject::IClass { ref superclass, module, .. } =>
                    if let RubyObject::Module { ref constants, .. } = *self.get_object(module) {
                        (superclass, constants)
                    } else {
                        panic!("iclass module does not reference object of module type")
                    },
            };

        match constants.get(name) {
            Some(id) => Some(RubyObjectRef { id: *id }),
            None => match superclass.get() {
                NO_OBJECT_ID => None,
                id => self.get_const(&RubyObjectRef { id: id }, name),
            }
        }
    }

    pub fn set_const(&self, object: &RubyObjectRef, name: &str, value: &RubyObjectRef) -> bool {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => panic!("called put_const with RubyObject::Object!"),
            RubyObject::Module { ref mut constants, .. } |
            RubyObject::Class { ref mut constants, .. } |
            RubyObject::Metaclass { ref mut constants, .. } =>
                if constants.contains_key(name) {
                    false
                } else {
                    constants.insert(name.to_owned(), value.id);
                    true
                },
            RubyObject::IClass { .. } =>
                panic!("cannot set constant directly on iclass"),
        }
    }

    pub fn get_const_for_definition(&self, object: &RubyObjectRef, name: &str) -> Option<RubyObjectRef> {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => panic!("called get_const_for_definition with RubyObject::Object!"),
            RubyObject::Module { ref superclass, ref constants, .. } |
            RubyObject::Class { ref superclass, ref constants, .. } |
            RubyObject::Metaclass { ref superclass, ref constants, .. } => {
                match constants.get(name) {
                    Some(id) => Some(RubyObjectRef { id: *id }),
                    None => {
                        // vm_search_const_defined_class special cases constant lookups against
                        // Object when used in a class/module definition context:
                        if *object == self.Object {
                            self.get_const(&RubyObjectRef { id: superclass.get() }, name)
                        } else {
                            None
                        }
                    }
                }
            },
            RubyObject::IClass { .. } => panic!("called get_const_for_definition with RubyObject::IClass"),
        }
    }

    pub fn constant_path(&self, object: &RubyObjectRef, name: &str) -> String {
        if *object == self.Object {
            name.to_owned()
        } else {
            self.name(object) + name
        }
    }

    pub fn define_method(&self, target: &RubyObjectRef, name: String, node: Rc<Node>) {
        match *self.get_object(target.id) {
            RubyObject::Object {..} => panic!("called define_method with RubyObject::Object!"),
            RubyObject::Module { ref mut methods, .. } |
            RubyObject::Class { ref mut methods, .. } |
            RubyObject::Metaclass { ref mut methods, .. } => {
                let mut def_list = methods.entry(name).or_insert(Vec::new());
                def_list.push(node);
            }
            RubyObject::IClass {..} => panic!("called define_method with RubyObject::IClass!"),
        }
    }

    pub fn include_module(&self, target: &RubyObjectRef, module: &RubyObjectRef) -> bool {
        // TODO - we'll need this to implement prepends later.
        // MRI's prepend implementation relies on changing the type of the object
        // at the module's address. We can't do that here, so instead let's go with
        // JRuby's algorithm involving keeping a reference to the real module.
        fn method_location(obj: &RubyObject) -> &RubyObject {
            match *obj {
                RubyObject::Object {..} => panic!(),
                RubyObject::Module {..} |
                RubyObject::Class {..} |
                RubyObject::Metaclass {..} |
                RubyObject::IClass {..} =>
                    obj
            }
        }

        fn delegate<'a>(g: &'a ObjectGraph, obj: &'a RubyObject) -> &'a RubyObject {
            match *obj {
                RubyObject::Object {..} => panic!(),
                RubyObject::Module {..} |
                RubyObject::Class {..} |
                RubyObject::Metaclass {..} =>
                    obj,
                RubyObject::IClass { module, .. } =>
                    g.get_object(module),
            }
        }

        let target = self.get_object(target.id);
        let module = self.get_object(module.id);

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

            let iclass_id = self.new_object_id();

            self.put_object(iclass_id, RubyObject::IClass {
                id: iclass_id,
                superclass: match *current_inclusion_point {
                    RubyObject::Object { .. } =>
                        panic!("current_inclusion_point is object"),
                    RubyObject::Module { ref superclass, .. } |
                    RubyObject::Class { ref superclass, .. } |
                    RubyObject::Metaclass { ref superclass, .. } |
                    RubyObject::IClass { ref superclass, .. } =>
                        superclass.clone(),
                },
                module: delegate(self, next_module).id(),
            });

            current_inclusion_point = self.get_object(iclass_id);
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

#[derive(Debug)]
enum RubyObject {
    Object {
        id: ObjectId,
        class: Cell<ObjectId>,
    },
    Module {
        id: ObjectId,
        class: Cell<ObjectId>,
        name: String,
        superclass: Cell<ObjectId>,
        constants: HashMap<String, ObjectId>,
        methods: HashMap<String, Vec<Rc<Node>>>,
    },
    Class {
        id: ObjectId,
        class: Cell<ObjectId>,
        name: String,
        superclass: Cell<ObjectId>,
        constants: HashMap<String, ObjectId>,
        methods: HashMap<String, Vec<Rc<Node>>>,
    },
    Metaclass {
        id: ObjectId,
        class: Cell<ObjectId>,
        superclass: Cell<ObjectId>,
        of: ObjectId,
        constants: HashMap<String, ObjectId>,
        methods: HashMap<String, Vec<Rc<Node>>>,
    },
    IClass {
        id: ObjectId,
        superclass: Cell<ObjectId>,
        module: ObjectId,
    }
}

impl RubyObject {
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

impl PartialEq for RubyObject {
    fn eq(&self, other: &RubyObject) -> bool {
        self.id() == other.id()
    }
}

impl Eq for RubyObject {}
