use gc::{Gc, GcCell};

type RubyObjectRef = Gc<GcCell<RubyObject>>;

#[derive(Trace,Finalize)]
struct BasicEnv {
    basic_object: RubyObjectRef,
    object: RubyObjectRef,
    module: RubyObjectRef,
    class: RubyObjectRef,
}

#[derive(Trace,Finalize)]
pub struct Env {
    pub basic_object: RubyObjectRef,
    pub object: RubyObjectRef,
    pub module: RubyObjectRef,
    pub class: RubyObjectRef,
}

fn mkref(object: RubyObject) -> RubyObjectRef {
    Gc::new(GcCell::new(object))
}

impl Env {
    fn bootstrap() -> BasicEnv {
        fn bootstrap_class(name: &str) -> Gc<GcCell<RubyObject>> {
            mkref(RubyObject::Class {
                _name: name.to_owned(),
                _class: None,
                _superclass: None,
            })
        }

        let basic_object = bootstrap_class("BasicObject");
        let object = bootstrap_class("Object");
        let module = bootstrap_class("Module");
        let class = bootstrap_class("Class");

        fn set_class(object: &mut RubyObject, class: &Gc<GcCell<RubyObject>>) {
            if let RubyObject::Class { ref mut _class, .. } = *object {
                *_class = Some(class.clone());
            }
        }

        fn set_superclass(object: &mut RubyObject, superclass: &Gc<GcCell<RubyObject>>) {
            if let RubyObject::Class { ref mut _superclass, .. } = *object {
                *_superclass = Some(superclass.clone());
            }
        }

        // The superclass of a class's metaclass is the metaclass of the
        // class's superclass. BasicObject's metaclass is slightly different
        // however, because BasicObject does not have a superclass. In this
        // case, BasicObject's metaclass's superclass is Class itself.
        set_class(&mut *basic_object.borrow_mut(), &mkref(RubyObject::Metaclass {
            _of: basic_object.clone(),
            _class: Some(class.clone()),
            _superclass: Some(class.clone()),
        }));

        set_class(&mut *object.borrow_mut(), &class);
        set_class(&mut *module.borrow_mut(), &class);
        set_class(&mut *class.borrow_mut(), &class);

        set_superclass(&mut *object.borrow_mut(), &basic_object);
        set_superclass(&mut *module.borrow_mut(), &object);
        set_superclass(&mut *class.borrow_mut(), &module);

        BasicEnv {
            basic_object: basic_object,
            object: object,
            module: module,
            class: class,
        }
    }

    pub fn new() -> Env {
        let basic_env = Env::bootstrap();

        Env {
            basic_object: basic_env.basic_object.clone(),
            object: basic_env.object.clone(),
            module: basic_env.module.clone(),
            class: basic_env.class.clone(),
        }
    }

    pub fn metaclass(&self, object: RubyObjectRef) -> RubyObjectRef {
        match *object.borrow_mut() {
            RubyObject::Object { ref mut _class, .. } |
            RubyObject::Module { ref mut _class, .. } => {
                let cell = _class.as_ref().unwrap();

                match *cell.borrow() {
                    RubyObject::Metaclass { .. } => cell.clone(),
                    _ => mkref(RubyObject::Metaclass {
                        _of: object.clone(),
                        _class: Some(self.class.clone()),
                        _superclass: Some(object.borrow().class()),
                    })
                }
            },
            RubyObject::Class { ref mut _class, .. } |
            RubyObject::Metaclass { ref mut _class, .. } => {
                let cell = _class.as_ref().unwrap();

                match *cell.borrow() {
                    RubyObject::Metaclass { .. } => cell.clone(),
                    _ => {
                        let class = object.borrow().raw_class();
                        let superclass = self.metaclass(
                            // no need to check for None superclass here -
                            // BasicObject's metaclass was already constructed
                            // in Env::bootstrap():
                            object.borrow().superclass().unwrap()
                        );
                        mkref(RubyObject::Metaclass {
                            _of: object.clone(),
                            _class: Some(class),
                            _superclass: Some(superclass),
                        })
                    },
                }
            }
        }
    }
}

#[derive(Trace,Finalize)]
pub enum RubyObject {
    Object {
        _class: Option<Gc<GcCell<RubyObject>>>,
    },
    Module {
        _class: Option<Gc<GcCell<RubyObject>>>,
        _name: String,
        _superclass: Option<Gc<GcCell<RubyObject>>>,
    },
    Class {
        _class: Option<Gc<GcCell<RubyObject>>>,
        _name: String,
        _superclass: Option<Gc<GcCell<RubyObject>>>,
    },
    Metaclass {
        _class: Option<Gc<GcCell<RubyObject>>>,
        _superclass: Option<Gc<GcCell<RubyObject>>>,
        _of: Gc<GcCell<RubyObject>>,
    }
}

impl RubyObject {
    pub fn raw_class(&self) -> RubyObjectRef {
        let cl = match *self {
            RubyObject::Object    { _class: ref class, .. } => class,
            RubyObject::Module    { _class: ref class, .. } => class,
            RubyObject::Class     { _class: ref class, .. } => class,
            RubyObject::Metaclass { _class: ref class, .. } => class,
        };

        match cl {
            &None => panic!("all objects must have classes after initial bootstrap!"),
            &Some(ref cell) => cell.clone(),
        }
    }

    pub fn class(&self) -> RubyObjectRef {
        self.raw_class()
    }

    pub fn name(&self) -> String {
        match *self {
            RubyObject::Object { .. } => {
                let name = {
                    let cl = self.class();
                    { let b = cl.borrow(); b.name() }
                };
                format!("#<{}>", name)
            },
            RubyObject::Module { _name: ref name, .. } =>
                name.clone(),
            RubyObject::Class { _name: ref name, .. } =>
                name.clone(),
            RubyObject::Metaclass { _of: ref of, .. } =>
                format!("Class::[{}]", of.borrow().name()),
        }
    }

    pub fn superclass(&self) -> Option<RubyObjectRef> {
        let cell = match *self {
            RubyObject::Object { .. } =>
                panic!("called superclass on RubyObject::Object!"),
            RubyObject::Module { _superclass: ref superclass, .. } =>
                superclass,
            RubyObject::Class { _superclass: ref superclass, .. } =>
                superclass,
            RubyObject::Metaclass { _superclass: ref superclass, .. } =>
                superclass,
        };

        cell.clone()
    }

    pub fn of(&self) -> RubyObjectRef {
        match *self {
            RubyObject::Object { .. } =>
                panic!("called superclass on RubyObject::Object!"),
            RubyObject::Module { .. } =>
                panic!("called superclass on RubyObject::Module!"),
            RubyObject::Class { .. } =>
                panic!("called superclass on RubyObject::Class!"),
            RubyObject::Metaclass { _of: ref of, .. } =>
                of.clone(),
        }
    }
}
