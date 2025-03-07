#![no_std]

pub trait RemovedAssociatedConstantFromTrait {
}

pub trait TraitWithConstant {
    const APPLE: i32;
}

pub trait RemovedAssociatedConstantDefaultFromTrait: TraitWithConstant {}

// this trait is private therefore removing the constant is not breaking
trait PrivateTraitWithTypeRemoved {}
