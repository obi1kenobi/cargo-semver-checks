#![no_std]

// These traits had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

#[must_use]
pub trait MustUseTraitToTrait {}

#[must_use = "Foo"]
pub trait MustUseMessageTraitToTrait {}

// These traits did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

pub trait TraitToMustUseTrait {}

pub trait TraitToMustUseMessageTrait {}

// These traits change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use]
pub trait MustUseTraitToMustUseMessageTrait {}

#[must_use = "Foo"]
pub trait MustUseMessageTraitToMustUseTrait {}

#[must_use = "Foo"]
pub trait MustUseMessageTraitToMustUseMessageTrait {}

// This trait is private and should NOT be reported by this rule.

#[must_use]
trait MustUsePrivateTrait {}

// This trait existed in the old version and is removed in the new one.
// It should NOT be reported by this rule to avoid duplicate lints.

#[must_use]
pub trait MustUseRemovedTrait {}
