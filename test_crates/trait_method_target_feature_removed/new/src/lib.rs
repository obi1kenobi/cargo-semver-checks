#![no_std]

mod private {
    pub trait Sealed {}
}

pub trait TraitA {
    unsafe fn feature_removed(&self) {}
}

pub trait TraitB {
    #[target_feature(enable = "avx2")]
    unsafe fn still_featured(&self) {}
}

pub trait TraitE {
    #[target_feature(enable = "sse2")]
    unsafe fn partial_feature_removed(&self) {}
}

pub trait TraitF {
    #[target_feature(enable = "avx2")]
    unsafe fn consolidated(&self) {}
}

pub trait TraitC {
    unsafe fn remove_globally_enabled_feature(&self) {}
}

pub trait TraitD: private::Sealed {
    unsafe fn sealed_trait_feature_removed(&self) {}
}

trait PrivateTrait {
    unsafe fn private_feature_removed(&self) {}
}
