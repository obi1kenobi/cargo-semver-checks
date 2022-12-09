pub struct Foo {}

impl Foo {}

// Moving an inherent method to an implemented trait should not be a breaking change,
// both when the method is defined inside the trait and when it's implemented externally.
pub trait Bar {
    fn moved_trait_provided_method(&self) {}

    fn moved_method(&self);
}

impl Bar for Foo {
    fn moved_method(&self) {}
}
