#![no_std]

mod abi_only {
    /// positive test - exported function now returns a value
    #[unsafe(no_mangle)]
    pub extern "C" fn return_value_added() {}

    /// positive test - exported function with explicit export_name now returns a value
    #[unsafe(export_name = "export_name_return_value_added")]
    pub extern "C" fn export_name_return_value_added() {}
}

pub struct ExportedType;

impl ExportedType {
    /// positive test - exported method now returns a value
    #[unsafe(export_name = "method_return_value_added")]
    pub fn method_return_value_added(&self) {}
}
