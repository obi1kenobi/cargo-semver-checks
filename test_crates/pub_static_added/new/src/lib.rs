#![no_std]

// This should not trigger this lint.
pub static ExistingStatic: i32 = 0;

// This should not trigger this lint.
pub static PUB_CONST_IN_GLOBAL_WILL_BE_STATIC: i32 = 0;

// private static item added, this should not trigger this lint.
static PrivateStatic: i32 = 0;

// const item added, this should not trigger this lint.
pub const NewConst: i32 = 0;

pub static NewStatic: i32 = 0;

pub mod my_module {
    pub static ExistingStaticinModule: i32 = 0;
    pub static NewStaticinModule: i32 = 0; 
}
