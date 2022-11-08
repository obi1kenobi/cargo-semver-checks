#[cfg(not(feature = "function_parameter_count_changed"))]
pub fn function_with_a_parameter_added(_: ()) {}

#[cfg(feature = "function_parameter_count_changed")]
pub fn function_with_a_parameter_added(_: (), _: ()) {}

#[cfg(not(feature = "function_parameter_count_changed"))]
pub fn function_with_parameters_removed(_: (), _: ()) {}

#[cfg(feature = "function_parameter_count_changed")]
pub fn function_with_parameters_removed() {}

#[cfg(not(feature = "function_parameter_count_changed"))]
fn private_function_with_a_parameter_added(_: ()) {}

#[cfg(feature = "function_parameter_count_changed")]
fn private_function_with_a_parameter_added(_: (), _: ()) {}

pub struct StructWithMethods {}

impl StructWithMethods {
    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn associated_function_with_a_parameter_added() {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn associated_function_with_a_parameter_added(_: ()) {}

    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn method_with_a_parameter_added(&self) {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn method_with_a_parameter_added(&self, _: ()) {}

    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn method_with_a_parameter_removed(&self, _: (), _: (), _: ()) {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn method_with_a_parameter_removed(&self, _: (), _: ()) {}

    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn moved_trait_provided_method(&self, _: ()) {}

    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn moved_method(&self, _: ()) {}
}

#[cfg(feature = "method_parameter_count_changed")]
pub trait Bar {
    fn moved_trait_provided_method(&self, _: (), _: ()) {}

    fn moved_method(&self);
}

#[cfg(feature = "method_parameter_count_changed")]
impl Bar for StructWithMethods {
    fn moved_method(&self) {}
}

struct PrivateStruct {}

impl PrivateStruct {
    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn method_with_a_parameter_added() {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn method_with_a_parameter_added(_: ()) {}
}
