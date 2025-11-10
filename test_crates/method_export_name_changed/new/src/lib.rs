#![no_std]

pub struct PublicStruct;

impl PublicStruct {
    #[unsafe(export_name = "method_export_name_changed_new")]
    pub fn export_name_changed(&self) {}

    #[unsafe(export_name = "no_mangle_changed_to_other_export_name_new")]
    pub fn no_mangle_changed_to_other_export_name(&self) {}

    #[unsafe(no_mangle)]
    pub fn export_name_changed_to_other_no_mangle(&self) {}

    pub fn no_mangle_removed(&self) {}

    pub fn export_name_removed(&self) {}

    #[unsafe(export_name = "private_export_name_changed_new")]
    fn private_export_name_changed(&self) {}

    pub fn export_name_moved_1(&self) {}

    #[unsafe(export_name = "export_name_moved")]
    pub fn export_name_moved_2(&self) {}

    #[unsafe(export_name = "export_name_added")]
    pub fn export_name_added(&self) {}
}

#[doc(hidden)]
pub struct HiddenType;

impl HiddenType {
    #[unsafe(export_name = "hidden_type_export_name_new")]
    pub fn hidden_type_export_name_changed(&self) {}
}

pub struct PublicStructWithHiddenMethod;

impl PublicStructWithHiddenMethod {
    #[doc(hidden)]
    #[unsafe(export_name = "hidden_method_export_name_new")]
    pub fn hidden_method_export_name_changed(&self) {}
}
