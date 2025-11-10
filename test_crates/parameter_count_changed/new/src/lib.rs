#![no_std]

pub fn function_with_a_parameter_added(a: (), b: ()) {}

pub fn function_with_parameters_removed() {}

fn private_function_with_a_parameter_added(_: (), _: ()) {}

pub struct StructWithMethods {}

impl StructWithMethods {
    pub fn associated_function_with_a_parameter_added(_: ()) {}

    pub fn method_with_a_parameter_added(&self, _: ()) {}

    pub fn method_with_a_parameter_removed(&self, _: (), _: ()) {}

    pub fn method_goes_from_taking_self_arg_to_explicit_arg(value: &Self) {}
}

pub trait Bar {
    fn moved_trait_provided_method(&self, _: (), _: ()) {}

    fn moved_trait_provided_method_with_unchanged_signature(&self, _: ()) {}

    fn moved_method(&self);

    fn moved_method_with_unchanged_signature(&self, _: ());
}

impl Bar for StructWithMethods {
    fn moved_method(&self) {}

    fn moved_method_with_unchanged_signature(&self, _: ()) {}
}

struct PrivateStruct {}

impl PrivateStruct {
    pub fn method_with_a_parameter_added(_: ()) {}
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

pub struct ReceiverChanges;

impl ReceiverChanges {
    // This isn't breaking, because callers had to use fully-qualified notation before,
    // and now they can merely *also* use the method-style syntax sugar.
    pub fn parameter_becomes_receiver(&self) {}

    // This is breaking: calling via the receiver doesn't work anymore.
    pub fn receiver_becomes_parameter(_x: &Self) {}
}

pub trait ProvideMethod {
    fn provided_method(&self) {}
}

pub struct MethodGoesToTrait;

impl ProvideMethod for MethodGoesToTrait {}

impl MethodGoesToTrait {
    /// Turning this method into an associated function is not a major breaking change by itself,
    /// because `pub trait ProvideMethod` has a matching method (not associated fn) that
    /// method-like calls will dispatch to.
    ///
    /// Proof:
    /// ```rust
    /// fn proof(x: MethodGoesToTrait) {
    ///     x.provided_method();
    /// }
    /// ```
    pub fn provided_method(_value: &Self) {}
}
