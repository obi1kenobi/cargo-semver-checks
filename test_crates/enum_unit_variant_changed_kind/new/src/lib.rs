// ---- Should be reported ----
pub enum TestStruct {
    WillBecomeStructLike{}
}

pub enum TestTuple {
    WillBecomeTupleLike(())
}

pub enum MultipleTest {
    WillBecomeStructLike{},
    WillBecomeTupleLike(()),
    WillStayUnitLike
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
pub enum TestUnit {
    WillStayUnitLike
}

pub enum TestStructNonExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike{}
}

pub enum TestTupleNonExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike(())
}

pub enum MultipleTestNonExhaustive {
    #[non_exhaustive]
    WillBecomeStructLike{},
    #[non_exhaustive]
    WillBecomeTupleLike(()),
    #[non_exhaustive]
    WillStayUnitLike
}

pub enum TestDocHidden {
    #[doc(hidden)]
    WillBecomeStructLike{},
    #[doc(hidden)]
    WillBecomeTupleLike(()),
    #[doc(hidden)]
    WillStayUnitLike
}

pub enum MultipleStayTheSame {
    StructLike{},
    TupleLike(()),
    UnitLike
}
