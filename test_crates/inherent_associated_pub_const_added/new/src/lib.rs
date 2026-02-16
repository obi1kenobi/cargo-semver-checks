#![no_std]

pub struct MyStruct;

impl MyStruct {
    // Existing public const (should not trigger)
    pub const EXISTING_CONST: i32 = 1;
    // New public const (should trigger)
    pub const MY_CONST: i32 = 42;
    // Private const (should not trigger)
    const PRIVATE_CONST: i32 = 99;
}

// Non-public struct with new const (should not trigger)
struct PrivateStruct;

impl PrivateStruct {
    pub const NEW_CONST: i32 = 100;
}
