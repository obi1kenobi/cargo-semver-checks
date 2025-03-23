#![no_std]

// ---- Should be reported ----
pub enum TestTuple {
    WillBecomeTupleLike(i32),
}

pub enum TestUnit {
    WillBecomeUnitLike,
}

pub enum TestMultipleFields {
    WillBecomeTupleLike(i32, f64),
}

pub enum TestBecomeDocHidden {
    #[doc(hidden)]
    WillBecomeTupleLike(i32),
}

pub enum TestBecomeNonExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike(i32),
}

// ---- Should not be reported ----
pub enum TestEmptyStruct {
    EmptyStruct,
}

pub enum TestTupleNonExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike(i32),
}

pub enum TestDocHidden {
    #[doc(hidden)]
    WillBecomeTupleLike(i32),
}

pub enum MultipleStayTheSame {
    StructLike { field: i32 },
    TupleLike(i32),
    UnitLike,
}
