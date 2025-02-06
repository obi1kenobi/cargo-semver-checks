pub union NormalUnion {
    a: i32,
    b: u32,
}

impl NormalUnion {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 42;

    #[deprecated = "Use new_const instead"]
    pub const CONST_WITH_MESSAGE: &'static str = "hello";

    #[deprecated]
    pub const ALREADY_DEPRECATED: u32 = 100;

    pub const STAYS_NORMAL: bool = true;
}

#[doc(hidden)]
pub union HiddenUnion {
    a: i32,
}

impl HiddenUnion {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub union UnionWithHiddenImpl {
    a: i32,
}

#[doc(hidden)]
impl UnionWithHiddenImpl {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub union UnionWithHiddenConst {
    a: i32,
}

impl UnionWithHiddenConst {
    #[doc(hidden)]
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

#[deprecated]
pub union UnionBecomesDeprecated {
    a: i32,
}

impl UnionBecomesDeprecated {
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

union PrivateUnion {
    a: i32,
}

impl PrivateUnion {
    #[deprecated]
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both union and const are deprecated
#[deprecated = "Use NewUnion instead"]
pub union BothBecomeDeprecated {
    a: i32,
}

impl BothBecomeDeprecated {
    #[deprecated]
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
