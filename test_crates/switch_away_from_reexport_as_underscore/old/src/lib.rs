mod inner {
    pub trait Trait {}

    pub struct Struct {}

    pub enum Enum {
        First,
    }
}

pub use inner::Trait as _;
pub use inner::Struct as _;
pub use inner::Enum as _;
pub use inner::Enum::First as _;
