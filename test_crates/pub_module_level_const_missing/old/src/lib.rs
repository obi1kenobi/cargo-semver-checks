
pub const pub_const_in_global: i32 = 0;
pub const pub_const_in_global_will_remove: i32 = 0;
pub const pub_const_in_global_will_rename: i32 = 0;
pub const pub_const_in_global_will_be_static: i32 = 0;
pub const pub_const_in_global_will_be_private_const: i32 = 0;
pub const pub_const_in_global_will_be_private_static: i32 = 0;

pub mod my_module {
    pub const pub_const_in_module: i32 = 0;
    pub const pub_const_in_module_will_remove: i32 = 0;
    pub const pub_const_in_module_will_rename: i32 = 0;
    pub const pub_const_in_module_will_be_static: i32 = 0;
    pub const pub_const_in_module_will_be_private_const: i32 = 0;
    pub const pub_const_in_module_will_be_private_static: i32 = 0;
    pub const pub_const_in_module_will_re_export: i32 = 0;
    pub const pub_const_in_module_will_re_export_static: i32 = 0;

    pub mod my_module_nested {
        pub const pub_const_in_nested_module: i32 = 0;
        pub const pub_const_in_nested_module_will_remove: i32 = 0;
        pub const pub_const_in_nested_module_will_rename: i32 = 0;
        pub const pub_const_in_nested_module_will_be_static: i32 = 0;
        pub const pub_const_in_nested_module_will_be_private_const: i32 = 0;
        pub const pub_const_in_nested_module_will_be_private_static: i32 = 0;
        pub const pub_const_in_nested_module_will_re_export: i32 = 0;
        pub const pub_const_in_nested_module_will_re_export_static: i32 = 0;
    }
}