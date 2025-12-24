#![no_std]
// ^^^^^^^ This is used to massively decrease the generated rustdoc JSON size.
// You can remove it if your test specifically requires `std` functionality.
//
// Usually, you can avoid depending on `std` by importing the same types from `core` instead:
// `std::fmt::Debug` is the same as `core::fmt::Debug`,
// `std::marker::PhantomData` is the same as `core::marker::PhantomData` etc.
//
// Similarly, unless you specifically need `String` for a test,
// you can usually avoid it by using `&'static str` instead.

#[derive(Clone, Copy)]
pub struct AddedDerivedCopy {
    pub value: u8,
}

#[derive(Clone)]
pub struct AddedManualCopy(pub u8);

impl Copy for AddedManualCopy {}

#[derive(Clone, Copy)]
pub struct HiddenCopy(pub u8);

#[derive(Clone, Copy)]
struct PrivateCopyCandidate(pub u8);
