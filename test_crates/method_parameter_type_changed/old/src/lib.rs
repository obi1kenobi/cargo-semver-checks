#![no_std]

pub struct StructWithMethods;

impl StructWithMethods {
    // These should all trigger the lint using witnesses
    pub fn parameter_changed_value(self, _: i32, _: i32) {}
    pub fn parameter_changed_ref(&self, _: i32, _: i32) {}
    pub fn parameter_changed_mut_ref(&mut self, _: i32, _: i32) {}

    // These should all trigger the lint using witnesses, as they will still change parameters.
    // Moving to a trait is not inherently a breaking change, so this still is entirely a breaking
    // changed caused by the change to the parameters.
    pub fn moved_to_trait_and_changed_value(self, _: i32) {}
    pub fn moved_to_trait_and_changed_ref(&self, _: i32) {}
    pub fn moved_to_trait_and_changed_mut_ref(&mut self, _: i32) {}

    // This should be detected by the lint, but should be ignored since i64 is compatible with impl Into<i64>
    pub fn explicit_arg_to_impl_trait(self, _: i64) {}
    pub fn moved_to_trait_explicit_arg_to_impl_trait(self, _: i64) {}

    // This should be detected by the lint, but should be ignored since Into<i64> is being extracted to a generic
    pub fn impl_trait_to_generic(self, _: impl Into<i64>) {}
    pub fn moved_to_trait_impl_trait_to_generic(self, _: impl Into<i64>) {}

    // This should be detected by the lint, but should be ignored since it is a symmetric non-breaking change
    pub fn impl_trait_to_generic_where_bound(self, _: impl Into<i64>) {}
    pub fn moved_to_trait_impl_trait_to_generic_where_bound(self, _: impl Into<i64>) {}

    // This should be detected by the lint, but should be ignored since it won't trigger the witness
    pub fn signature_changed(self, _param: ()) {}

    // This should be ignored entirely, since it has no receiver. This is an associated function, not a method.
    pub fn associated_function_parameter_changed(_: i32) {}
}
