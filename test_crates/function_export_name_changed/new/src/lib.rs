/// positive test - a function changes export_name
#[export_name = "export_name_changed_new"]
pub fn export_name_changed() {}

/// positive test - a function changes no_mangle to a different export_name
#[export_name = "no_mangle_changed_to_other_export_name_new"]
pub fn no_mangle_changed_to_other_export_name() {}

/// positive test - a function changes export_name to a different no_mangle
#[no_mangle]
pub fn export_name_changed_to_other_no_mangle() {}

/// negative test - a function changes no_mangle to an equivalent export_name
#[export_name = "no_mangle_changed_to_same_export_name"]
pub fn no_mangle_changed_to_same_export_name() {}

/// negative test - a function changes export_name to an equivalent no_mangle
#[no_mangle]
pub fn export_name_changed_to_same_no_mangle() {}

/// positive test - a function's no_mangle is removed
pub fn no_mangle_removed() {}

/// positive test - a function's export_name is removed
pub fn export_name_removed() {}

/// positive test - a non-public function changes export name
#[export_name = "private_export_name_changed_new"]
pub fn private_export_name_changed() {}

/// positive test - export name on one function gets moved to the other
/// this is still a breaking change because a call to `export_name_moved_1` in the API
/// is no longer equivalent to a call to `export_name_moved` in the ABI
pub mod export_name_moved {
    pub fn export_name_moved_1() {}
    
    #[export_name = "export_name_moved"]
    pub fn export_name_moved_2() {}
}

/// negative test - a function's export name is added
#[export_name = "export_name_added"]
pub fn export_name_added() {}