#![no_std]

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

// Public repr(C) struct with doc(hidden) fields swapped positions - should not trigger
#[repr(C)]
pub struct PublicStructWithDocHiddenFieldsSwapped {
    pub a: u8,
    #[doc(hidden)]
    pub b: u16,
    #[doc(hidden)]
    pub c: u32,
    pub d: u64,
}

// Public repr(C) struct with private fields swapped positions - should not trigger
#[repr(C)]
pub struct PublicStructWithPrivateFieldsSwapped {
    pub a: u8,
    b: u16,
    c: u32,
    pub d: u64,
}

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

// A case in which a new field is later added.
#[repr(C)]
pub struct StructWithAddition {
    pub a: u8,
    pub b: u16,
}

// A case in which a field is later removed.
#[repr(C)]
pub struct StructWithRemoval {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}
