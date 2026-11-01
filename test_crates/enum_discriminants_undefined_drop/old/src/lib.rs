#![no_std]

// Report: a unit-only enum loses numeric casts.
pub enum Unit {
    Value,
}

// Report: integer repr and explicit unit discriminants do not protect casts from Drop.
#[repr(u8)]
pub enum IntegerRepr {
    Unit = 7,
    Tuple(),
    Struct {},
}

// Report: a C representation also permits numeric casts before Drop is added.
#[repr(C)]
pub enum CRepr {
    Unit,
}

// Report: an uninhabited enum can still be cast in a downstream function.
pub enum Empty {}

// Do not report: the field prevents casts even when it and its variant are doc-hidden.
pub enum TupleFields {
    #[doc(hidden)]
    Value(#[doc(hidden)] u8),
}

// Do not report: integer repr exposes a discriminant but does not allow casts with fields.
#[repr(u8)]
pub enum StructFields {
    Value { field: u8 },
}

// Do not report Drop or non-exhaustive cast loss: the explicit tuple discriminant
// already prevents casts, even though it equals the implicit value.
#[repr(u8)]
pub enum ExplicitTuple {
    Unit,
    Empty() = 1,
}

// Do not report: the doc-hidden empty struct variant has an explicit discriminant,
// which already prevents casts.
#[repr(u8)]
pub enum ExplicitStruct {
    Unit,
    #[doc(hidden)]
    Empty {} = 1,
}

// Do not report: even a doc-hidden non-exhaustive variant already prevents downstream casts.
pub enum NonExhaustiveVariant {
    Unit,
    #[doc(hidden)]
    #[non_exhaustive]
    Hidden(),
}

// Do not report: this enum is private.
enum Private {
    Unit,
}

// Do not report: this enum is outside the public API.
#[doc(hidden)]
pub enum Hidden {
    Unit,
}

// Do not report: adding an unrelated trait named Drop does not affect casts.
pub trait Drop {}
pub enum ShadowDrop {
    Unit,
}

// Report: the unrelated baseline trait does not prevent casts.
pub enum ShadowDropThenReal {
    Unit,
}
impl Drop for ShadowDropThenReal {}

// Report through the stable public re-export even though the definition is renamed.
#[doc(hidden)]
mod hidden {
    pub enum Before {
        Unit,
    }
}
pub use hidden::Before as Reexported;

// Do not report: the enum was not public API in the baseline.
#[doc(hidden)]
pub enum PreviouslyHidden {
    Unit,
}

// Do not report cast loss for an enum that leaves the public API.
pub enum BecomesHidden {
    Unit,
}
