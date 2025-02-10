pub struct NormalStruct {
    value: i32,
}

impl NormalStruct {
    // Will become deprecated
    pub const CONST_TO_DEPRECATED: i32 = 42;

    // Will become deprecated with message
    pub const CONST_WITH_MESSAGE: &'static str = "hello";

    // Already deprecated - should not trigger
    #[deprecated]
    pub const ALREADY_DEPRECATED: u32 = 100;

    // Stays normal
    pub const STAYS_NORMAL: bool = true;
}

#[doc(hidden)]
pub struct HiddenStruct {
    value: i32,
}

impl HiddenStruct {
    // Should not trigger - type is hidden
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub struct StructWithHiddenImpl {
    value: i32,
}

#[doc(hidden)]
impl StructWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub struct StructWithHiddenConst {
    value: i32,
}

impl StructWithHiddenConst {
    #[doc(hidden)]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

// Will become deprecated
pub struct StructBecomesDeprecated {
    value: i32,
}

impl StructBecomesDeprecated {
    // Should not trigger since type is deprecated
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

// Private struct - should not trigger
struct PrivateStruct {
    value: i32,
}

impl PrivateStruct {
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both struct and const will become deprecated
pub struct BothBecomeDeprecated {
    value: i32,
}

impl BothBecomeDeprecated {
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
