#![no_std]

// Basic Test cases
pub static STATIC_A: i32 = 0;
pub static mut STATIC_B: i32 = 0;
static STATIC_C: i32 = 0;

// Test case for #[doc(hidden)] pub static
#[doc(hidden)]
pub static DOC_HIDDEN_STATIC_A: i32 = 0;

// Renaming or making a static #[doc(hidden)] along with making it immutable
// should trigger only one lint
#[doc(hidden)]
pub static DOC_HIDDEN_STATIC_B: i32 = 0;
pub static STATIC_RENAMED: i32 = 0;

// Testing for static defined in private module
mod PRIVATE_MODULE {
    pub static STATIC_C: i32 = 0;
}

// Testing for static defined in #[doc(hidden)] module
#[doc(hidden)]
pub mod DOC_HIDDEN_MODULE {
    pub static STATIC_C: i32 = 0;
}
