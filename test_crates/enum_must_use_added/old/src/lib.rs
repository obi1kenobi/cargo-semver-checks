// These enums did not have the #[must_use] attribute in the old version.
// Addition of the attribute should be reported by this rule.

pub enum EnumToMustUseEnum {
    Bar,
}

pub enum EnumToMustUseMessageEnum {
    Bar,
}


// These enums had the #[must_use] attribute in the old version. Changes of
// the attribute, including deletion, should not be reported by this rule.

#[must_use]
pub enum MustUseEnumToEnum {
    Bar,
}

#[must_use]
pub enum MustUseEnumToMustUseMessageEnum {
    Bar,
}


// These enums had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should not be reported by this rule.

#[must_use = "Foo"]
pub enum MustUseMessageEnumToEnum {
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


// This enum is private and should not be reported by this rule.

enum MustUsePrivateEnum {
    Bar,
}
