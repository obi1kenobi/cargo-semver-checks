#![no_std]

// Public repr(C) enum with struct variant having reordered fields - should trigger warning
#[repr(C)]
pub enum PublicEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Private repr(C) enum with struct variant having reordered fields - should not trigger
#[repr(C)]
enum PrivateEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public non-repr(C) enum with struct variant having reordered fields - should not trigger
pub enum RegularEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public repr(C) enum with doc(hidden) - should not trigger
#[doc(hidden)]
#[repr(C)]
pub enum DocHiddenEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public repr(C) enum with doc(hidden) fields - should not trigger
#[repr(C)]
pub enum PublicEnumWithDocHiddenFields {
    StructVariant {
        a: u8,
        #[doc(hidden)]
        b: u16,
        #[doc(hidden)]
        c: u32,
        d: u64,
    },
}

// Public repr(C) enum with doc(hidden) variant - should not trigger
#[repr(C)]
pub enum EnumWithDocHiddenVariant {
    #[doc(hidden)]
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public repr(C) enum with same order - should not trigger
#[repr(C)]
pub enum UnchangedEnum {
    StructVariant { x: u8, y: u16 },
}

// Test multiple repr attributes
#[repr(C, align(8))]
pub enum MultipleReprEnum {
    StructVariant { a: u8, b: u16 },
}

// A case in which a new field is later added
#[repr(C)]
pub enum EnumWithAddition {
    StructVariant { a: u8, b: u16 },
}

// A case in which a field is later removed
#[repr(C)]
pub enum EnumWithRemoval {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Test with tuple variant - should not trigger even if reordered
#[repr(C)]
pub enum EnumWithTupleVariant {
    TupleVariant(u8, u16, u32),
}

// Test with multiple variants - should check each struct variant independently
#[repr(C)]
pub enum MultiVariantEnum {
    First { a: u8, b: u16 },
    Second { x: u32, y: u64 },
    Third(u8, u16), // tuple variant should be ignored
    Fourth,         // unit variant should be ignored
}
