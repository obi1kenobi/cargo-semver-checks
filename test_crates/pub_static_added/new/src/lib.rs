#![no_std]

// This should not trigger this lint.
pub static EXISTING_STATIC_IN_GLOBAL: i32 = 0;

// private static item added, this should not trigger this lint.
static NEW_PRIVATE_STATIC_IN_GLOBAL: i32 = 0;

pub static NEW_PUB_STATIC_IN_GLOBAL: i32 = 0;

pub mod my_module {
    pub static EXISTING_STATIC_IN_MODULE: i32 = 0;
    pub static NEW_PUB_STATIC_IN_MODULE: i32 = 0;
}
