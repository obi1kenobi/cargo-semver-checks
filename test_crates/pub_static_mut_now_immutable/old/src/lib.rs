// Basic Test cases
pub static mut STATIC_A: i32 = 0;
pub static mut STATIC_B: i32 = 0;
static mut STATIC_C: i32 = 0;

// Test case for #[doc(hidden)] pub static
#[doc(hidden)]
pub static mut DOC_HIDDEN_STATIC_A: i32 = 0;

// Renaming or making a static #[doc(hidden)] along with making it immutable
// should trigger only one lint
pub static mut DOC_HIDDEN_STATIC_B: i32 = 0;
pub static mut STATIC_RENAME: i32 = 0;

// Testing for static defined in private module
mod PRIVATE_MODULE {
    pub static mut STATIC_C: i32 = 0;
}

// Testing for static defined in #[doc(hidden)] module
#[doc(hidden)]
pub mod DOC_HIDDEN_MODULE {
    pub static mut STATIC_C: i32 = 0;
}
