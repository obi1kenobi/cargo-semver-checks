#![no_std]

mod private {
    pub trait Sealed {}
}

pub trait TraitA {
    #[target_feature(enable = "avx")]
    unsafe fn unsafe_method(&self) {}
}

pub trait TraitSealed: private::Sealed {
    #[target_feature(enable = "avx")]
    unsafe fn sealed_trait_method(&self) {}
}

pub trait TraitImpliedFeature {
    #[target_feature(enable = "avx2")]
    unsafe fn implied_feature_method(&self) {}
}

pub trait TraitGloballyEnabled {
    #[target_feature(enable = "bmi1")]
    unsafe fn globally_enabled_method(&self) {}
}
