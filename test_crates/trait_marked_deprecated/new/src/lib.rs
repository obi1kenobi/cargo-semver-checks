#![no_std]

// These traits did not have the #[deprecated] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[deprecated]
pub trait TraitToDeprecatedTrait {
    fn method(&self);
}

#[deprecated = "This attribute was added"]
pub trait TraitToDeprecatedMessageTrait {
    fn method(&self);
}

// These traits had the #[deprecated] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub trait DeprecatedTraitToTrait {
    fn method(&self);
}

#[deprecated = "This message was added"]
pub trait DeprecatedTraitToDeprecatedMessageTrait {
    fn method(&self);
}

// These traits had the #[deprecated] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub trait DeprecatedMessageTraitToTrait {
    fn method(&self);
}

#[deprecated]
pub trait DeprecatedMessageTraitToDeprecatedTrait {
    fn method(&self);
}

#[deprecated = "This message was changed"]
pub trait DeprecatedMessageTraitToDeprecatedMessageTrait {
    fn method(&self);
}

// This trait is private and should NOT be reported by this rule.

#[deprecated]
trait DeprecatedPrivateTrait {
    fn method(&self);
}
