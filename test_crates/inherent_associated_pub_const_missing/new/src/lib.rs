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
