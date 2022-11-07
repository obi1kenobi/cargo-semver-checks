#[cfg(not(feature = "function_parameter_count_changed"))]
pub fn function_with_a_parameter_added(_: ()) {}

#[cfg(feature = "function_parameter_count_changed")]
pub fn function_with_a_parameter_added(_: (), _: ()) {}

#[cfg(not(feature = "function_parameter_count_changed"))]
pub fn function_with_parameters_removed(_: (), _: ()) {}

#[cfg(feature = "function_parameter_count_changed")]
pub fn function_with_parameters_removed() {}

pub struct StructWithMethods {}

impl StructWithMethods {
    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn method_with_a_parameter_added() {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn method_with_a_parameter_added(_: ()) {}

    #[cfg(not(feature = "method_parameter_count_changed"))]
    pub fn method_with_a_parameter_removed(_: (), _: (), _: ()) {}

    #[cfg(feature = "method_parameter_count_changed")]
    pub fn method_with_a_parameter_removed(_: (), _: ()) {}
}