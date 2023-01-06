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
// the attribute, including deletion, should not be reported by this rule.

pub enum MustUseEnumToEnum {
    Bar,
}

#[must_use = "Foo"]
pub enum MustUseEnumToMustUseMessageEnum {
    Bar,
}


// These enums had the #[must_use] attribute in the old version.
// They also included the user-defined warning message. Changes of
// the attribute, including deletion, should not be reported by this rule.

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
