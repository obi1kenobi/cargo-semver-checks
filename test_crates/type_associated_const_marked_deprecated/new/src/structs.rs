pub struct NormalStruct {
    value: i32,
}

impl NormalStruct {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 42;

    #[deprecated = "Use new_const instead"]
    pub const CONST_WITH_MESSAGE: &'static str = "hello";

    #[deprecated]
    pub const ALREADY_DEPRECATED: u32 = 100;

    pub const STAYS_NORMAL: bool = true;
}

#[doc(hidden)]
pub struct HiddenStruct {
    value: i32,
}

impl HiddenStruct {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub struct StructWithHiddenImpl {
    value: i32,
}

#[doc(hidden)]
impl StructWithHiddenImpl {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub struct StructWithHiddenConst {
    value: i32,
}

impl StructWithHiddenConst {
    #[doc(hidden)]
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

#[deprecated]
pub struct StructBecomesDeprecated {
    value: i32,
}

impl StructBecomesDeprecated {
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

struct PrivateStruct {
    value: i32,
}

impl PrivateStruct {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both struct and const are deprecated
#[deprecated = "Use NewStruct instead"]
pub struct BothBecomeDeprecated {
    value: i32,
}

impl BothBecomeDeprecated {
    #[deprecated]
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
