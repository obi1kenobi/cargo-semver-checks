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

// Struct that no longer derives PartialOrd but has reordered fields - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct NoLongerPartialOrd {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Struct that newly derives PartialOrd and has reordered fields - should not trigger
pub struct NewlyPartialOrd {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Struct that switches from derived to hand-implemented PartialOrd - should not trigger
#[derive(PartialOrd, PartialEq)]
pub struct SwitchToHandImpl {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

// Struct that switches from hand-implemented to derived PartialOrd - should not trigger
pub struct SwitchToDerived {
    pub a: u8,
    pub b: u16,
    pub c: u32,
}

impl PartialEq for SwitchToDerived {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl PartialOrd for SwitchToDerived {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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
