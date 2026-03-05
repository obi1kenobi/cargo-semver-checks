#![no_std]

//TRUE POSITIVES: these methods are now const, but were not const in the old version, so should be reported.

pub struct PubStruct;

impl PubStruct {
    /// This method became const, but was not const in the old version, so should be reported.
    pub const fn becomes_const(&self) -> i64 {
        42
    }

    /// This associated fn became const, but was not const in the old version, so should be reported.
    pub const fn assoc_fn_becomes_const(x: i64, y: i64) -> i64 {
        x + y
    }
}

pub enum PubEnum {
    A,
    B,
}

impl PubEnum {
    /// Enum method became const, but was not const in the old version, so should be reported.
    pub const fn becomes_const(&self) -> bool {
        match self {
            PubEnum::A => true,
            PubEnum::B => false,
        }
    }
}

pub union PubUnion {
    pub f1: u32,
    pub f2: f32,
}

impl PubUnion {
    /// Union associated fn became const, but was not const in the old version, so should be reported.
    pub const fn new(val: u32) -> Self {
        PubUnion { f1: val }
    }
}

//FALSE POSITIVES: these methods are const in both versions, so should NOT be reported.

pub struct AlreadyConst;

impl AlreadyConst {
    /// Already const in both versions, so should NOT be reported.
    pub const fn already_const_method(&self) -> i64 {
        0
    }
}

pub struct NeverConst;

impl NeverConst {
    /// Not const in either version, so should NOT be reported.
    pub fn stays_non_const(&self) -> i64 {
        1
    }
}

// Private struct, so should NOT be reported.
struct PrivateStruct;

impl PrivateStruct {
    pub const fn becomes_const(&self) -> i64 {
        0
    }
}

pub struct PubStructPrivateMethod;

impl PubStructPrivateMethod {
    // Private method, so should NOT be reported.
    const fn private_becomes_const(&self) -> i64 {
        0
    }
}

// #[doc(hidden)] struct, so should NOT be reported.
#[doc(hidden)]
pub struct DocHiddenStruct;

impl DocHiddenStruct {
    pub const fn becomes_const(&self) -> i64 {
        0
    }
}

pub struct PubStructDocHiddenMethod;

impl PubStructDocHiddenMethod {
    // #[doc(hidden)] method, so should NOT be reported.
    #[doc(hidden)]
    pub const fn becomes_const(&self) -> i64 {
        0
    }
}

// A new method that starts as const, so should NOT be reported (it wasn't in baseline at all).
pub struct NewConstMethod;

impl NewConstMethod {
    pub fn existing_method(&self) -> i64 {
        0
    }

    pub const fn brand_new_const_method(&self) -> i64 {
        99
    }
}