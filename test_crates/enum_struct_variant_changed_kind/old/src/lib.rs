#![no_std]

// ---- Should be reported ----
pub enum TestTuple {
    WillBecomeTupleLike { field: i32 },
}

pub enum TestUnit {
    WillBecomeUnitLike { field: i32 },
}

pub enum TestMultipleFields {
    WillBecomeTupleLike { a: i32, b: f64 },
}

pub enum TestBecomeDocHidden {
    WillBecomeTupleLike { field: i32 },
}

pub enum TestBecomeNonExhaustive {
    WillBecomeTupleLike { field: i32 },
}

// ---- Should not be reported ----
pub enum TestEmptyStruct {
    EmptyStruct {},
}

pub enum TestTupleNonExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike { field: i32 },
}

pub enum TestDocHidden {
    #[doc(hidden)]
    WillBecomeTupleLike { field: i32 },
}

pub enum MultipleStayTheSame {
    StructLike { field: i32 },
    TupleLike(i32),
    UnitLike,
}
