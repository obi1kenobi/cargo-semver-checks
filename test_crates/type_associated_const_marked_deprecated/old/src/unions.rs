pub union NormalUnion {
    a: i32,
    b: u32,
}

impl NormalUnion {
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
pub union HiddenUnion {
    a: i32,
}

impl HiddenUnion {
    // Should not trigger - union is hidden
    pub const CONST_TO_DEPRECATED: i32 = 0;
}

pub union UnionWithHiddenImpl {
    a: i32,
}

#[doc(hidden)]
impl UnionWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub const CONST_TO_DEPRECATED: i32 = 1;
}

pub union UnionWithHiddenConst {
    a: i32,
}

impl UnionWithHiddenConst {
    #[doc(hidden)]
    pub const CONST_TO_DEPRECATED: i32 = 2;
}

// Will become deprecated
pub union UnionBecomesDeprecated {
    a: i32,
}

impl UnionBecomesDeprecated {
    // Should not trigger since union is deprecated
    pub const CONST_REMAINS_NORMAL: i32 = 3;
}

// Private union - should not trigger
union PrivateUnion {
    a: i32,
}

impl PrivateUnion {
    pub const CONST_TO_DEPRECATED: i32 = 4;
}

// Both union and const will become deprecated
pub union BothBecomeDeprecated {
    a: i32,
}

impl BothBecomeDeprecated {
    pub const CONST_BOTH_DEPRECATED: i32 = 5;
}
