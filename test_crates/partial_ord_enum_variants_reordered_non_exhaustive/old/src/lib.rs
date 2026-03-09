#![no_std]

// New variant added at the front, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum NewAtFront {
    A,
    B,
    C,
}

// New variant added in the middle, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum InsertedInMiddle {
    A,
    B,
    C,
}

// Multiple new variants added, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MultipleInserted {
    A,
    B,
    C,
}

// Variant removed from the middle, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum RemovedMiddle {
    A,
    B,
    C,
    D,
}

// Same bug on an exhaustive enum, should not trigger (false positive)
#[derive(PartialOrd, PartialEq)]
pub enum ExhaustiveWithNew {
    A,
    B,
    C,
}

// One removed and one added but relative order preserved, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum AddAndRemove {
    A,
    B,
    C,
    D,
}

// Variants genuinely reordered, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum PlainReorder {
    A,
    B,
    C,
}

// Reordered and a new variant added, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum ReorderWithNew {
    A,
    B,
    C,
}

// Reordered after a variant was removed, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum ReorderAfterRemoval {
    A,
    B,
    C,
    D,
}

// First variant moved to the end, should trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MovedToEnd {
    A,
    B,
    C,
    D,
    E,
}

// Two variants swapped, middle one stays, should trigger for the swapped ones
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum TwoSwapped {
    A,
    B,
    C,
}

// Single-variant enum gets a new variant prepended, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum SingleVariant {
    A,
}

// New variants inserted everywhere, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum FilledWithNew {
    A,
    B,
    C,
}

// Some variants genuinely move, others just shift from addition (mixed)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum MixedShiftAndReorder {
    A,
    B,
    C,
    D,
}

// #[doc(hidden)] variant added, shifting public variants, should not trigger (false positive)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum HiddenAdded {
    A,
    B,
    C,
}

// New variant added and two others swapped (false positive on one, false negative on another)
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
pub enum SwapPlusNew {
    A,
    B,
    C,
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
}

// Doesn't derive PartialOrd, should not trigger
#[non_exhaustive]
pub enum NoDerivePartialOrd {
    A,
    B,
    C,
}

// Private enum, should not trigger
#[non_exhaustive]
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    C,
    A,
    B,
}
