#![no_std]

pub struct PublicStruct;

impl PublicStruct {
    #[unsafe(export_name = "method_export_name_changed_old")]
    pub fn export_name_changed(&self) {}

    #[unsafe(no_mangle)]
    pub fn no_mangle_changed_to_other_export_name(&self) {}

    #[unsafe(export_name = "export_name_changed_to_other_no_mangle_old")]
    pub fn export_name_changed_to_other_no_mangle(&self) {}

    #[unsafe(no_mangle)]
    pub fn no_mangle_removed(&self) {}

    #[unsafe(export_name = "export_name_removed")]
    pub fn export_name_removed(&self) {}

    #[unsafe(export_name = "private_export_name_changed_old")]
    fn private_export_name_changed(&self) {}

    #[unsafe(export_name = "export_name_moved")]
    pub fn export_name_moved_1(&self) {}

    pub fn export_name_moved_2(&self) {}

    pub fn export_name_added(&self) {}
}

#[doc(hidden)]
pub struct HiddenType;

impl HiddenType {
    #[unsafe(export_name = "hidden_type_export_name_old")]
    pub fn hidden_type_export_name_changed(&self) {}
}

pub struct PublicStructWithHiddenMethod;

impl PublicStructWithHiddenMethod {
    #[doc(hidden)]
    #[unsafe(export_name = "hidden_method_export_name_old")]
    pub fn hidden_method_export_name_changed(&self) {}
}
