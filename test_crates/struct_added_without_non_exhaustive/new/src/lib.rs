// Existing structs shouldn't trigger the lint
pub struct ExistingStruct {
    pub field: i32,
}

// Should NOT trigger the lint - private struct
struct PrivateStruct {
    pub field: i32,
}

// Should trigger the lint - new public struct with all pub fields
pub struct NewAllPubFields {
    pub field1: i32,
    pub field2: String,
}

// Should NOT trigger the lint - has private field
pub struct NewWithPrivateField {
    pub field1: i32,
    field2: String, // private field
}

// Should NOT trigger the lint - marked non_exhaustive
#[non_exhaustive]
pub struct NewWithNonExhaustive {
    pub field1: i32,
    pub field2: String,
}

// Should NOT trigger the lint - not public
struct NewPrivateStruct {
    pub field1: i32,
    pub field2: String,
}

// Should trigger the lint - unit struct
pub struct NewUnitStruct;

// Should NOT trigger the lint - private in public module
pub mod public_mod {
    struct PrivateInPublicMod {
        pub field: i32,
    }
}

// Should trigger the lint - tuple struct with all pub fields
pub struct NewTupleStruct(pub i32, pub String);

// Should NOT trigger the lint - tuple struct with private field
pub struct NewTupleWithPrivate(pub i32, String);
