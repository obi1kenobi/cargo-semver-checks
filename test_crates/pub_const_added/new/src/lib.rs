#![no_std]

// This should not trigger this lint.
pub const EXISTING_CONST_IN_GLOBAL: i32 = 0;

// This should not trigger this lint.
pub const PUB_STATIC_IN_GLOBAL_WILL_BE_CONST: i32 = 0;

// private const item added, this should not trigger this lint.
const NEW_PRIVATE_CONST_IN_GLOBAL: i32 = 0;

// static item added, this should not trigger this lint.
pub static NEW_PUB_STATIC_IN_GLOBAL: i32 = 0;

pub const NEW_PUB_CONST_IN_GLOBAL: i32 = 0;

pub mod my_module {
    pub const EXISTING_CONST_IN_MODULE: i32 = 0;
    pub const NEW_PUB_CONST_IN_MODULE: i32 = 0; 
}
