#![no_std]

trait Private {
    fn read<T, const N: usize>(&self, _: u8) {}
    unsafe fn seek(&self) {}
    fn reset() {}
    fn limit() -> u8 {
        4
    }
    const LIMIT: u8 = 4;
}

mod traits {
    pub trait Reader {
        unsafe fn read<T, U, const N: usize, const M: usize>(&self, _: u8, _: u8) {}
        fn seek(_: &Self) {}
        fn limit() -> u8 {
            4
        }
        const LIMIT: u8 = 4;
    }

    pub trait OtherReader {
        unsafe fn read<T, U, V, const N: usize, const M: usize, const K: usize>(
            &self,
            _: u8,
            _: u8,
            _: u8,
        ) {
        }
        fn seek(_: &Self) {}
        fn limit() -> u8 {
            4
        }
        const LIMIT: u8 = 4;
    }

    pub trait Original {
        fn from_prelude() {}
        const FROM_PRELUDE: u8 = 4;
        fn read<T, const N: usize>(&self, _: u8) {}
        unsafe fn seek(&self) {}
        const LIMIT: u8 = 4;
    }

    pub trait Unexported {
        fn stranded() {}
        const STRANDED: u8 = 4;
    }
}

#[doc(hidden)]
pub use traits::OtherReader;
pub use traits::Reader;
pub use traits::Reader as ReaderAlias;
pub mod prelude {
    pub use crate::traits::Original as _;
    pub use crate::traits::OtherReader as _;
}

// Only the private trait still provides `read`, `seek`, `reset`, `limit`, and `LIMIT`.
// Downstream callers lose those items, so report their removal.
//
// Callers also lose `stranded` and `STRANDED`. The removal lints miss these because neither
// `Unexported` nor a trait re-exported only as `_` has a named public API path in the adapter.
pub struct Removed;
impl Private for Removed {}
impl traits::Unexported for Removed {}

// With `Reader` in scope, `read` now requires an unsafe call, another argument,
// and two more generic arguments (one type and one const). The private trait's `read`
// cannot preserve the old call. Report only `Reader::read`, even though it has two public names.
//
// `seek` loses its receiver and becomes safe. `Changed::seek(&value)` still works,
// but `value.seek()` fails and an unsafe block around the qualified call is now redundant.
//
// A const call to `Changed::limit()` fails because `Reader::limit` is an ordinary function.
// `Changed::LIMIT` remains available through `Reader`.
pub struct Changed;
impl Private for Changed {}
impl Reader for Changed {}
impl OtherReader for Changed {}

// A glob import of `prelude` brings the replacement methods into scope. The lints miss their
// changed signatures and loss of constness because the only named path to `OtherReader` is doc-hidden.
// The same glob import still makes `FromGlob::LIMIT` available.
pub struct FromGlob;
impl OtherReader for FromGlob {}

// Importing `prelude::*` keeps the old `read` and `seek` calls valid through `Original`.
// Importing `Reader` instead would select their changed signatures.
// The glob is also the only way to keep using `Preserved::from_prelude()` and `Preserved::FROM_PRELUDE`.
pub struct Preserved;
impl Reader for Preserved {}
impl traits::Original for Preserved {}
