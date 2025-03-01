// Public enum with #[derive(PartialOrd)] and reordered variants - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub enum PublicEnum {
    A,
    B,
    C,
}

// Private enum with #[derive(PartialOrd)] and reordered variants - should not trigger
#[derive(PartialOrd, PartialEq)]
enum PrivateEnum {
    A,
    B,
    C,
}

// Public enum without #[derive(PartialOrd)] but with reordered variants - should not trigger
pub enum RegularEnum {
    A,
    B,
    C,
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub enum DocHiddenEnum {
    A,
    B,
    C,
}

// Public enum with #[derive(PartialOrd)] and doc(hidden) variants - should not trigger
#[derive(PartialOrd, PartialEq)]
pub enum EnumWithDocHiddenVariants {
    A,
    #[doc(hidden)]
    B,
    #[doc(hidden)]
    C,
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
    A,
    B,
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
    A,
    B,
    C,
}

// Test with mixed variant types - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum MixedVariantTypes {
    Unit,
    Tuple(i32, String),
    Struct { x: i32, y: String },
}

// Test with struct variants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum StructVariants {
    A { x: i32 },
    B { y: String },
    C { z: bool },
}

// Test with tuple variants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum TupleVariants {
    A(i32),
    B(String),
    C(bool),
}

// Test with discriminants - should trigger
#[derive(PartialOrd, PartialEq)]
pub enum WithDiscriminants {
    A = 10,
    B = 20,
    C = 30,
}

// Test case: Used to derive PartialOrd but no longer does - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum NoLongerPartialOrd {
    A,
    B,
    C,
}

// Test case: Didn't derive PartialOrd before but now does - should not trigger this lint
pub enum NewlyPartialOrd {
    A,
    B,
    C,
}

// Test case: Switching from derived to hand-implemented PartialOrd - should not trigger this lint
#[derive(PartialOrd, PartialEq)]
pub enum DerivedToHandImpl {
    A,
    B,
    C,
}

// Test case: Switching from hand-implemented to derived PartialOrd - should not trigger this lint
#[derive(PartialEq)]
pub enum HandImplToDerived {
    A,
    B,
    C,
}

impl PartialOrd for HandImplToDerived {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::A, Self::A) => Some(std::cmp::Ordering::Equal),
            (Self::A, Self::B) | (Self::A, Self::C) => Some(std::cmp::Ordering::Less),
            (Self::B, Self::A) => Some(std::cmp::Ordering::Greater),
            (Self::B, Self::B) => Some(std::cmp::Ordering::Equal),
            (Self::B, Self::C) => Some(std::cmp::Ordering::Less),
            (Self::C, Self::A) | (Self::C, Self::B) => Some(std::cmp::Ordering::Greater),
            (Self::C, Self::C) => Some(std::cmp::Ordering::Equal),
        }
    }
}
