#![no_std]

// ---- Should be reported ----
pub enum TestTuple {
    WillBecomeTupleLike(i32),
    #[non_exhaustive]
    WillBecomeTupleLikeNonExhaustive(i32),
}

pub enum TestUnit {
    WillBecomeUnitLike,
    #[non_exhaustive]
    WillBecomeUnitLikeNonExhaustive,
}

pub enum TestMultipleFields {
    WillBecomeTupleLike(i32, f64),
    #[non_exhaustive]
    WillBecomeTupleLikeNonExhaustive(i32, f64),
}

pub enum TestBecomeDocHidden {
    #[doc(hidden)]
    WillBecomeTupleLike(i32),
    #[non_exhaustive]
    #[doc(hidden)]
    WillBecomeTupleLikeNonExhaustive(i32),
}

pub enum TestBecomeNonExhaustive {
    #[non_exhaustive]
    WillBecomeTupleLike(i32),
}

pub enum TestBecomeExhaustive {
    WillBecomeTupleLike(i32),
}

// ---- Should not be reported ----
pub enum TestEmptyStruct {
    EmptyStruct,
    #[non_exhaustive]
    EmptyStructNonExhaustive {},
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
