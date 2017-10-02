pub enum Or<L, R> {
    Left(L),
    Right(R),
    Both(L, R),
}

impl<L, R> Or<L, R> {
    pub fn map_left<F, Lt>(self, f: F) -> Or<Lt, R>
        where F : FnOnce(L) -> Lt
    {
        match self {
            Or::Left(l) => Or::Left(f(l)),
            Or::Both(l, r) => Or::Both(f(l), r),
            Or::Right(r) => Or::Right(r),
        }
    }

    pub fn map_right<F, Rt>(self, f: F) -> Or<L, Rt>
        where F : FnOnce(R) -> Rt
    {
        match self {
            Or::Left(l) => Or::Left(l),
            Or::Both(l, r) => Or::Both(l, f(r)),
            Or::Right(r) => Or::Right(f(r)),
        }
    }

    pub fn append<Lf, Rf>(self, other: Or<L, R>, lf: Lf, rf: Rf) -> Or<L, R>
        where Lf : FnOnce(L, L) -> L,
              Rf : FnOnce(R, R) -> R
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

impl<T> Or<T, T> {
    pub fn flatten<F>(self, f: F) -> T
        where F : FnOnce(T, T) -> T
    {
        match self {
            Or::Left(val) | Or::Right(val) => val,
            Or::Both(l, r) => f(l, r),
        }
    }
}
