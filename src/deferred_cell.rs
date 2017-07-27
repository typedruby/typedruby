use std::ops::{Deref, DerefMut};

pub struct DeferredCell<T> {
    val: Option<T>
}

impl<T> DeferredCell<T> {
    pub fn new() -> DeferredCell<T> {
        DeferredCell { val: None }
    }

    pub fn set(deferred: &mut DeferredCell<T>, value: T) {
        if let Some(_) = deferred.val {
            panic!("DeferredCell has already been set")
        }

        deferred.val = Some(value)
    }
}

impl<T> Deref for DeferredCell<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.val.as_ref().expect("DeferredCell is unset")
    }
}

impl<T> DerefMut for DeferredCell<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.val.as_mut().expect("DeferredCell is unset")
    }
}
