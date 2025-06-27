#![no_std]

pub static PUB_STATIC_IN_GLOBAL: i32 = 0;
pub static PUB_STATIC_IN_GLOBAL_IS_RENAMED: i32 = 0;
pub const PUB_STATIC_IN_GLOBAL_WILL_BE_CONST: i32 = 0;
const PUB_STATIC_IN_GLOBAL_WILL_BE_PRIVATE_CONST: i32 = 0;
static PUB_STATIC_IN_GLOBAL_WILL_BE_PRIVATE_STATIC: i32 = 0;

mod re_exporter {
    pub const SPECIAL_CONST: i32 = 0;
    pub static SPECIAL_STATIC: i32 = 0;
}

pub mod my_module {
    pub static PUB_STATIC_IN_MODULE: i32 = 0;
    pub static PUB_STATIC_IN_MODULE_IS_RENAMED: i32 = 0;
    pub const PUB_STATIC_IN_MODULE_WILL_BE_CONST: i32 = 0;
    const PUB_STATIC_IN_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
    static PUB_STATIC_IN_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
    pub use crate::re_exporter::SPECIAL_CONST as PUB_STATIC_IN_MODULE_WILL_RE_EXPORT_CONST;
    pub use crate::re_exporter::SPECIAL_STATIC as PUB_STATIC_IN_MODULE_WILL_RE_EXPORT;

    pub mod my_module_nested {
        pub static PUB_STATIC_IN_NESTED_MODULE: i32 = 0;
        pub static PUB_STATIC_IN_NESTED_MODULE_IS_RENAMED: i32 = 0;
        pub const PUB_STATIC_IN_NESTED_MODULE_WILL_BE_CONST: i32 = 0;
        const PUB_STATIC_IN_NESTED_MODULE_WILL_BE_PRIVATE_CONST: i32 = 0;
        static PUB_STATIC_IN_NESTED_MODULE_WILL_BE_PRIVATE_STATIC: i32 = 0;
        pub use crate::re_exporter::SPECIAL_CONST as PUB_STATIC_IN_NESTED_MODULE_WILL_RE_EXPORT_CONST;
        pub use crate::re_exporter::SPECIAL_STATIC as PUB_STATIC_IN_NESTED_MODULE_WILL_RE_EXPORT;
    }
}
