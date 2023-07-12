
pub const pub_const_in_global: i32 = 0;
pub const pub_const_in_global_is_renamed: i32 = 0;
pub static pub_const_in_global_will_be_static: i32 = 0;
const pub_const_in_global_will_be_private_const: i32 = 0;
static pub_const_in_global_will_be_private_static: i32 = 0;

mod re_exporter {
    pub const special_const: i32 = 0;
    pub static special_static: i32 = 0;
}

pub mod my_module {
    pub const pub_const_in_module: i32 = 0;
    pub const pub_const_in_module_is_renamed: i32 = 0;
    pub static pub_const_in_module_will_be_static: i32 = 0;
    const pub_const_in_module_will_be_private_const: i32 = 0;
    static pub_const_in_module_will_be_private_static: i32 = 0;

    pub use crate::re_exporter::special_const as 
        pub_const_in_module_will_re_export;
    pub use crate::re_exporter::special_static as 
        pub_const_in_module_will_re_export_static;

    pub mod my_module_nested {
        pub const pub_const_in_nested_module: i32 = 0;
        static pub_const_in_nested_module_will_remove_const: i32 = 0;
        pub const pub_const_in_nested_module_is_renamed: i32 = 0;
        pub static pub_const_in_nested_module_will_be_static: i32 = 0;
        const pub_const_in_nested_module_will_be_private: i32 = 0;

        pub use crate::re_exporter::special_const as 
            pub_const_in_nested_module_will_re_export;
        pub use crate::re_exporter::special_static as 
            pub_const_in_nested_module_will_re_export_static;
    }
}