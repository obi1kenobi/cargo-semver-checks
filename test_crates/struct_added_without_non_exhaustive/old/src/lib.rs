// Existing structs shouldn't trigger the lint
pub struct ExistingStruct {
    pub field: i32,
}

struct PrivateStruct {
    pub field: i32,
}
