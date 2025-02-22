// Public repr(C) struct with reordered fields - should trigger warning
#[repr(C)]
pub struct PublicStruct {
    pub b: u16, // moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Private repr(C) struct with reordered fields - should not trigger
#[repr(C)]
struct PrivateStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public non-repr(C) struct with reordered fields - should not trigger
pub struct RegularStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public repr(C) struct with doc(hidden) - should not trigger
#[doc(hidden)]
#[repr(C)]
pub struct DocHiddenStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public repr(C) struct with doc(hidden) fields swapped positions - should not trigger
#[repr(C)]
pub struct PublicStructWithDocHiddenFieldsSwapped {
    pub a: u8,
    #[doc(hidden)]
    pub c: u32,
    #[doc(hidden)]
    pub b: u16,
    pub d: u64,
}

// Public repr(C) struct with private fields swapped positions - should not trigger
#[repr(C)]
pub struct PublicStructWithPrivateFieldsSwapped {
    pub a: u8,
    c: u32,
    b: u16,
    pub d: u64,
}

// Public repr(C) struct with mixed visibility fields
#[repr(C)]
pub struct MixedVisibilityStruct {
    b: u16,     // private field moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Public repr(C) struct with same order - should not trigger
#[repr(C)]
pub struct UnchangedStruct {
    pub x: u8,  // same position
    pub y: u16, // same position
}

// Test multiple repr attributes
#[repr(C, align(8))]
pub struct MultipleReprStruct {
    pub b: u16, // moved
    pub a: u8,  // moved
}

// A case in which a new field is later added.
#[repr(C)]
pub struct StructWithAddition {
    pub b: u16, // reordered...
    pub a: u8,
    pub c: u32, // extra field added
}

// A case in which a field is later removed.
#[repr(C)]
pub struct StructWithRemoval {
    pub b: u16, // fields are reordered and one field is removed
    pub c: u32,
}
