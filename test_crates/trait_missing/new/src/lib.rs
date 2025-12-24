#![no_std]

trait AddedPrivateTrait {}

pub trait AddedPubTrait {}

// This trait stops being unsafe. It is not removed and should not be reported.
pub trait TraitStopsBeingUnsafe {}

// This trait becomes unsafe. It is not removed and should not be reported.
pub unsafe trait TraitBecomesUnsafe {}

// This item replaces a trait but is doc-hidden. The trait removal should still be reported.
#[doc(hidden)]
pub struct DocHiddenReplacement;
