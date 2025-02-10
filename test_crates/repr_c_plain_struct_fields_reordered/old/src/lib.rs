// Public repr(C) struct with reordered fields - should trigger warning
#[repr(C)]
pub struct PublicStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Private repr(C) struct with reordered fields - should not trigger
#[repr(C)]
struct PrivateStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public non-repr(C) struct with reordered fields - should not trigger
pub struct RegularStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public repr(C) struct with doc(hidden) - should not trigger
#[doc(hidden)]
#[repr(C)]
pub struct DocHiddenStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public repr(C) struct with mixed visibility fields
#[repr(C)]
pub struct MixedVisibilityStruct {
    pub a: u8,
    b: u16, // private field
    pub c: u32,
}

// Public repr(C) struct with same order - should not trigger
#[repr(C)]
pub struct UnchangedStruct {
    pub x: u8,
    pub y: u16,
}

// Test multiple repr attributes
#[repr(C, align(8))]
pub struct MultipleReprStruct {
    pub a: u8,
    pub b: u16,
}
