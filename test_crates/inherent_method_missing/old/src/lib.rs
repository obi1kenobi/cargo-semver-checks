// Test that associated function removal:
// - is caught and reported by `inherent_method_missing`
// - is not caught by `function_missing`.
pub struct Foo {}

impl Foo {
    pub fn will_be_removed_associated_fn() {}

    pub fn will_be_removed_method(&self) {}

    pub fn moved_trait_provided_method(&self) {}

    pub fn moved_method(&self) {}
}

// Test that the removal of an inherent method `next()` does not trigger `inherent_method_missing`
// if the equivalent method `Iterator::next()` is available and public API.
pub struct MyIter;

impl MyIter {
    // This method is getting removed.
    pub fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        <Self as Iterator>::next()
    }
}

impl Iterator for MyIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// Test that the removal of an inherent method `next()` triggers `inherent_method_missing`
// if there is an equivalent method `Iterator::next()` that isn't public API via `#[doc(hidden)]`.
pub struct SecretlyIter;

impl SecretlyIter {
    // This method is getting removed.
    pub fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        <Self as Iterator>::next()
    }
}

#[doc(hidden)]
impl Iterator for SecretlyIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
