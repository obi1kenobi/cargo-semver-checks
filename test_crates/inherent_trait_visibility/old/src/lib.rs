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

pub struct Removed;
impl Private for Removed {}
impl traits::Unexported for Removed {}
impl Removed {
    pub unsafe fn read<T, U, const N: usize, const M: usize>(&self, _: u8, _: u8) {}
    pub fn seek(&self) {}
    pub fn reset(&self) {}
    pub const fn limit() -> u8 {
        4
    }
    pub const LIMIT: u8 = 4;
    pub fn stranded() {}
    pub const STRANDED: u8 = 4;
}

pub struct Changed;
impl Private for Changed {}
impl Reader for Changed {}
impl OtherReader for Changed {}
impl Changed {
    pub fn read<T, const N: usize>(&self, _: u8) {}
    pub unsafe fn seek(&self) {}
    pub const fn limit() -> u8 {
        4
    }
    pub const LIMIT: u8 = 4;
}

pub struct FromGlob;
impl OtherReader for FromGlob {}
impl FromGlob {
    pub fn read<T, const N: usize>(&self, _: u8) {}
    pub unsafe fn seek(&self) {}
    pub const fn limit() -> u8 {
        4
    }
    pub const LIMIT: u8 = 4;
}

pub struct Preserved;
impl Reader for Preserved {}
impl traits::Original for Preserved {}
impl Preserved {
    pub fn from_prelude() {}
    pub const FROM_PRELUDE: u8 = 4;
    pub fn read<T, const N: usize>(&self, _: u8) {}
    pub unsafe fn seek(&self) {}
    pub const LIMIT: u8 = 4;
}
