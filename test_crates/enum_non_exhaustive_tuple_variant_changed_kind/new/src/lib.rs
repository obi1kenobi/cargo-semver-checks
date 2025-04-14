// ---- Should be reported ----
pub enum TestStruct {
    #[non_exhaustive]
    WillBecomeStructLike { f: i32 },
}

pub enum TestUnit {
    #[non_exhaustive]
    WillBecomeUnitLike,
}

pub enum MultipleTest {
    #[non_exhaustive]
    WillBecomeStructLike { f: i32 },
    #[non_exhaustive]
    WillBecomeUnitLike,
    #[non_exhaustive]
    WillStayTupleLike(i32),
}

pub enum TestBecomeDocHidden {
    #[doc(hidden)]
    #[non_exhaustive]
    WillBecomeStructLike { f: i32 },
}

pub enum TestBecomeExhaustive {
    WillBecomeStructLike { f: i32 },
}

// ---- Should not be reported ----
pub enum TestTuple {
    #[non_exhaustive]
    WillStayTupleLike(()),
}

pub enum TestStructExhaustive {
    WillBecomeStructLike { f: i32 },
}

pub enum TestUnitExhaustive {
    WillBecomeUnitLike,
}

pub enum MultipleTestExhaustive {
    WillBecomeStructLike { f: i32 },
    WillBecomeUnitLike,
    WillStayTupleLike(i32),
}

pub enum TestDocHidden {
    #[doc(hidden)]
    #[non_exhaustive]
    WillBecomeStructLike { f: i32 },
    #[doc(hidden)]
    #[non_exhaustive]
    WillBecomeUnitLike,
    #[doc(hidden)]
    #[non_exhaustive]
    WillStayTupleLike(i32),
}

pub enum MultipleStayTheSame {
    #[non_exhaustive]
    StructLike {},
    #[non_exhaustive]
    TupleLike(i32),
    #[non_exhaustive]
    UnitLike,
}
