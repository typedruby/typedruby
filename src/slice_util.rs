use std::ops::Deref;
use std::marker::Sized;

pub struct View<'a, T: 'a + Sized>(pub &'a [T]);

impl<'a, T> View<'a, T> {
    // need to implement first and last manually rather than relying on Deref
    // because &T actually has lifetime 'a, not lifetime of self:
    pub fn first(&self) -> Option<&'a T> {
        self.0.first()
    }

    pub fn last(&self) -> Option<&'a T> {
        self.0.last()
    }

    pub fn consume_front(&mut self) {
        self.0 = &self.0[1..self.0.len()]
    }

    pub fn consume_back(&mut self) {
        self.0 = &self.0[0..(self.0.len() - 1)]
    }
}

impl<'a, T> Deref for View<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub trait Consumer<'a, T: 'a> {
    fn peek(&self) -> Option<&'a T>;
    fn consume(&mut self);
}

pub struct ForwardConsumer<'v, 'a: 'v, T: 'a>(pub &'v mut View<'a, T>);

impl<'v, 'a, T> Consumer<'a, T> for ForwardConsumer<'v, 'a, T> {
    fn peek(&self) -> Option<&'a T> {
        self.0.first()
    }

    fn consume(&mut self) {
        self.0.consume_front();
    }
}

pub struct ReverseConsumer<'v, 'a: 'v, T: 'a>(pub &'v mut View<'a, T>);

impl<'v, 'a, T> Consumer<'a, T> for ReverseConsumer<'v, 'a, T> {
    fn peek(&self) -> Option<&'a T> {
        self.0.last()
    }

    fn consume(&mut self) {
        self.0.consume_back()
    }
}
