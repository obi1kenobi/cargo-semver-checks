#![no_std]

// Genuine reorder in a #[non_exhaustive] enum - should trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum ReorderedNonExhaustive {
    C,
    A,
    B,
}

// New variants inserted between existing ones, no reorder - should NOT trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum InsertedOnly {
    X,
    New1,
    Y,
    New2,
    Z,
}

// New variants inserted AND existing variants reordered - should trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum InsertedAndReordered {
    R,
    New1,
    P,
    Q,
}

// Not #[non_exhaustive] - should NOT trigger (handled by companion lint)
#[derive(PartialOrd, PartialEq)]
pub enum ExhaustiveReordered {
    C,
    A,
    B,
}

// #[non_exhaustive] but no changes - should NOT trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum Unchanged {
    A,
    B,
    C,
}

// Private enum - should NOT trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
enum PrivateReordered {
    C,
    A,
    B,
}

// No #[derive(PartialOrd)] - should NOT trigger
#[non_exhaustive]
pub enum NoDerivePartialOrd {
    C,
    A,
    B,
}

// Manual PartialOrd impl - should NOT trigger
#[derive(PartialEq)]
#[non_exhaustive]
pub enum ManualPartialOrd {
    B,
    A,
}

impl PartialOrd for ManualPartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match (self, other) {
            (Self::A, Self::A) => Some(core::cmp::Ordering::Equal),
            (Self::A, Self::B) => Some(core::cmp::Ordering::Less),
            (Self::B, Self::A) => Some(core::cmp::Ordering::Greater),
            (Self::B, Self::B) => Some(core::cmp::Ordering::Equal),
        }
    }
}

// #[doc(hidden)] enum - should NOT trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum DocHiddenReordered {
    C,
    A,
    B,
}

// Enum with doc(hidden) variants - should NOT trigger for hidden variants
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum HasDocHiddenVariants {
    A,
    #[doc(hidden)]
    B,
    C,
}

// Enum that becomes #[non_exhaustive] in current with reordering - should trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum BecomesNonExhaustive {
    C,
    A,
    B,
}

// Mixed variant types - should trigger on reorder
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum MixedVariants {
    Struct { x: i32 },
    Unit,
    Tuple(i32),
}
