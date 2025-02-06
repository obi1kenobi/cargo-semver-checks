pub union NormalUnion {
    a: i32,
    b: u32,
}

impl NormalUnion {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) -> i32 {
        unsafe { self.a }
    }

    #[deprecated = "Use get_b instead"]
    pub fn method_with_message_to_be_deprecated(&self) -> u32 {
        unsafe { self.b }
    }

    #[deprecated]
    pub fn already_deprecated_method(&self) {}

    pub fn stays_normal(&self) {}
}

#[doc(hidden)]
pub union HiddenUnion {
    a: i32,
}

impl HiddenUnion {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub union UnionWithHiddenImpl {
    a: i32,
}

#[doc(hidden)]
impl UnionWithHiddenImpl {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

pub union UnionWithHiddenMethod {
    a: i32,
}

impl UnionWithHiddenMethod {
    #[doc(hidden)]
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

union PrivateUnion {
    a: i32,
}

impl PrivateUnion {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}

// Enum is deprecated but methods remain unchanged
#[deprecated = "Use NewUnion instead"]
pub union UnionBecomesDeprecated {
    a: i32,
    b: u32,
}

impl UnionBecomesDeprecated {
    // Methods intentionally not deprecated
    pub fn method_stays_normal(&self) -> i32 {
        unsafe { self.a }
    }

    pub fn another_normal_method(&self) -> u32 {
        unsafe { self.b }
    }
}

// Both union and methods are deprecated
#[deprecated]
pub union BothUnionAndMethodDeprecated {
    a: i32,
    b: u32,
}

impl BothUnionAndMethodDeprecated {
    #[deprecated]
    pub fn method_to_be_deprecated(&self) {}
}
