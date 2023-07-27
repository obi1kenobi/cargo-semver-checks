#![feature(associated_type_defaults)]
// removing a type from a trait is breaking.
pub trait RemovedTypeFromTrait {}

pub trait TraitWithType {
    type MyType;
}

// removing a default from a type in a trait is breaking, but for a different lint.
pub trait RemovedTypeDefaultFromTrait: TraitWithType {}

// this trait is private therefore removing a type change is not breaking.
trait PrivateTraitWithTypeRemoved {}
