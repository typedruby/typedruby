pub type Id = usize;

pub const NULL_ID: Id = 0;

pub struct SeqMap<T> {
    vec: Vec<T>,
}

impl<T> SeqMap<T> {
    pub fn new() -> Self {
        SeqMap { vec: Vec::new() }
    }

    pub fn insert(&mut self, value: Option<T>) -> Id {
        match value {
            None => NULL_ID,
            Some(x) => {
                let id = self.vec.len() + 1;
                self.vec.push(x);
                id
            }
        }
    }

    /// Panics if id is out of bounds
    pub fn get(&self, id: Id) -> Option<&T> {
        if id == 0 {
            None
        } else {
            Some(&self.vec[id - 1])
        }
    }
}
