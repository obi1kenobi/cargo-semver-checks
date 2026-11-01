#![no_std]

// Report: a unit-only enum loses numeric casts.
pub enum Unit {
    Value,
}

impl core::ops::Drop for Unit {
    fn drop(&mut self) {}
}

// Report: integer repr and explicit unit discriminants do not protect casts from Drop.
#[repr(u8)]
pub enum IntegerRepr {
    Unit = 7,
    Tuple(),
    Struct {},
}

impl core::ops::Drop for IntegerRepr {
    fn drop(&mut self) {}
}

// Report: a C representation also permits numeric casts before Drop is added.
#[repr(C)]
pub enum CRepr {
    Unit,
}

impl core::ops::Drop for CRepr {
    fn drop(&mut self) {}
}

// Report: an uninhabited enum can still be cast in a downstream function.
pub enum Empty {}

impl core::ops::Drop for Empty {
    fn drop(&mut self) {}
}

// Do not report: the field prevents casts even when it and its variant are doc-hidden.
pub enum TupleFields {
    #[doc(hidden)]
    Value(#[doc(hidden)] u8),
}

impl core::ops::Drop for TupleFields {
    fn drop(&mut self) {}
}

// Do not report: integer repr exposes a discriminant but does not allow casts with fields.
#[repr(u8)]
pub enum StructFields {
    Value { field: u8 },
}

impl core::ops::Drop for StructFields {
    fn drop(&mut self) {}
}

// Do not report Drop or non-exhaustive cast loss: the explicit tuple discriminant
// already prevents casts, even though it equals the implicit value.
#[repr(u8)]
pub enum ExplicitTuple {
    Unit,
    #[non_exhaustive]
    Empty() = 1,
}

impl core::ops::Drop for ExplicitTuple {
    fn drop(&mut self) {}
}

// No discriminant-loss lint should report: the baseline's doc-hidden empty struct
// variant has an explicit discriminant, which already prevents casts.
#[repr(u8)]
pub enum ExplicitStruct {
    Unit,
    #[doc(hidden)]
    #[non_exhaustive]
    Empty {
        value: u8,
    } = 1,
}

impl core::ops::Drop for ExplicitStruct {
    fn drop(&mut self) {}
}

// Do not report: even a doc-hidden non-exhaustive variant already prevents downstream casts.
pub enum NonExhaustiveVariant {
    Unit,
    #[doc(hidden)]
    #[non_exhaustive]
    Hidden(),
}

impl core::ops::Drop for NonExhaustiveVariant {
    fn drop(&mut self) {}
}

// Do not report: this enum is private.
enum Private {
    Unit,
}

impl core::ops::Drop for Private {
    fn drop(&mut self) {}
}

// Do not report: this enum is outside the public API.
#[doc(hidden)]
pub enum Hidden {
    Unit,
}

impl core::ops::Drop for Hidden {
    fn drop(&mut self) {}
}

// Do not report: adding an unrelated trait named Drop does not affect casts.
pub trait Drop {}
pub enum ShadowDrop {
    Unit,
}
impl Drop for ShadowDrop {}

// Report: the unrelated baseline trait does not prevent casts.
pub enum ShadowDropThenReal {
    Unit,
}
impl Drop for ShadowDropThenReal {}
impl core::ops::Drop for ShadowDropThenReal {
    fn drop(&mut self) {}
}

// Report through the stable public re-export despite the renamed definition
// and the doc-hidden Drop implementation.
#[doc(hidden)]
mod hidden {
    pub enum After {
        Unit,
    }
    #[doc(hidden)]
    impl core::ops::Drop for After {
        fn drop(&mut self) {}
    }
}
pub use hidden::After as Reexported;

// Do not report: the enum was not public API in the baseline.
pub enum PreviouslyHidden {
    Unit,
}
impl core::ops::Drop for PreviouslyHidden {
    fn drop(&mut self) {}
}

// Do not report cast loss for an enum that leaves the public API.
#[doc(hidden)]
pub enum BecomesHidden {
    Unit,
}
impl core::ops::Drop for BecomesHidden {
    fn drop(&mut self) {}
}
