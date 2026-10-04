//! We're going to replace the original type (named like `OriginalType`) with two types:
//! - `pub struct NewType<A, B = ()>`
//! - `pub type OriginalType<A> = NewType<A>`
//!
//! This is not semver-major, since the type alias has:
//! - the same name
//! - the same fields/variants
//! - the same constructibility and pattern-matching behavior.

pub struct NewStructOnlyPrivateFields<A> {
    _marker: std::marker::PhantomData<A>,
}

pub type OriginalStructOnlyPrivateFields = NewStructOnlyPrivateFields<()>;

// ---

pub struct NewStructGeneric<A, B = ()> {
    _marker: std::marker::PhantomData<(A, B)>,
}

pub type OriginalStructGeneric<A> = NewStructGeneric<A>;

// ---

pub struct NewStructMixOfFields<A> {
    _marker: std::marker::PhantomData<A>,
    pub x: i64,
}

pub type OriginalStructMixOfFields = NewStructMixOfFields<()>;

// ---

pub struct NewStructConstructible<T> {
    pub x: T,
}

pub type OriginalStructConstructible = NewStructConstructible<i64>;

// ---

pub struct NewStructEmpty<A> {
    _marker: std::marker::PhantomData<A>,
}

pub type OriginalStructEmpty = NewStructEmpty<()>;

// ---

#[non_exhaustive]
pub struct NewStructConstructibleNonexhaustive<A> {
    pub x: A,
}

pub type OriginalStructConstructibleNonexhaustive = NewStructConstructibleNonexhaustive<i64>;

// ---

#[non_exhaustive]
pub struct NewStructEmptyNonexhaustive<A> {
    pub x: A,
}

pub type OriginalStructEmptyNonexhaustive = NewStructEmptyNonexhaustive<i64>;

// ---
// The enum case is interesting: to add a generic parameter, we need to use it somewhere.
// We'll need to add a variant, so the original enum must have been non-exhaustive
// to avoid an immediate major breaking change.

#[non_exhaustive]
pub enum NewEnumNonexhaustive<A> {
    First,
    Second(A),
}

pub type OriginalEnumNonexhaustive = NewEnumNonexhaustive<()>;
