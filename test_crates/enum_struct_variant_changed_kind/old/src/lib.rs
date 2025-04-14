#![no_std]

// ---- Should be reported ----
pub enum TestTuple {
    WillBecomeTupleLike { field: i32 },
    #[non_exhaustive]
    WillBecomeTupleLikeNonExhaustive { field: i32 },
}

pub enum TestUnit {
    WillBecomeUnitLike { field: i32 },
    #[non_exhaustive]
    WillBecomeUnitLikeNonExhaustive { field: i32 },
}

pub enum TestMultipleFields {
    WillBecomeTupleLike { a: i32, b: f64 },
    #[non_exhaustive]
    WillBecomeTupleLikeNonExhaustive { a: i32, b: f64 },
}

pub enum TestBecomeDocHidden {
    WillBecomeTupleLike { field: i32 },
    #[non_exhaustive]
    WillBecomeTupleLikeNonExhaustive { field: i32 },
}

pub enum TestBecomeNonExhaustive {
    WillBecomeTupleLike { field: i32 },
}

pub enum TestBecomeExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike { field: i32 },
}

// ---- Should not be reported ----
pub enum TestEmptyStruct {
    EmptyStruct {},
    #[non_exhaustive]
    EmptyStructNonExhaustive {},
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
