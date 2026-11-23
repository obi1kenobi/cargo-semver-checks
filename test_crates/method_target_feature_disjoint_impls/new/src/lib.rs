#![no_std]

pub struct Reader<T>(pub T);
pub struct GainsParameter<const N: u8 = 0>;
pub struct LosesParameter;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::{GainsParameter, LosesParameter, Reader};

    impl Reader<u8> {
        // On x86-64, SSE2 is enabled by default. Adding it to sum prevents
        // `Reader::<u8>::sum` from implementing `Fn`. Adding it to sum_unchecked
        // places no new CPU requirement on callers. The u16 methods still require AVX.
        // TODO: Report the change to sum on u8 once we can match impl targets.
        #[target_feature(enable = "sse2")]
        pub fn sum(&self) {}
        #[target_feature(enable = "sse2")]
        pub unsafe fn sum_unchecked(&self) {}

        // The unsafe method now needs AVX on u8, just as it already did on u16.
        // TODO: Report the new precondition on u8 once we can match impl targets.
        #[target_feature(enable = "avx")]
        pub unsafe fn load(&self) {}

        // Both u8 methods gain AVX2. On u16, scan already required AVX2.
        // Matching scan_unchecked by name could also report BMI1, even though
        // the u8 method never required it and the u16 method already did.
        // TODO: Report AVX2 on the two u8 methods once we can match impl targets.
        #[target_feature(enable = "avx2")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx2")]
        pub unsafe fn scan_unchecked(&self) {}
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

        // Calls on u8 lose read. The surviving read on u16 has required AVX throughout.
        #[target_feature(enable = "avx")]
        pub fn read(&self) {}

        // The sole write method moved from u8 to u16. Its AVX2 requirement
        // cannot be attributed to the old method on u8.
        #[target_feature(enable = "avx2")]
        pub unsafe fn write(&self) {}
    }

    impl GainsParameter<1> {
        // An old alias `type R = GainsParameter;` now denotes GainsParameter<0>.
        // The methods on GainsParameter<1> cannot impose requirements on calls through R.
        #[target_feature(enable = "avx")]
        pub fn sum(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn sum_unchecked(&self) {}
        #[target_feature(enable = "avx2")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx2")]
        pub unsafe fn scan_unchecked(&self) {}
    }

    impl LosesParameter {
        // The old methods belonged to LosesParameter<1>. Callers that named
        // that type can no longer use it, regardless of the features on the new methods.
        #[target_feature(enable = "avx")]
        pub fn sum(&self) {}
        #[target_feature(enable = "avx")]
        pub unsafe fn sum_unchecked(&self) {}
        #[target_feature(enable = "avx2")]
        pub fn scan(&self) {}
        #[target_feature(enable = "avx2")]
        pub unsafe fn scan_unchecked(&self) {}
    }
}
