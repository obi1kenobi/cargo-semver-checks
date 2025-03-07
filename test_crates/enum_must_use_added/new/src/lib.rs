#![no_std]

// These enums did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

#[must_use]
pub enum EnumToMustUseEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum EnumToMustUseMessageEnum {
    Bar,
}


// These enums had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub enum MustUseEnumToEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum MustUseEnumToMustUseMessageEnum {
    Bar,
}


// These enums had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should NOT be reported by this rule.

pub enum MustUseMessageEnumToEnum {
    Bar,
}

#[must_use]
pub enum MustUseMessageEnumToMustUseEnum {
    Bar,
}

#[must_use = "Baz"]
pub enum MustUseMessageEnumToMustUseMessageEnum {
    Bar,
}


// This enum is private and should NOT be reported by this rule.

#[must_use]
enum MustUsePrivateEnum {
    Bar,
}


// This enum was added in the new version of the crate with its attribute.
// It should NOT be reported by this rule to avoid duplicate lints.
// It should be reported as a new pub type that is part of the crate's API.

#[must_use]
pub enum MustUseNewEnum {
    Bar,
}
