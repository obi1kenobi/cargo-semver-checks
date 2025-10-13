#![no_std]

// Public enum with #[derive(PartialOrd)] and struct variant with reordered fields - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub enum PublicEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Private enum with #[derive(PartialOrd)] and struct variant with reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public enum without #[derive(PartialOrd)] but with struct variant with reordered fields - should not trigger
pub enum RegularEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub enum DocHiddenEnum {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) variant - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenVariant {
    #[doc(hidden)]
    StructVariant { b: u16, c: u32, a: u8 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) fields - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenFields {
    StructVariant {
        a: u8,
        #[doc(hidden)]
        c: u32,
        #[doc(hidden)]
        b: u16,
    },
}

// Public enum with #[derive(PartialOrd)] and same field order - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum UnchangedEnum {
    StructVariant { x: u8, y: u16 },
}

// Test with manual PartialOrd implementation - should not trigger
#[derive(PartialEq)]
pub enum ManuallyImplemented {
    StructVariant { b: u16, a: u8 },
}

impl PartialOrd for ManuallyImplemented {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        todo!()
    }
}

// Test with multiple variants - should check each struct variant independently
#[derive(PartialOrd, PartialEq)]
pub enum MultipleVariants {
    First { b: u16, a: u8 },
    Second { y: u64, x: u32 },
    Third(u16, u8), // tuple variant reordered but should be ignored
}

// Test case: Used to derive PartialOrd but no longer does - should not trigger this lint
#[derive(PartialEq)]
pub enum NoLongerPartialOrd {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Test case: Didn't derive PartialOrd before but now does - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum NewlyPartialOrd {
    StructVariant { b: u16, c: u32, a: u8 },
}

// Test case: Switching from derived to hand-implemented PartialOrd - should not trigger this lint
#[derive(PartialEq)]
pub enum DerivedToHandImpl {
    StructVariant { b: u16, c: u32, a: u8 },
}

impl PartialOrd for DerivedToHandImpl {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        todo!()
    }
}
