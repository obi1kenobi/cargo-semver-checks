/// positive test - a function changes export_name
#[export_name = "export_name_changed_old"]
pub fn export_name_changed() {}

/// positive test - a function changes no_mangle to a different export_name
#[no_mangle]
pub fn no_mangle_changed_to_other_export_name() {}

/// positive test - a function changes export_name to a different no_mangle
#[export_name = "export_name_changed_to_other_no_mangle_old"]
pub fn export_name_changed_to_other_no_mangle() {}

/// negative test - a function changes no_mangle to an equivalent export_name
#[no_mangle]
pub fn no_mangle_changed_to_same_export_name() {}

/// negative test - a function changes export_name to an equivalent no_mangle
#[export_name = "export_name_changed_to_same_no_mangle"]
pub fn export_name_changed_to_same_no_mangle() {}

/// negative test - a function's export name is removed
/// this is a breaking change,  but it's not this lint
#[no_mangle]
pub fn export_name_removed() {}

/// positive test - a non-public function changes export name
#[export_name = "private_export_name_changed_old"]
pub fn private_export_name_changed() {}

/// negative test - export name on one function gets moved to the other
/// this is not necessarily a breaking change, as long as the ABIs are the same
/// but that is a different lint
pub mod export_name_moved {
    #[export_name = "export_name_moved"]
    pub fn export_name_moved_1() {}
    
    pub fn export_name_moved_2() {}
}

