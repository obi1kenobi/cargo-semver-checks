// ---- Should be reported ----
pub enum TestStruct {
    #[non_exhaustive]
    WillBecomeStructLike(i32),
}

pub enum TestUnit {
    #[non_exhaustive]
    WillBecomeUnitLike(i32),
}

pub enum MultipleTest {
    #[non_exhaustive]
    WillBecomeStructLike(i32),
    #[non_exhaustive]
    WillBecomeUnitLike(i32),
    #[non_exhaustive]
    WillStayTupleLike(i32),
}

pub enum TestBecomeDocHidden {
    #[non_exhaustive]
    WillBecomeStructLike(i32),
}

pub enum TestBecomeExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike(i32),
}

// ---- Should not be reported ----
pub enum TestTuple {
    #[non_exhaustive]
    WillStayTupleLike(i32),
}

pub enum TestStructExhaustive {
    WillBecomeStructLike(i32),
}

pub enum TestUnitExhaustive {
    WillBecomeUnitLike(i32),
}

pub enum MultipleTestExhaustive {
    WillBecomeStructLike(i32),
    WillBecomeUnitLike(i32),
    WillStayTupleLike(i32),
}

pub enum TestDocHidden {
    #[doc(hidden)]
    #[non_exhaustive]
    WillBecomeStructLike(i32),
    #[doc(hidden)]
    #[non_exhaustive]
    WillBecomeUnitLike(i32),
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
