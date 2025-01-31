// Existing structs shouldn't trigger the lint
pub struct ExistingStruct {
    pub field: i32,
}

struct PrivateStruct {
    pub field: i32,
}

// This module helps test that private paths don't trigger the lint
mod private {
    pub struct InPrivateModule {
        pub field: i32,
    }
}
