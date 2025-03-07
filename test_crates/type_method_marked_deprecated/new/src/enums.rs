pub enum NormalEnum {
    A,
    B,
}

impl NormalEnum {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) -> bool {
        matches!(self, NormalEnum::A)
    }

    #[deprecated = "Use to_string instead"]
    pub fn method_with_message_to_be_deprecated(&self) -> &'static str {
        todo!()
    }

    #[deprecated]
    pub fn already_deprecated_method(&self) {}

    pub fn stays_normal(&self) {}
}

#[doc(hidden)]
pub enum HiddenEnum {
    A,
}

impl HiddenEnum {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub enum EnumWithHiddenImpl {
    A,
}

#[doc(hidden)]
impl EnumWithHiddenImpl {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub enum EnumWithHiddenMethod {
    A,
}

impl EnumWithHiddenMethod {
    #[doc(hidden)]
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

enum PrivateEnum {
    A,
}

impl PrivateEnum {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

// Enum is deprecated but methods remain unchanged
#[deprecated = "Use NewEnum instead"]
pub enum EnumBecomesDeprecated {
    A,
    B,
}

impl EnumBecomesDeprecated {
    // Methods intentionally not deprecated
    pub fn method_stays_normal(&self) -> bool {
        matches!(self, EnumBecomesDeprecated::A)
    }

    pub fn another_normal_method(&self) -> &'static str {
        todo!()
    }
}

// Both enum and methods are deprecated
#[deprecated]
pub enum BothEnumAndMethodDeprecated {
    A,
}

impl BothEnumAndMethodDeprecated {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}
