#![no_std]

// These enums had the #[must_use] attribute in the old version.
// Removal of the attribute should be reported by this rule.

#[must_use]
pub enum MustUseEnumToEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum MustUseMessageEnumToEnum {
    Bar,
}

// These enums did not have the #[must_use] attribute in the old version.
// Addition of the attribute should NOT be reported by this rule.

pub enum EnumToMustUseEnum {
    Bar,
}

pub enum EnumToMustUseMessageEnum {
    Bar,
}

// These enums change from one form of #[must_use] to another.
// They should NOT be reported by this rule.

#[must_use]
pub enum MustUseEnumToMustUseMessageEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum MustUseMessageEnumToMustUseEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum MustUseMessageEnumToMustUseMessageEnum {
    Bar,
}

// This enum is private and should NOT be reported by this rule.

#[must_use]
enum MustUsePrivateEnum {
    Bar,
}

// This enum existed in the old version and is removed in the new one.
// It should NOT be reported by this rule to avoid duplicate lints.

#[must_use]
pub enum MustUseRemovedEnum {
    Bar,
}
