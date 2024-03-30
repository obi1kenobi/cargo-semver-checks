// Test Cases where #[doc(hidden)] is neither added or removed
pub struct StructA {}

impl StructA {
    // Basic Test case should be caught
    pub const PublicConstantA: i32 = 0;
    pub const PublicConstantB: i32 = 0;
    // Const that was #[doc(hidden)] will be removed
    #[doc(hidden)]
    pub const DocHiddenConstantA: i32 = 0;
    // Should Be caught on renaming
    pub const PublicConstantRenameA: i32 = 0;
    // Should not be caught on removing since its not pub
    const ConstantA: i32 = 0;
}

struct StructB {}

impl StructB {
    // Should not be caught since StructB is not pub
    pub const PublicConstantC: i32 = 0;
    pub const PublicConstantRenameB: i32 = 0;
    const ConstantB: i32 = 0;
}

#[doc(hidden)]
pub struct StructC {}

impl StructC {
    // Should not be caught on removing or renaming since the struct #[doc(hidden)]
    pub const PublicConstantD: i32 = 0;
    pub const PublicConstantRenameC: i32 = 0;
}

pub struct StructD {}

#[doc(hidden)]
impl StructD {
    pub const PublicConstantE: i32 = 0;
    pub const PublicConstantF: i32 = 0;
}

// Test Cases where #[doc(hidden)] is added
pub struct StructE {}

// This impl block will be #[doc(hidden)]
impl StructE {
    pub const PublicConstantH: i32 = 0;
    // This will be removed
    pub const PublicConstantI: i32 = 0;
}

// This struct will be #[doc(hidden)]
pub struct StructF {}

impl StructF {
    pub const PublicConstantJ: i32 = 0;
    // This const will be removed
    pub const PublicConstantK: i32 = 0;
}

// Test cases where #[doc(hidden)] is removed
#[doc(hidden)]
pub struct DocHiddenStruct {}

impl DocHiddenStruct {
    // This const will be removed
    #[doc(hidden)]
    pub const DocHiddenConstantB: i32 = 0;
    pub const PublicConstantL: i32 = 0;
    // This const will be removed
    pub const PublicConstantM: i32 = 0;
}

#[doc(hidden)]
impl DocHiddenStruct {
    // This const will be removed
    pub const PublicConstantN: i32 = 0;
    pub const PublicConstantO: i32 = 0;
}

// Test for when an inherent const is implemented as trait's const
pub trait TraitA {}

pub struct ExampleA;

impl ExampleA {
    pub const N_A: i64 = 0;
}

impl TraitA for ExampleA {}

// Test for when an inherent const is implemented as trait's const but given a value in impl
pub trait TraitB {}

pub struct ExampleB;

impl ExampleB {
    pub const N_B: i64 = 0;
}

impl TraitB for ExampleB {}
