#![no_std]

pub struct MyStruct;

impl MyStruct {
    // Existing public const (should not trigger)
    pub const EXISTING_CONST: i32 = 1;
}
