pub struct NormalStruct {
    value: i32,
}

impl NormalStruct {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) -> i32 {
        self.value
    }

    #[deprecated = "Use new_method instead"]
    pub fn method_with_message_to_be_deprecated(&self) -> String {
        "hello".to_string()
    }

    #[deprecated]
    pub fn already_deprecated_method(&self) {}

    pub fn stays_normal(&self) {}
}

#[doc(hidden)]
pub struct HiddenStruct {
    value: i32,
}

impl HiddenStruct {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub struct StructWithHiddenImpl {
    value: i32,
}

#[doc(hidden)]
impl StructWithHiddenImpl {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub struct StructWithHiddenMethod {
    value: i32,
}

impl StructWithHiddenMethod {
    #[doc(hidden)]
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

struct PrivateStruct {
    value: i32,
}

impl PrivateStruct {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

// Struct is deprecated but methods remain unchanged
#[deprecated = "Use NewStruct instead"]
pub struct StructBecomesDeprecated {
    value: i32,
}

impl StructBecomesDeprecated {
    // Methods intentionally not deprecated
    pub fn method_stays_normal(&self) -> i32 {
        self.value
    }

    pub fn another_normal_method(&self) -> String {
        "hello".to_string()
    }
}

// Both struct and methods are deprecated
#[deprecated]
pub struct BothStructAndMethodDeprecated {
    value: i32,
}

impl BothStructAndMethodDeprecated {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}
