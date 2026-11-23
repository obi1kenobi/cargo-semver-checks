#![no_std]

pub struct Reader<T>(pub T);
pub struct GainsParameter;
pub struct LosesParameter<const N: u8 = 0>;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::{GainsParameter, LosesParameter, Reader};

    impl Reader<u8> {
        pub fn sum(&self) {}
        pub unsafe fn sum_unchecked(&self) {}

        pub unsafe fn load(&self) {}

        #[target_feature(enable = "avx")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn scan_unchecked(&self) {}

        // Both methods disappear from u8. Only read already has a sibling on u16.
        pub fn read(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn write(&self) {}
    }

    impl Reader<u16> {
        #[target_feature(enable = "avx")]
        pub fn sum(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn sum_unchecked(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn load(&self) {}

        #[target_feature(enable = "avx2")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx,bmi1")]
        pub unsafe fn scan_unchecked(&self) {}

        #[target_feature(enable = "avx")]
        pub fn read(&self) {}
    }

    impl GainsParameter {
        pub fn sum(&self) {}
        pub unsafe fn sum_unchecked(&self) {}
        #[target_feature(enable = "avx")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn scan_unchecked(&self) {}
    }

    impl LosesParameter<1> {
        pub fn sum(&self) {}
        pub unsafe fn sum_unchecked(&self) {}
        #[target_feature(enable = "avx")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn scan_unchecked(&self) {}
    }
}
