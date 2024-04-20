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



// Test that the removal of an inherent method `next()` does not trigger `inherent_method_missing`
// if the equivalent method `Iterator::next()` is available and public API.
pub struct MyIter;

impl Iterator for MyIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// Test that the removal of an inherent method `next()` triggers `inherent_method_missing`
// if there is an equivalent method `Iterator::next()` but isn't public API due to `#[doc(hidden)]`.
pub struct SecretlyIter;

#[doc(hidden)]
impl Iterator for SecretlyIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
