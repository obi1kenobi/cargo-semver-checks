#![no_std]

// This should not trigger this lint.
pub const ExistingConst: i32 = 0;

// This should not trigger this lint.
pub const PUB_STATIC_IN_GLOBAL_WILL_BE_CONST: i32 = 0;

// private const item added, this should not trigger this lint.
const PrivateConst: i32 = 0;

// static item added, this should not trigger this lint.
pub static NewStatic: i32 = 0;

pub const NewConst: i32 = 0;

pub mod my_module {
    pub const ExistingConstInModule: i32 = 0;
    pub const NewConstInModule: i32 = 0; 
}
