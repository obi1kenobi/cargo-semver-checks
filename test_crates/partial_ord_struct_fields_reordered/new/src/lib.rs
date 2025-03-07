#![no_std]

// Public struct with #[derive(PartialOrd)] and reordered fields - should trigger warning
#[derive(PartialOrd, PartialEq)]
pub struct PublicStruct {
    pub b: u16, // moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Private struct with #[derive(PartialOrd)] and reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
struct PrivateStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public struct without #[derive(PartialOrd)] but with reordered fields - should not trigger
pub struct RegularStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public struct with #[derive(PartialOrd)] and doc(hidden) - should not trigger
#[doc(hidden)]
#[derive(PartialOrd, PartialEq)]
pub struct DocHiddenStruct {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Public struct with #[derive(PartialOrd)] and doc(hidden) fields swapped - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct PublicStructWithDocHiddenFieldsSwapped {
    pub a: u8,
    #[doc(hidden)]
    pub c: u32,
    #[doc(hidden)]
    pub b: u16,
    pub d: u64,
}

// Public struct with #[derive(PartialOrd)] and same order - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct UnchangedStruct {
    pub x: u8,  // same position
    pub y: u16, // same position
}

// Test with manual PartialOrd implementation - should not trigger
#[derive(PartialEq)]
pub struct ManuallyImplemented {
    pub b: u16, // reordered but has manual impl
    pub a: u8,
}

impl PartialOrd for ManuallyImplemented {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.a.cmp(&other.a))
    }
}

// Test with other derives - should not trigger
#[derive(Debug, Clone, PartialEq)]
pub struct OtherDerives {
    pub b: u16,
    pub a: u8,
}

// Test with multiple derives including PartialOrd - should trigger
#[derive(Debug, PartialOrd, Clone, PartialEq)]
pub struct MultipleDerivesWithPartialOrd {
    pub b: u16, // moved
    pub c: u32, // moved
    pub a: u8,  // moved
}

// Struct that no longer derives PartialOrd but has reordered fields - should not trigger
pub struct NoLongerPartialOrd {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Struct that newly derives PartialOrd and has reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct NewlyPartialOrd {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

// Struct that switches from derived to hand-implemented PartialOrd - should not trigger
pub struct SwitchToHandImpl {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}

impl PartialEq for SwitchToHandImpl {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl PartialOrd for SwitchToHandImpl {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        // Custom implementation that maintains original ordering logic
        match self.a.partial_cmp(&other.a) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.b.partial_cmp(&other.b) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.c.partial_cmp(&other.c)
    }
}

// Struct that switches from hand-implemented to derived PartialOrd - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct SwitchToDerived {
    pub b: u16,
    pub c: u32,
    pub a: u8,
}
