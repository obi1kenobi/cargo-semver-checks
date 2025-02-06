pub struct NormalStruct {
    value: i32,
}

impl NormalStruct {
    pub fn method_to_be_deprecated(&self) -> i32 {
        self.value
    }

    pub fn method_with_message_to_be_deprecated(&self) -> String {
        "hello".to_string()
    }

    // Should not trigger - already deprecated
    #[deprecated]
    pub fn already_deprecated_method(&self) {}

    // Should not trigger - stays normal
    pub fn stays_normal(&self) {}
}

#[doc(hidden)]
pub struct HiddenStruct {
    value: i32,
}

impl HiddenStruct {
    // Should not trigger - type is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub struct StructWithHiddenImpl {
    value: i32,
}

#[doc(hidden)]
impl StructWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub struct StructWithHiddenMethod {
    value: i32,
}

impl StructWithHiddenMethod {
    #[doc(hidden)]
    pub fn method_to_be_deprecated(&self) {}
}

// Private struct - should not trigger
struct PrivateStruct {
    value: i32,
}

impl PrivateStruct {
    pub fn method_to_be_deprecated(&self) {}
}

// This struct will become deprecated, but methods stay normal
pub struct StructBecomesDeprecated {
    value: i32,
}

impl StructBecomesDeprecated {
    pub fn method_stays_normal(&self) -> i32 {
        self.value
    }

    pub fn another_normal_method(&self) -> String {
        "hello".to_string()
    }
}
