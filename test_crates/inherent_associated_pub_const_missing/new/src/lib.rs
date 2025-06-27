#![no_std]

// Test Cases where #[doc(hidden)] is neither added or removed
pub struct StructA {}

impl StructA {
    pub const PublicConstantB: i32 = 0;
    // Should Be caught on renaming
    pub const PublicConstantRenamedA: i32 = 0;
}

struct StructB {}

impl StructB {
    // Should not be caught since StructB is not pub
    pub const PublicConstantRenamedB: i32 = 0;
}

#[doc(hidden)]
pub struct StructC {}

impl StructC {
    // Should not be caught on removing or renaming since the struct is #[doc(hidden)]
    pub const PublicConstantRenamedC: i32 = 0;
}

pub struct StructD {}

#[doc(hidden)]
impl StructD {
    pub const PublicConstantE: i32 = 0;
}

// Test Cases where #[doc(hidden)] is added
pub struct StructE {}

#[doc(hidden)]
impl StructE {
    pub const PublicConstantH: i32 = 0;
}

#[doc(hidden)]
pub struct StructF {}

impl StructF {
    pub const PublicConstantJ: i32 = 0;
}

// Test cases where #[doc(hidden)] is removed
pub struct DocHiddenStruct {}

impl DocHiddenStruct {
    pub const PublicConstantL: i32 = 0;
}

impl DocHiddenStruct {
    pub const PublicConstantO: i32 = 0;
}

// Test for when an inherent const is implemented as trait's const
pub trait TraitA {
    const N_A: i64 = 0;
}

pub struct ExampleA;

impl TraitA for ExampleA {}

// Test for when an inherent const is implemented as trait's const but given a value in impl block
pub trait TraitB {
    const N_B: i64;
}

pub struct ExampleB;

impl TraitB for ExampleB {
    const N_B: i64 = 0;
}

// Test for when an inherent const is implemented as trait's const,
// but the `impl Trait` block is not public API. In this case, we report the lint
// since falling back to the trait implementation would require using non-public API.
pub trait TraitC {
    const N_C: i64;
}

pub struct ExampleC;

#[doc(hidden)]
impl TraitC for ExampleC {
    const N_C: i64 = 0;
}
