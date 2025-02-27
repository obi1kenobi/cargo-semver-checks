// Public enum with #[derive(PartialOrd)] and reordered variants - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub enum PublicEnum {
    B,
    C,
    A,
}

// Private enum with #[derive(PartialOrd)] and reordered variants - should not trigger
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    B,
    C,
    A,
}

// Public enum without #[derive(PartialOrd)] but with reordered variants - should not trigger
pub enum RegularEnum {
    B,
    C,
    A,
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub enum DocHiddenEnum {
    B,
    C,
    A,
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) variants - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenVariants {
    A,
    #[doc(hidden)]
    C,
    #[doc(hidden)]
    B,
    D,
}

// Public enum with #[derive(PartialOrd)] and same order - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum UnchangedEnum {
    X,
    Y,
}

// Test with manual PartialOrd implementation - should not trigger
#[derive(PartialEq)]
pub enum ManuallyImplemented {
    B,
    A,
}

impl PartialOrd for ManuallyImplemented {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::A, Self::A) => Some(std::cmp::Ordering::Equal),
            (Self::A, Self::B) => Some(std::cmp::Ordering::Less),
            (Self::B, Self::A) => Some(std::cmp::Ordering::Greater),
            (Self::B, Self::B) => Some(std::cmp::Ordering::Equal),
        }
    }
}

// Test with multiple derives including PartialOrd - should trigger
#[derive(Debug, PartialOrd, Clone, PartialEq)]
pub enum MultipleDerivesWithPartialOrd {
    B,
    C,
    A,
}

// Test with mixed variant types - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum MixedVariantTypes {
    Tuple(i32, String),
    Struct { x: i32, y: String },
    Unit,
}

// Test with struct variants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum StructVariants {
    B { y: String },
    C { z: bool },
    A { x: i32 },
}

// Test with tuple variants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum TupleVariants {
    B(String),
    C(bool),
    A(i32),
}

// Test with discriminants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum WithDiscriminants {
    B = 20,
    C = 30,
    A = 10,
}

// Test case: Used to derive PartialOrd but no longer does - should not trigger this lint
#[derive(PartialEq)]
pub enum NoLongerPartialOrd {
    B,
    C,
    A,
}

// Test case: Didn't derive PartialOrd before but now does - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum NewlyPartialOrd {
    B,
    C,
    A,
}

// Test case: Switching from derived to hand-implemented PartialOrd - should not trigger this lint
#[derive(PartialEq)]
pub enum DerivedToHandImpl {
    B,
    C,
    A,
}

impl PartialOrd for DerivedToHandImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

// Test case: Switching from hand-implemented to derived PartialOrd - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum HandImplToDerived {
    B,
    C,
    A,
}
