#![no_std]

mod private {
    pub trait Sealed {}
}

pub trait TraitA {
    unsafe fn feature_added(&self) {}
}

pub trait TraitB {
    #[target_feature(enable = "avx2")]
    unsafe fn already_featured(&self) {}
}

pub trait TraitC {
    unsafe fn add_globally_enabled_feature(&self) {}
}

pub trait TraitD: private::Sealed {
    unsafe fn sealed_trait_method_feature_added(&self) {}
}

trait PrivateTrait {
    unsafe fn private_feature_added(&self) {}
}
