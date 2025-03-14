#![no_std]

// This should not trigger this lint.
pub const ExistingConst: i32 = 0;

// This should not trigger this lint.
pub static PUB_STATIC_IN_GLOBAL_WILL_BE_CONST: i32 = 0;

pub mod my_module {
    pub const ExistingConstInModule: i32 = 0;
}
