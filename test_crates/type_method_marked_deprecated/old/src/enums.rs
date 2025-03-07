pub enum NormalEnum {
    A,
    B,
}

impl NormalEnum {
    pub fn method_to_be_deprecated(&self) -> bool {
        matches!(self, NormalEnum::A)
    }

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
    // Should not trigger - enum is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub enum EnumWithHiddenImpl {
    A,
}

#[doc(hidden)]
impl EnumWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub enum EnumWithHiddenMethod {
    A,
}

impl EnumWithHiddenMethod {
    #[doc(hidden)]
    pub fn method_to_be_deprecated(&self) {}
}

// Private enum - should not trigger
enum PrivateEnum {
    A,
}

impl PrivateEnum {
    pub fn method_to_be_deprecated(&self) {}
}

// This enum will become deprecated, but methods stay normal
pub enum EnumBecomesDeprecated {
    A,
    B,
}

impl EnumBecomesDeprecated {
    pub fn method_stays_normal(&self) -> bool {
        matches!(self, EnumBecomesDeprecated::A)
    }

    pub fn another_normal_method(&self) -> &'static str {
        todo!()
    }
}

// Both enum and methods are deprecated
pub enum BothEnumAndMethodDeprecated {
    A,
}

impl BothEnumAndMethodDeprecated {
    pub fn method_to_be_deprecated(&self) {}
}
