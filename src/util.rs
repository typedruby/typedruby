use std::iter::Fuse;

pub enum Or<L, R> {
    Left(L),
    Right(R),
    Both(L, R),
}

impl<L, R> Or<L, R> {
    pub fn append<Lf, Rf>(self, other: Or<L, R>, mut lf: Lf, mut rf: Rf) -> Or<L, R>
        where Lf : FnMut(L, L) -> L,
              Rf : FnMut(R, R) -> R
    {
        match (self, other) {
            (Or::Left(l1), Or::Left(l2)) => Or::Left(lf(l1, l2)),
            (Or::Right(r1), Or::Right(r2)) => Or::Right(rf(r1, r2)),

            (Or::Right(r), Or::Left(l)) |
            (Or::Left(l), Or::Right(r)) => Or::Both(l, r),

            (Or::Left(l1), Or::Both(l2, r)) |
            (Or::Both(l1, r), Or::Left(l2)) => Or::Both(lf(l1, l2), r),

            (Or::Both(l, r1), Or::Right(r2)) |
            (Or::Right(r1), Or::Both(l, r2)) => Or::Both(l, rf(r1, r2)),

            (Or::Both(l1, r1), Or::Both(l2, r2)) => Or::Both(lf(l1, l2), rf(r1, r2)),
        }
    }
}

pub struct ConcatIter<I: Sized, J: Sized> {
    i: I,
    j: J,
}

impl<I, J, T> Iterator for ConcatIter<I, J>
    where I: Sized + Iterator<Item=T>, J: Sized + Iterator<Item=T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.i.next().or_else(|| self.j.next())
    }
}

pub trait IterExt : Iterator {
    fn concat<J>(self, other: J) -> ConcatIter<Fuse<Self>, J::IntoIter>
        where Self: Sized, J: IntoIterator<Item=Self::Item>
    {
        ConcatIter { i: self.fuse(), j: other.into_iter() }
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}
