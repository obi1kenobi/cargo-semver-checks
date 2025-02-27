// Public enum with #[derive(PartialOrd)] and struct variant with reordered fields - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub enum PublicEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Private enum with #[derive(PartialOrd)] and struct variant with reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public enum without #[derive(PartialOrd)] but with struct variant with reordered fields - should not trigger
pub enum RegularEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub enum DocHiddenEnum {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) variant - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenVariant {
    #[doc(hidden)]
    StructVariant { a: u8, b: u16, c: u32 },
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) fields - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenFields {
    StructVariant {
        a: u8,
        #[doc(hidden)]
        b: u16,
        #[doc(hidden)]
        c: u32,
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
    StructVariant { a: u8, b: u16 },
}

impl PartialOrd for ManuallyImplemented {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

// Test with multiple variants - should check each struct variant independently
#[derive(PartialOrd, PartialEq)]
pub enum MultipleVariants {
    First { a: u8, b: u16 },
    Second { x: u32, y: u64 },
    Third(u8, u16), // tuple variant should be ignored
    Fourth,         // unit variant should be ignored
}

// Test case: Used to derive PartialOrd but no longer does - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum NoLongerPartialOrd {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Test case: Didn't derive PartialOrd before but now does - should not trigger this lint
pub enum NewlyPartialOrd {
    StructVariant { a: u8, b: u16, c: u32 },
}

// Test case: Switching from derived to hand-implemented PartialOrd - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum DerivedToHandImpl {
    StructVariant { a: u8, b: u16, c: u32 },
}
