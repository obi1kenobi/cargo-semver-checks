#![no_std]

// New variant added at the front, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum NewAtFront {
    New,
    A,
    B,
    C,
}

// New variant added in the middle, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum InsertedInMiddle {
    A,
    New,
    B,
    C,
}

// Multiple new variants added, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MultipleInserted {
    X,
    A,
    Y,
    B,
    C,
}

// Variant removed from the middle, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum RemovedMiddle {
    A,
    C,
    D,
}

// Same bug on an exhaustive enum, should not trigger (false positive)
#[derive(PartialOrd, PartialEq)]
pub enum ExhaustiveWithNew {
    A,
    B,
    New,
    C,
}

// One removed and one added but relative order preserved, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum AddAndRemove {
    X,
    A,
    C,
    D,
}

// Variants genuinely reordered, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum PlainReorder {
    C,
    A,
    B,
}

// Reordered and a new variant added, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum ReorderWithNew {
    New,
    C,
    A,
    B,
}

// Reordered after a variant was removed, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum ReorderAfterRemoval {
    C,
    D,
    A,
}

// First variant moved to the end, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MovedToEnd {
    B,
    C,
    D,
    E,
    A,
}

// Two variants swapped, middle one stays, should trigger for the swapped ones
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum TwoSwapped {
    C,
    B,
    A,
}

// Single-variant enum gets a new variant prepended, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum SingleVariant {
    New,
    A,
}

// New variants inserted everywhere, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum FilledWithNew {
    W,
    A,
    X,
    B,
    Y,
    C,
    Z,
}

// Some variants genuinely move, others just shift from addition (mixed)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MixedShiftAndReorder {
    New,
    C,
    A,
    B,
    D,
}

// #[doc(hidden)] variant added, shifting public variants, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum HiddenAdded {
    #[doc(hidden)]
    Hidden,
    A,
    B,
    C,
}

// New variant added and two others swapped (false positive on one, false negative on another)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum SwapPlusNew {
    New,
    A,
    C,
    B,
}

// Nothing changed, should not trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum Unchanged {
    A,
    B,
    C,
}

// Variant added at the end only, should not trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum AppendedAtEnd {
    A,
    B,
    C,
    New,
}

// Doesn't derive PartialOrd, should not trigger
#[non_exhaustive]
pub enum NoDerivePartialOrd {
    C,
    A,
    B,
}

// Private enum, should not trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    C,
    A,
    B,
}
