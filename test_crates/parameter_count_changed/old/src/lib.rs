#![no_std]

pub fn function_with_a_parameter_added(a: ()) {}

pub fn function_with_parameters_removed(a: (), b: ()) {}

fn private_function_with_a_parameter_added(_: ()) {}

pub struct StructWithMethods {}

impl StructWithMethods {
    pub fn associated_function_with_a_parameter_added() {}

    pub fn method_with_a_parameter_added(&self) {}

    pub fn method_with_a_parameter_removed(&self, _: (), _: (), _: ()) {}

    pub fn moved_trait_provided_method(&self, _: ()) {}

    pub fn moved_trait_provided_method_with_unchanged_signature(&self, _: ()) {}

    pub fn moved_method(&self, _: ()) {}

    pub fn moved_method_with_unchanged_signature(&self, _: ()) {}
}

struct PrivateStruct {}

impl PrivateStruct {
    pub fn method_with_a_parameter_added() {}
}

// Ensure there are no false-positives in cases with
// multiple methods with the same name but different signatures.
// Here, the struct defines an inherent method `duplicate()`,
// and also has `<Self as DuplicateMethodTrait>::duplicate()` available with a different signature.
//
// In https://github.com/obi1kenobi/cargo-semver-checks/issues/193 this sort of setup
// matched the `method_parameter_count_changed` lint and was a false-positive.
// It should obviously not match since no change happened.
pub struct DuplicateMethodNames {}

impl DuplicateMethodNames {
    pub fn duplicate(&self) {}
}

trait DuplicateMethodTrait {
    fn duplicate(&self, _: ()) {}
}

impl DuplicateMethodTrait for DuplicateMethodNames {}
