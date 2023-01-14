// These traits did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub trait TraitToMustUseTrait {}

pub trait TraitToMustUseMessageTrait {}


// These traits had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use]
pub trait MustUseTraitToTrait {}

#[must_use]
pub trait MustUseTraitToMustUseMessageTrait {}


// These traits had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

#[must_use = "Foo"]
pub trait MustUseMessageTraitToTrait {}

#[must_use = "Foo"]
pub trait MustUseMessageTraitToMustUseTrait {}

#[must_use = "Foo"]
pub trait MustUseMessageTraitToMustUseMessageTrait {}


// This trait is private and should NOT be reported by this rule.

trait MustUsePrivateTrait {}
