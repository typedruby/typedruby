use std::collections::HashMap;
use std::cell::{Cell, RefCell};

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
pub struct Env {
    pub BasicObject: RubyObjectRef,
    pub Object: RubyObjectRef,
    pub Module: RubyObjectRef,
    pub Class: RubyObjectRef,

    _objects: RefCell<ObjectMap>,
    ids: GenId,
}

pub struct RubyObjectRef {
    id: ObjectId
}

impl RubyObjectRef {
    fn to_cell(&self) -> Cell<ObjectId> {
        Cell::new(self.id)
    }
}

impl Env {
    fn new_object_id(&self) -> ObjectId {
        self.ids.next()
    }

    pub fn new() -> Env {
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
        }));

        objects.insert(basic_object_metaclass_ref.id, Box::new(RubyObject::Metaclass {
            id: basic_object_metaclass_ref.id,
            of: basic_object_ref.id,
            class: class_ref.to_cell(),
            superclass: class_ref.to_cell(),
        }));

        objects.insert(object_ref.id, Box::new(RubyObject::Class {
            id: object_ref.id,
            name: "Object".to_owned(),
            class: class_ref.to_cell(),
            superclass: basic_object_ref.to_cell(),
        }));

        objects.insert(module_ref.id, Box::new(RubyObject::Class {
            id: module_ref.id,
            name: "Module".to_owned(),
            class: class_ref.to_cell(),
            superclass: object_ref.to_cell(),
        }));

        objects.insert(class_ref.id, Box::new(RubyObject::Class {
            id: class_ref.id,
            name: "Class".to_owned(),
            class: class_ref.to_cell(),
            superclass: module_ref.to_cell(),
        }));

        Env {
            BasicObject: basic_object_ref,
            Object: object_ref,
            Module: module_ref,
            Class: class_ref,

            _objects: RefCell::new(objects),
            ids: ids,
        }
    }

    fn get_object(&self, id: ObjectId) -> &RubyObject {
        let objects = self._objects.borrow();
        let ref_ = &**objects.get(&id).expect("dangling ObjectId");

        // extend lifetime of &RubyObject to that of env
        // WARNING: potentially unsafe - these references *must not* be
        // retained across GCs of the object graph:
        unsafe { ::std::mem::transmute::<&RubyObject, &RubyObject>(ref_) }
    }

    fn put_object(&self, id: ObjectId, object: RubyObject) -> RubyObjectRef {
        self._objects.borrow_mut().insert(id, Box::new(object));
        RubyObjectRef { id: id }
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

                        class.set(metaclass_id);

                        self.put_object(metaclass_id, RubyObject::Metaclass {
                            id: metaclass_id,
                            of: id,
                            class: self.Class.to_cell(),
                            superclass: class.clone(),
                        })
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

                        class.set(metaclass_id);

                        self.put_object(metaclass_id, RubyObject::Metaclass {
                            id: metaclass_id,
                            of: *id,
                            class: class.clone(),
                            // no need to check for None superclass here - BasicObject's metaclass was already
                            // constructed in Env::bootstrap:
                            // TODO - we do need to replace the direct superclass field get with something that
                            // ignores iclasses:
                            superclass: self.metaclass(&RubyObjectRef { id: superclass.get() }).to_cell(),
                        })
                    },
                }
            }
        }
    }

    pub fn name(&self, object: &RubyObjectRef) -> String {
        match *self.get_object(object.id) {
            RubyObject::Object { ref class, .. } => {
                format!("#<{}>", self.name(&RubyObjectRef { id: class.get() }))
            },
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
}

pub enum RubyObject {
    Object {
        id: ObjectId,
        class: Cell<ObjectId>,
    },
    Module {
        id: ObjectId,
        class: Cell<ObjectId>,
        name: String,
        superclass: Cell<ObjectId>,
    },
    Class {
        id: ObjectId,
        class: Cell<ObjectId>,
        name: String,
        superclass: Cell<ObjectId>,
    },
    Metaclass {
        id: ObjectId,
        class: Cell<ObjectId>,
        superclass: Cell<ObjectId>,
        of: ObjectId,
    }
}
