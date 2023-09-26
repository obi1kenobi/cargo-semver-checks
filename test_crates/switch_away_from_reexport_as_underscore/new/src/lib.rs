mod inner {
    pub trait Trait {}

    pub struct Struct {}

    pub enum Enum {
        First,
    }
}

// This is not a breaking change:
// all the items merely became nameable for the first time.
pub use inner::*;
pub use inner::Enum::*;
