use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::ops::DerefMut;

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

#[allow(non_snake_case)]
pub struct ObjectGraph {
    pub BasicObject: RubyObjectRef,
    pub Object: RubyObjectRef,
    pub Module: RubyObjectRef,
    pub Class: RubyObjectRef,

    _objects: RefCell<ObjectMap>,
    ids: GenId,
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
        }));

        objects.insert(basic_object_metaclass_ref.id, Box::new(RubyObject::Metaclass {
            id: basic_object_metaclass_ref.id,
            of: basic_object_ref.id,
            class: class_ref.to_cell(),
            superclass: class_ref.to_cell(),
            constants: HashMap::new(),
        }));

        objects.insert(object_ref.id, Box::new(RubyObject::Class {
            id: object_ref.id,
            name: "Object".to_owned(),
            class: class_ref.to_cell(),
            superclass: basic_object_ref.to_cell(),
            constants: HashMap::new(),
        }));

        objects.insert(module_ref.id, Box::new(RubyObject::Class {
            id: module_ref.id,
            name: "Module".to_owned(),
            class: class_ref.to_cell(),
            superclass: object_ref.to_cell(),
            constants: HashMap::new(),
        }));

        objects.insert(class_ref.id, Box::new(RubyObject::Class {
            id: class_ref.id,
            name: "Class".to_owned(),
            class: class_ref.to_cell(),
            superclass: module_ref.to_cell(),
            constants: HashMap::new(),
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
                            superclass: class.clone(),
                            constants: HashMap::new(),
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
                        });

                        class.set(metaclass_id);

                        metaclass_ref
                    },
                }
            }
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
        }
    }

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
                }
        }
    }

    pub fn type_of(&self, object: &RubyObjectRef) -> ObjectType {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => ObjectType::Object,
            RubyObject::Module {..} => ObjectType::Module,
            RubyObject::Class {..} => ObjectType::Class,
            RubyObject::Metaclass {..} => ObjectType::Metaclass,
        }
    }

    pub fn has_const(&self, object: &RubyObjectRef, name: &str) -> bool {
        self.get_const(object, name).is_some()
    }

    pub fn get_const(&self, object: &RubyObjectRef, name: &str) -> Option<RubyObjectRef> {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => panic!("called get_const with RubyObject::Object!"),
            RubyObject::Module { ref superclass, ref constants, .. } |
            RubyObject::Class { ref superclass, ref constants, .. } |
            RubyObject::Metaclass { ref superclass, ref constants, .. } => {
                match constants.get(name) {
                    Some(id) => Some(RubyObjectRef { id: *id }),
                    None => match superclass.get() {
                        NO_OBJECT_ID => None,
                        id => self.get_const(&RubyObjectRef { id: id }, name),
                    }
                }
            }
        }
    }

    pub fn set_const(&self, object: &RubyObjectRef, name: &str, value: &RubyObjectRef) -> Option<()> {
        match *self.get_object(object.id) {
            RubyObject::Object {..} => panic!("called put_const with RubyObject::Object!"),
            RubyObject::Module { ref mut constants, .. } |
            RubyObject::Class { ref mut constants, .. } |
            RubyObject::Metaclass { ref mut constants, .. } =>
                if constants.contains_key(name) {
                    None
                } else {
                    constants.insert(name.to_owned(), value.id);
                    Some(())
                }
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
            }
        }
    }

    pub fn constant_path(&self, object: &RubyObjectRef, name: &str) -> String {
        if *object == self.Object {
            name.to_owned()
        } else {
            self.name(object) + name
        }
    }
}

pub enum ObjectType {
    Object,
    Module,
    Class,
    Metaclass,
}

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
    },
    Class {
        id: ObjectId,
        class: Cell<ObjectId>,
        name: String,
        superclass: Cell<ObjectId>,
        constants: HashMap<String, ObjectId>,
    },
    Metaclass {
        id: ObjectId,
        class: Cell<ObjectId>,
        superclass: Cell<ObjectId>,
        of: ObjectId,
        constants: HashMap<String, ObjectId>,
    }
}
