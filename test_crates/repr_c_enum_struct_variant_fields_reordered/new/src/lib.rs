// Public repr(C) enum with struct variant having reordered fields - should trigger warning
#[repr(C)]
pub enum PublicEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Private repr(C) enum with struct variant having reordered fields - should not trigger
#[repr(C)]
enum PrivateEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public non-repr(C) enum with struct variant having reordered fields - should not trigger
pub enum RegularEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public repr(C) enum with doc(hidden) - should not trigger
#[doc(hidden)]
#[repr(C)]
pub enum DocHiddenEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public repr(C) enum with doc(hidden) fields - should not trigger
#[repr(C)]
pub enum PublicEnumWithDocHiddenFields {
    StructVariant {
        a: u8,
        #[doc(hidden)]
        c: u32,
        #[doc(hidden)]
        b: u16,
        d: u64,
    },
}

// Public repr(C) enum with doc(hidden) variant - should not trigger
#[repr(C)]
pub enum EnumWithDocHiddenVariant {
    #[doc(hidden)]
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public repr(C) enum with same order - should not trigger
#[repr(C)]
pub enum UnchangedEnum {
    StructVariant { x: u8, y: u16 },
}

// Test multiple repr attributes
#[repr(C, align(8))]
pub enum MultipleReprEnum {
    StructVariant { b: u16, a: u8 },
}

// A case in which a new field is later added
#[repr(C)]
pub enum EnumWithAddition {
    StructVariant {
        b: u16,
        a: u8,
        c: u32, // new field
    },
}

// A case in which a field is later removed
#[repr(C)]
pub enum EnumWithRemoval {
    StructVariant { b: u16, c: u32 },
}

// Test with tuple variant - should not trigger even if reordered
#[repr(C)]
pub enum EnumWithTupleVariant {
    TupleVariant(u16, u32, u8), // reordered but should not trigger
}

// Test with multiple variants - should check each struct variant independently
#[repr(C)]
pub enum MultiVariantEnum {
    First { b: u16, a: u8 },
    Second { y: u64, x: u32 },
    Third(u16, u8), // tuple variant reordered but should be ignored
    Fourth,         // unit variant unchanged
}
