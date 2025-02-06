pub enum NormalEnum {
    A,
    B,
}

impl NormalEnum {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 42;

    #[deprecated = "Use new_const instead"]
    pub const CONST_WITH_MESSAGE: &'static str = "hello";

    #[deprecated]
    pub const ALREADY_DEPRECATED: u32 = 100;

    pub const STAYS_NORMAL: bool = true;
}

#[doc(hidden)]
pub enum HiddenEnum {
    A,
}

impl HiddenEnum {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub enum EnumWithHiddenImpl {
    A,
}

#[doc(hidden)]
impl EnumWithHiddenImpl {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub enum EnumWithHiddenConst {
    A,
}

impl EnumWithHiddenConst {
    #[doc(hidden)]
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

#[deprecated]
pub enum EnumBecomesDeprecated {
    A,
}

impl EnumBecomesDeprecated {
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

enum PrivateEnum {
    A,
}

impl PrivateEnum {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both enum and const are deprecated
#[deprecated = "Use NewEnum instead"]
pub enum BothBecomeDeprecated {
    A,
    B,
}

impl BothBecomeDeprecated {
    #[deprecated]
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
