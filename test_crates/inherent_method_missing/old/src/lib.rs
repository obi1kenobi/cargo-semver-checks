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
