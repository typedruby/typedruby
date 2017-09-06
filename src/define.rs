use std::cell::RefCell;
use std::rc::Rc;

use ast::{Id, Node};
use object::{RubyObject, Scope};

#[derive(Copy,Clone,Debug)]
pub enum MethodVisibility {
    Public,
    Protected,
    Private,
}

#[derive(Debug)]
pub enum MethodDef<'object> {
    Def {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: String,
        node: Rc<Node>,
        scope: Rc<Scope<'object>>,
    },
    Alias {
        module: &'object RubyObject<'object>,
        to: Id,
        from: Id,
    },
    AttrReader {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
    },
    AttrWriter {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
    },
    SetVisi {
        module: &'object RubyObject<'object>,
        visi: MethodVisibility,
        name: Id,
    },
    ModuleFunc {
        module: &'object RubyObject<'object>,
        name: Id,
    }
}

#[derive(Debug)]
pub struct IvarDef<'object> {
    pub module: &'object RubyObject<'object>,
    pub name: Id,
    pub type_node: Rc<Node>,
    pub scope: Rc<Scope<'object>>,
}

#[derive(Debug)]
pub struct Definitions<'object> {
    methods: RefCell<Vec<MethodDef<'object>>>,
    ivars: RefCell<Vec<IvarDef<'object>>>,
}

impl<'object> Definitions<'object> {
    pub fn new() -> Self {
        Definitions {
            methods: RefCell::new(Vec::new()),
            ivars: RefCell::new(Vec::new()),
        }
    }

    pub fn define_method(&self, method: MethodDef<'object>) {
        self.methods.borrow_mut().push(method);
    }

    pub fn define_ivar(&self, ivar: IvarDef<'object>) {
        self.ivars.borrow_mut().push(ivar);
    }
}
