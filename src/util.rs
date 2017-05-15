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
