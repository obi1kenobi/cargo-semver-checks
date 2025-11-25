#![no_std]

// These traits had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

pub trait MustUseTraitToTrait {}

pub trait MustUseMessageTraitToTrait {}

// These traits did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

#[must_use]
pub trait TraitToMustUseTrait {}

#[must_use = "Foo"]
pub trait TraitToMustUseMessageTrait {}

// These traits change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use = "Foo"]
pub trait MustUseTraitToMustUseMessageTrait {}

#[must_use]
pub trait MustUseMessageTraitToMustUseTrait {}

#[must_use = "Baz"]
pub trait MustUseMessageTraitToMustUseMessageTrait {}

// This trait is private and should NOT be reported by this rule.

trait MustUsePrivateTrait {}
