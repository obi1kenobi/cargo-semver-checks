pub union NormalUnion {
    a: i32,
    b: u32,
}

impl NormalUnion {
    pub fn method_to_be_deprecated(&self) -> i32 {
        unsafe { self.a }
    }

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
    // Should not trigger - union is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub union UnionWithHiddenImpl {
    a: i32,
}

#[doc(hidden)]
impl UnionWithHiddenImpl {
    // Should not trigger - impl is hidden
    pub fn method_to_be_deprecated(&self) {}
}

pub union UnionWithHiddenMethod {
    a: i32,
}

impl UnionWithHiddenMethod {
    #[doc(hidden)]
    pub fn method_to_be_deprecated(&self) {}
}

// Private union - should not trigger
union PrivateUnion {
    a: i32,
}

impl PrivateUnion {
    pub fn method_to_be_deprecated(&self) {}
}

// This union will become deprecated, but methods stay normal
pub union UnionBecomesDeprecated {
    a: i32,
    b: u32,
}

impl UnionBecomesDeprecated {
    pub fn method_stays_normal(&self) -> i32 {
        unsafe { self.a }
    }

    pub fn another_normal_method(&self) -> u32 {
        unsafe { self.b }
    }
}
