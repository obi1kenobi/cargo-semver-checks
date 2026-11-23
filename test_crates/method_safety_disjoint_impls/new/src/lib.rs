#![no_std]

pub struct Reader<T>(pub T);

impl Reader<u8> {
    // `Reader(0u8).read_converged()` now requires an unsafe block, while
    // `unsafe { Reader(0u8).write_converged() }` now warns about the unnecessary block.
    // TODO: Once we can match impl targets, report the partial and converged changes
    // on u8 and the changes to `read_both_changed` and `write_both_changed` on both types.
    pub unsafe fn read_partial(&self) {}
    pub fn write_partial(&self) {}
    pub unsafe fn read_converged(&self) {}
    pub fn write_converged(&self) {}
    pub unsafe fn read_both_changed(&self) {}
    pub fn write_both_changed(&self) {}

    // These methods leave the public API with their original safety requirements.
    #[doc(hidden)]
    pub fn read_hidden(&self) {}
    #[doc(hidden)]
    pub unsafe fn write_hidden(&self) {}
    fn read_private(&self) {}
    unsafe fn write_private(&self) {}
}

impl Reader<u16> {
    pub fn read_partial(&self) {}
    pub unsafe fn write_partial(&self) {}
    pub unsafe fn read_converged(&self) {}
    pub fn write_converged(&self) {}
    pub unsafe fn read_both_changed(&self) {}
    pub fn write_both_changed(&self) {}

    // `Reader(0u8).read_removed()` and `unsafe { Reader(0u8).write_removed() }` lose their
    // methods entirely. The u16 methods keep their original safety requirements here
    // and when the u8 methods become hidden or private.
    pub unsafe fn read_removed(&self) {}
    pub fn write_removed(&self) {}
    pub unsafe fn read_hidden(&self) {}
    pub fn write_hidden(&self) {}
    pub unsafe fn read_private(&self) {}
    pub fn write_private(&self) {}

    // Each name had a single declaration on u8. Calling either on `Reader(0u8)` now
    // fails even inside an unsafe block because the new methods exist only on u16.
    pub unsafe fn read_moved(&self) {}
    pub fn write_moved(&self) {}

    pub unsafe fn read(&self) {}
    pub fn write(&self) {}
}

pub trait Access {
    fn read(&self) {}
    unsafe fn write(&self) {}
}

// With `Access` in scope, `Reader(0u8).read()` still compiles and
// `unsafe { Reader(0u8).write() }` still requires the block.
impl Access for Reader<u8> {}

// The old type `LosesParameter<1>` can no longer be named.
pub struct LosesParameter;
impl LosesParameter {
    pub unsafe fn read() {}
    pub fn write() {}
}

// `type Old = GainsParameter;` now fixes the const argument to 0, so `Old` loses both functions.
// Calling `GainsParameter::read()` directly lets Rust infer 1 and select the new function.
pub struct GainsParameter<const N: u8 = 0>;
impl GainsParameter<1> {
    pub unsafe fn read() {}
    pub fn write() {}
}

pub struct Cursor;
impl Cursor {
    // Report both changes even though the methods themselves are generic.
    // `Cursor.read(0u8)` now needs an unsafe block, and `unsafe { Cursor.write(0u8) }`
    // still compiles but warns under `unused_unsafe`.
    pub unsafe fn read<T>(&self, _: T) {}
    pub fn write<T>(&self, _: T) {}
}
