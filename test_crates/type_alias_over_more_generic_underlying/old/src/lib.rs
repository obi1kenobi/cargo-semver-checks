//! We're going to replace the original type (named like `OriginalType`) with two types:
//! - `pub struct NewType<A, B = ()>`
//! - `pub type OriginalType<A> = NewType<A>`
//!
//! This is not semver-major, since the type alias has:
//! - the same name
//! - the same fields
//! - the same constructibility and pattern-matching behavior.

pub struct OriginalStructOnlyPrivateFields {
    _marker: std::marker::PhantomData<()>,
}

pub struct OriginalStructGeneric<A> {
    _marker: std::marker::PhantomData<A>,
}

pub struct OriginalStructMixOfFields {
    _marker: std::marker::PhantomData<()>,
    pub x: i64,
}

pub struct OriginalStructConstructible {
    pub x: i64,
}

pub struct OriginalStructEmpty {}

#[non_exhaustive]
pub struct OriginalStructConstructibleNonexhaustive {
    pub x: i64,
}

#[non_exhaustive]
pub struct OriginalStructEmptyNonexhaustive {
    pub x: i64,
}

// The enum case is interesting: to add a generic parameter, we need to use it somewhere.
// We'll need to add a variant, so the original enum must have been non-exhaustive
// to avoid an immediate major breaking change.

#[non_exhaustive]
pub enum OriginalEnumNonexhaustive {
    First,
}
