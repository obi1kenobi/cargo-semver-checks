#![no_std]

// This should not trigger this lint.
pub static ExistingStatic: i32 = 0;

// This should not trigger this lint.
pub const PUB_CONST_IN_GLOBAL_WILL_BE_STATIC: i32 = 0;

pub mod my_module {
    pub static ExistingStaticinModule: i32 = 0;
}
