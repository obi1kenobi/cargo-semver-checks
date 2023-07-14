pub const PUB_CONST_IN_GLOBAL: i32 = 0;
pub const PUB_CONST_IN_GLOBAL_IS_RENAMED: i32 = 0;
pub static PUB_CONST_IN_GLOBAL_WILL_BE_STATIC: i32 = 0;
const PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_CONST: i32 = 0;
static PUB_CONST_IN_GLOBAL_WILL_BE_PRIVATE_STATIC: i32 = 0;

mod re_exporter {
    pub const SPECIAL_CONST: i32 = 0;
    pub static SPECIAL_STATIC: i32 = 0;
}

pub mod my_module {
    pub const PUB_CONST_IN_MODULE: i32 = 0;
    pub const PUB_CONST_IN_MODULE_IS_RENAMED: i32 = 0;
    pub static PUB_CONST_IN_MODULE_WILL_BE_STATIC: i32 = 0;
    const PUB_CONST_IN_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
    static PUB_CONST_IN_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;

    pub use crate::re_exporter::SPECIAL_CONST as PUB_CONST_IN_MODULE_WILL_RE_EXPORT;
    pub use crate::re_exporter::SPECIAL_STATIC as PUB_CONST_IN_MODULE_WILL_RE_EXPORT_STATIC;

    pub mod my_module_nested {
        pub const PUB_CONST_IN_NESTED_MODULE: i32 = 0;
        static PUB_CONST_IN_NESTED_MODULE_WILL_REMOVE_CONST: i32 = 0;
        pub const PUB_CONST_IN_NESTED_MODULE_IS_RENAMED: i32 = 0;
        pub static PUB_CONST_IN_NESTED_MODULE_WILL_BE_STATIC: i32 = 0;
        const PUB_CONST_IN_NESTED_MODULE_WILL_BE_PRIVATE: i32 = 0;

        pub use crate::re_exporter::SPECIAL_CONST as PUB_CONST_IN_NESTED_MODULE_WILL_RE_EXPORT;
        pub use crate::re_exporter::SPECIAL_STATIC as PUB_CONST_IN_NESTED_MODULE_WILL_RE_EXPORT_STATIC;
    }
}
