#![no_std]

mod private {
    pub trait Sealed {}
}

pub trait TraitA {
    #[target_feature(enable = "avx2")]
    unsafe fn feature_added(&self) {}
}

pub trait TraitB {
    #[target_feature(enable = "avx2")]
    unsafe fn already_featured(&self) {}
}

pub trait TraitC {
    #[target_feature(enable = "sse2")]
    unsafe fn add_globally_enabled_feature(&self) {}
}

pub trait TraitD: private::Sealed {
    #[target_feature(enable = "avx2")]
    unsafe fn sealed_trait_method_feature_added(&self) {}
}

trait PrivateTrait {
    #[target_feature(enable = "avx2")]
    unsafe fn private_feature_added(&self) {}
}
