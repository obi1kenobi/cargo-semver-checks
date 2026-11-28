#![no_std]

#[derive(Debug, Clone)]
pub struct DebugFoo;

#[derive(Clone, Copy)]
pub enum CopyBar {
    Var,
}

#[derive(PartialEq, Eq)]
pub struct EqFoo;

// The following is not a semver issue: it's not breaking to replace
// a derived impl with a hand-impl of the same trait.

#[derive(PartialEq, Eq)]
pub struct ManualEqFoo;

pub trait Debug {}

// Mark the local `Debug` impl as a custom derive would.
// For `DebugFoo`, the lint should report only the loss of `core::fmt::Debug`.
#[automatically_derived]
impl Debug for DebugFoo {}
