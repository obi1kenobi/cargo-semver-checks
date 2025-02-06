pub enum NormalEnum {
    A,
    B,
}

impl NormalEnum {
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
pub enum HiddenEnum {
    A,
}

impl HiddenEnum {
    // Should not trigger - enum is hidden
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub enum EnumWithHiddenImpl {
    A,
}

#[doc(hidden)]
impl EnumWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub enum EnumWithHiddenConst {
    A,
}

impl EnumWithHiddenConst {
    #[doc(hidden)]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

// Will become deprecated
pub enum EnumBecomesDeprecated {
    A,
}

impl EnumBecomesDeprecated {
    // Should not trigger since enum is deprecated
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

// Private enum - should not trigger
enum PrivateEnum {
    A,
}

impl PrivateEnum {
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both enum and const will become deprecated
pub enum BothBecomeDeprecated {
    A,
    B,
}

impl BothBecomeDeprecated {
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
