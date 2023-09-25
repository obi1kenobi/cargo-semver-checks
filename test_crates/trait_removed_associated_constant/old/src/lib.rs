pub trait RemovedAssociatedConstantFromTrait {
    const APPLE: i32;
}

pub trait TraitWithConstant {
    const APPLE: i32;
}

pub trait RemovedAssociatedConstantDefaultFromTrait: TraitWithConstant {
    const APPLE: i32 = 2;
}

// this trait is private therefore removing the constant is not breaking
trait PrivateTraitWithTypeRemoved {
    const APPLE: i32;
}

/// This trait should not be reported by the `trait_removed_associated_constant` rule:
/// it will be removed altogether, so the correct rule to catch it is
/// the `trait_missing` rule and not the rule for missing associated constants.
pub trait RemovedTrait {
    const APPLE: i32;
}
