// Public struct with #[derive(PartialOrd)] and reordered fields - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub struct PublicStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Private struct with #[derive(PartialOrd)] and reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
struct PrivateStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public struct without #[derive(PartialOrd)] but with reordered fields - should not trigger
pub struct RegularStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public struct with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub struct DocHiddenStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Public struct with #[derive(PartialOrd)] and doc(hidden) fields swapped - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct PublicStructWithDocHiddenFieldsSwapped {
    pub a: u8,
    #[doc(hidden)]
    pub b: u16,
    #[doc(hidden)]
    pub c: u32,
    pub d: u64,
}

// Public struct with #[derive(PartialOrd)] and same order - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct UnchangedStruct {
    pub x: u8,
    pub y: u16,
}

// Test with manual PartialOrd implementation - should not trigger
#[derive(PartialEq)]
pub struct ManuallyImplemented {
    pub a: u8,
    pub b: u16,
}

impl PartialOrd for ManuallyImplemented {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.a.cmp(&other.a))
    }
}

// Test with other derives - should not trigger
#[derive(Debug, Clone, PartialEq)]
pub struct OtherDerives {
    pub a: u8,
    pub b: u16,
}

// Test with multiple derives including PartialOrd - should trigger
#[derive(Debug, PartialOrd, Clone, PartialEq)]
pub struct MultipleDerivesWithPartialOrd {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}
