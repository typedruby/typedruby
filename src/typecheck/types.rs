use ast::SourceLoc;
use object::{ObjectGraph, RubyObject};
use typed_arena::Arena;

pub struct Types<'ty, 'env, 'object: 'ty + 'env> {
    arena: &'ty Arena<Type<'ty, 'object>>,
    object: &'env ObjectGraph<'object>,
}

impl<'ty, 'env, 'object: 'ty + 'env> Types<'ty, 'env, 'object> {
    pub fn new(arena: &'ty Arena<Type<'ty, 'object>>, object: &'env ObjectGraph<'object>) -> Types<'ty, 'env, 'object> {
        Types { arena: arena, object: object }
    }

    pub fn new_var(&'ty self, loc: SourceLoc) -> &'ty Type<'ty, 'object> {
        self.arena.alloc(Type::Var { loc: loc, instance: None })
    }
}

pub enum Type<'ty, 'object: 'ty> {
    Instance {
        loc: SourceLoc,
        class: &'object RubyObject<'object>,
        type_parameters: Vec<&'ty Type<'ty, 'object>>,
    },
    Tuple {
        loc: SourceLoc,
        lead: Vec<&'ty Type<'ty, 'object>>,
        splat: Option<&'ty Type<'ty, 'object>>,
        post: Vec<&'ty Type<'ty, 'object>>,
    },
    Union {
        loc: SourceLoc,
        a: &'ty Type<'ty, 'object>,
        b: &'ty Type<'ty, 'object>,
    },
    Any {
        loc: SourceLoc,
    },
    TypeParameter {
        loc: SourceLoc,
        name: String,
    },
    KeywordHash {
        loc: SourceLoc,
        keywords: Vec<(String, &'ty Type<'ty, 'object>)>,
    },
    Proc {
        loc: SourceLoc,
        args: Vec<&'ty Type<'ty, 'object>>,
        block: Option<&'ty Type<'ty, 'object>>,
        retn: &'ty Type<'ty, 'object>,
    },
    Var {
        loc: SourceLoc,
        instance: Option<&'ty Type<'ty, 'object>>,
    }
}
