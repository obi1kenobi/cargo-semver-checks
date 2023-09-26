mod inner {
    pub trait Trait {}

    pub struct Struct {}

    pub enum Enum {
        First,
    }
}

// Each of these items is a breaking change:
// none of their names are usable now, even though the items are public.
pub use inner::Trait as _;
pub use inner::Struct as _;
pub use inner::Enum as _;
pub use inner::Enum::First as _;
