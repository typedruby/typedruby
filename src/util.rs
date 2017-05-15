pub enum Or<L, R> {
    Left(L),
    Right(R),
    Both(L, R),
}

impl<L, R> Or<L, R> {
    pub fn map_left<F, Lm>(self, mut f: F) -> Or<Lm, R>
        where F : FnMut(L) -> Lm
    {
        match self {
            Or::Left(l) => Or::Left(f(l)),
            Or::Right(r) => Or::Right(r),
            Or::Both(l, r) => Or::Both(f(l), r),
        }
    }
}
