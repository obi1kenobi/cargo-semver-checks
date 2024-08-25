mod private {
    pub trait Sealed {}
    pub struct Token;
}

// Default trait method impl is removed, should be reported
pub trait TraitA {
    fn method_default_impl_removed(&self);
}

// Default trait method becomes removed completely, should not be reported as it becomes
// missing instead
pub trait TraitB {}

// Default trait method impl is removed, but it's private, shouldn't be reported
trait TraitC {
    fn method_default_impl_removed(&self);
}

// Default trait method impl is removed, and becomes non-object-safe, should be reported
pub trait TraitD {
    fn method_default_impl_removed_and_becomes_non_obj_safe(&self) -> Self;
}

// Default trait method impl is removed, and becomes sealed, should be reported
pub trait TraitE {
    fn method_default_impl_removed_and_becomes_sealed(&self, _: private::Token);
}

// Default trait method impl is removed, and it's partially sealed, should be reported
pub trait TraitF {
    fn method_partially_sealed_has_default_impl_removed(&self, _: private::Token);
}

// Default trait method impl is removed, but it's sealed. The fact that it's sealed dominates,
// thus having its default impl removed is not a problem since it was never possible to
// implement it, should not be reported
pub trait TraitG: private::Sealed {
    fn method_sealed_has_default_impl_removed(&self);
}
