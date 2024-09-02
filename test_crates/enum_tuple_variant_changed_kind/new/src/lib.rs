// ---- Should be reported ----
pub enum TestStruct {
    WillBecomeStructLike{}
}

pub enum TestUnit {
    WillBecomeUnitLike
}

pub enum MultipleTest {
    WillBecomeStructLike{},
    WillBecomeUnitLike,
    WillStayTupleLike(()),
}

pub enum TestBecomeDocHidden {
    #[doc(hidden)]
    WillBecomeStructLike{}
}

pub enum TestBecomeNonExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike{}
}

// ---- Should not be reported ----
pub enum TestTuple {
    WillStayTupleLike(())
}

pub enum TestStructNonExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike{}
}

pub enum TestUnitNonExhaustive {
    #[non_exhaustive]
    WillBecomeUnitLike
}

pub enum MultipleTestNonExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike{},
    #[non_exhaustive]
    WillBecomeUnitLike,
    #[non_exhaustive]
    WillStayTupleLike(()),
}

pub enum TestDocHidden {
    #[doc(hidden)]
    WillBecomeStructLike{},
    #[doc(hidden)]
    WillBecomeUnitLike,
    #[doc(hidden)]
    WillStayTupleLike(()),
}

pub enum MultipleStayTheSame {
    StructLike{},
    TupleLike(()),
    UnitLike
}
