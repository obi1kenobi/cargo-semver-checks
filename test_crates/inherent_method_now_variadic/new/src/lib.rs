#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

#[repr(C)]
pub struct Methods {
    value: u32,
}

impl Methods {
    // These associated functions and receiver methods should be reported.
    pub unsafe extern "C" fn associated(_: ...) {}
    pub unsafe extern "C" fn with_fixed_argument(_: i32, _: ...) {}
    pub unsafe extern "C" fn shared_receiver(&self, _: ...) {}
    pub unsafe extern "C" fn mutable_receiver(&mut self, _: ...) {}
    pub unsafe extern "C-unwind" fn unwind(_: ...) {}

    // Both variadicness and fixed arity changes should be reported.
    pub unsafe extern "C" fn fixed_parameter_added(_: i32, _: ...) {}
    pub unsafe extern "C" fn fixed_parameter_removed(_: ...) {}

    // Private and doc-hidden methods should not be reported.
    unsafe extern "C" fn private(_: ...) {}
    #[doc(hidden)]
    pub unsafe extern "C" fn hidden(_: ...) {}

    // Methods entering or leaving the public API should not trigger this lint.
    pub unsafe extern "C" fn newly_public(_: ...) {}
    unsafe extern "C" fn newly_private(_: ...) {}
    #[doc(hidden)]
    pub unsafe extern "C" fn newly_hidden(_: ...) {}
    pub unsafe extern "C" fn formerly_hidden(_: ...) {}
}

// The same change is breaking on enum and union owners.
#[repr(u8)]
pub enum Enum {
    Value,
}

impl Enum {
    pub unsafe extern "C" fn associated(_: ...) {}
    pub unsafe extern "C" fn method(&self, _: ...) {}
}

#[repr(C)]
pub union Union {
    pub value: u32,
}

impl Union {
    pub unsafe extern "C" fn associated(_: ...) {}
    pub unsafe extern "C" fn method(&self, _: ...) {}
}

// Reexports remain public API even though the defining module is private.
pub use private_module::Reexported;

mod private_module {
    pub struct Reexported;

    impl Reexported {
        pub unsafe extern "C" fn associated(_: ...) {}
    }

    // This owner has no public importable path and should not be reported.
    pub struct Unreachable;

    impl Unreachable {
        pub unsafe extern "C" fn associated(_: ...) {}
    }
}

// Private and doc-hidden owners should not be reported.
struct Private;

impl Private {
    pub unsafe extern "C" fn associated(_: ...) {}
}

#[doc(hidden)]
pub struct Hidden;

impl Hidden {
    pub unsafe extern "C" fn associated(_: ...) {}
}

// Owners entering or leaving the public API should not trigger this lint.
pub struct NewlyPublic;

impl NewlyPublic {
    pub unsafe extern "C" fn associated(_: ...) {}
}

#[doc(hidden)]
pub struct NewlyHidden;

impl NewlyHidden {
    pub unsafe extern "C" fn associated(_: ...) {}
}

// A generic owner with a single impl should still be reported.
pub struct Generic<T>(core::marker::PhantomData<T>);

impl<T> Generic<T> {
    pub unsafe extern "C" fn associated(_: ...) {}
}

// Adding a method in a disjoint impl must not be mistaken for changing the existing one.
pub struct Disjoint<T>(core::marker::PhantomData<T>);

impl Disjoint<u8> {
    pub unsafe extern "C" fn associated() {}
}

impl Disjoint<u16> {
    pub unsafe extern "C" fn associated(_: ...) {}
}

// Trait methods can replace inherent methods without changing their call syntax.
pub trait VariadicTrait {
    unsafe extern "C" fn associated(_: ...) {}
}

pub trait NonVariadicTrait {
    unsafe extern "C" fn associated() {}
}

// Moving to a trait while changing variadicness should be reported,
// whether the method uses the trait default or overrides it.
pub struct MovedToDefaultTrait;

impl VariadicTrait for MovedToDefaultTrait {}

pub struct MovedToOverriddenTrait;

impl VariadicTrait for MovedToOverriddenTrait {
    unsafe extern "C" fn associated(_: ...) {}
}

// Adding a trait method must not flag an unchanged inherent method of the same name.
pub struct PreservedInherentMethod;

impl PreservedInherentMethod {
    pub unsafe extern "C" fn associated() {}
}

impl VariadicTrait for PreservedInherentMethod {}

// A matching trait method must also suppress reports about other same-name methods.
pub struct MovedToMatchingTrait;

impl NonVariadicTrait for MovedToMatchingTrait {}

impl VariadicTrait for MovedToMatchingTrait {}

// An unchanged trait method does not preserve a changed inherent method that shadows it.
pub struct ChangedInherentWithMatchingTrait;

impl ChangedInherentWithMatchingTrait {
    pub unsafe extern "C" fn associated(_: ...) {}
}

impl NonVariadicTrait for ChangedInherentWithMatchingTrait {}

// Making this inherent method doc-hidden removes its public API guarantee and is breaking.
// The signature itself is unchanged, so inherent_method_now_doc_hidden should report the loss,
// without a variadicness lint reporting the unrelated trait method that it still shadows.
pub struct NewlyHiddenInherentWithTrait;

impl NewlyHiddenInherentWithTrait {
    #[doc(hidden)]
    pub unsafe extern "C" fn associated() {}
}

impl VariadicTrait for NewlyHiddenInherentWithTrait {}

// A doc-hidden trait method cannot preserve the removed inherent method's public API.
// Even if downstream code could keep compiling by using that method, it would lose its
// SemVer guarantee: the hidden method could change without a future major version bump.
// The variadicness change must therefore be reported despite the hidden matching signature.
pub trait HiddenMatchingTrait {
    #[doc(hidden)]
    unsafe extern "C" fn associated() {}
}

// Use a separate default method so this case and MovedToDefaultTrait have distinct
// source spans, keeping snapshot ordering deterministic.
pub trait VariadicTraitForHiddenMatch {
    unsafe extern "C" fn associated(_: ...) {}
}

pub struct MovedToHiddenMatchingTrait;

impl HiddenMatchingTrait for MovedToHiddenMatchingTrait {}

impl VariadicTraitForHiddenMatch for MovedToHiddenMatchingTrait {}

// A hidden overload in a disjoint impl cannot preserve the changed public method.
pub struct ChangedInherentWithHiddenOverload<T>(core::marker::PhantomData<T>);

impl ChangedInherentWithHiddenOverload<u8> {
    pub unsafe extern "C" fn associated(_: ...) {}
}

impl ChangedInherentWithHiddenOverload<u16> {
    #[doc(hidden)]
    pub unsafe extern "C" fn associated() {}
}
