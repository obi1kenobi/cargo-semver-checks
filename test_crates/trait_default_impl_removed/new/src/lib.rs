// Default trait method impl is removed, should be reported
pub trait TraitA {
    fn method_default_impl_removed(&self);
}

// Default trait method becomes removed completely, should not be reported as it becomes
// missing instead
pub trait TraitB {}

//Default trait method impl is removed, but its private, shouldnt be reported
trait TraitC {
    fn method_default_impl_removed(&self);
}

// Non-object safe trait has its default impl removed, should be reported
pub trait TraitD {
    fn method_becomes_non_obj_safe(&self) -> Self;
}
