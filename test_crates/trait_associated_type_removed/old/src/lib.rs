#![feature(associated_type_defaults)]
pub trait RemovedTypeFromTrait {
    type Apple;
}

pub trait TraitWithType {
    type MyType;
}

pub trait RemovedTypeDefaultFromTrait: TraitWithType {
    type MyType = u8;
}

// this trait is private therefore removing a type change is not breaking.
trait PrivateTraitWithTypeRemoved {
    type MyType;
}

/// This trait should not be reported by the `trait_associated_type_removed` rule:
/// it will be removed altogether, so the correct rule to catch it is
/// the `trait_missing` rule and not the rule for missing types.
trait RemovedTrait {
    type ATypeInARemovedTrait;
}
