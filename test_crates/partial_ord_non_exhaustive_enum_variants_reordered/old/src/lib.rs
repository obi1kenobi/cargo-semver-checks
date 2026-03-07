#![no_std]

// Genuine reorder in a #[non_exhaustive] enum - should trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum ReorderedNonExhaustive {
    A,
    B,
    C,
}

// New variants inserted between existing ones, no reorder - should NOT trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum InsertedOnly {
    X,
    Y,
    Z,
}

// New variants inserted AND existing variants reordered - should trigger
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum InsertedAndReordered {
    P,
    Q,
    R,
}

// Not #[non_exhaustive] - should NOT trigger (handled by companion lint)
#[derive(PartialOrd, PartialEq)]
pub enum ExhaustiveReordered {
    A,
    B,
    C,
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
    A,
    B,
    C,
}

// No #[derive(PartialOrd)] - should NOT trigger
#[non_exhaustive]
pub enum NoDerivePartialOrd {
    A,
    B,
    C,
}

// Manual PartialOrd impl - should NOT trigger
#[derive(PartialEq)]
#[non_exhaustive]
pub enum ManualPartialOrd {
    A,
    B,
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
    A,
    B,
    C,
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
pub enum BecomesNonExhaustive {
    A,
    B,
    C,
}

// Mixed variant types - should trigger on reorder
#[derive(PartialOrd, PartialEq)]
#[non_exhaustive]
pub enum MixedVariants {
    Unit,
    Tuple(i32),
    Struct { x: i32 },
}
