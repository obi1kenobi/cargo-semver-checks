#![no_std]
// Rustdoc fixtures also run on Rust versions predating
// stabilization of C-variadic definitions in Rust 1.99.
// TODO: Remove the feature gate and stable_features allowance when Rust 1.99 is the MSRV.
#![feature(c_variadic)]
#![allow(stable_features)]

// The u8 functions change while the u16 functions keep their original signatures.
// TODO: Report the two u8 changes once impl targets can be matched.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}
impl Partial<u16> {
    pub unsafe extern "C-unwind" fn becomes_variadic(_: i32, _: i64) {}
    pub unsafe extern "C-unwind" fn loses_variadic(_: i32, _: i64, _: ...) {}
}

// On both impls, adding or removing `...` breaks assignments to the old pointer types.
// TODO: Report both declarations in each direction once impl targets can be matched.
pub struct BothChanged<T>(pub T);
impl BothChanged<u8> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}
impl BothChanged<u16> {
    pub unsafe extern "C-unwind" fn becomes_variadic(_: i32, _: i64, _: ...) {}
    pub unsafe extern "C-unwind" fn loses_variadic(_: i32, _: i64) {}
}

// The u8 functions adopt the signatures of their unchanged u16 siblings.
// TODO: Report the changes on u8 once impl matching can distinguish them from removal.
pub struct Converged<T>(pub T);
impl Converged<u8> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}
impl Converged<u16> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

// Removing the u8 functions leaves the same u16 signatures as in Converged.
pub struct RemovedOriginal<T>(pub T);
impl RemovedOriginal<u16> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

// The only declaration of each function moves to a different concrete type.
// Assignments to the u8 functions break because the functions disappear.
pub struct Moved<T>(pub T);
impl Moved<u16> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

pub trait OriginalSignatures {
    unsafe extern "C" fn becomes_variadic() {}
    unsafe extern "C" fn loses_variadic(_: ...) {}
}

// With OriginalSignatures in scope, TraitFallback::<u8>::becomes_variadic and
// TraitFallback::<u8>::loses_variadic still coerce to their old pointer types.
pub struct TraitFallback<T>(pub T);
impl OriginalSignatures for TraitFallback<u8> {}
impl TraitFallback<u16> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

// The new methods exist only on GainsParameter<1>. Downstream code using
// `type Old = GainsParameter;` loses both functions on the default instantiation.
pub struct GainsParameter<const N: usize = 0>;
impl GainsParameter<1> {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

// Removing the const parameter makes the original owner, LosesParameter<1>,
// unavailable to downstream callers.
pub struct LosesParameter;
impl LosesParameter {
    pub unsafe extern "C" fn becomes_variadic(_: ...) {}
    pub unsafe extern "C" fn loses_variadic() {}
}

// Each trait also has a function with the old variadicness. Its different name
// means it cannot preserve a caller's `Owner::call` function pointer.
pub trait VariadicCall {
    unsafe extern "C" fn call(_: i32, _: ...) {}
    fn other() {}
}

pub trait FixedCall {
    unsafe extern "C-unwind" fn call(_: i32, _: i64) {}
    unsafe extern "C" fn other(_: ...) {}
}

// Inherent functions take precedence over in-scope trait functions. Report each
// change at the inherent declaration for both public paths to the owner.
pub struct BecomesVariadic;
pub use BecomesVariadic as ReexportedBecomesVariadic;
impl VariadicCall for BecomesVariadic {}
impl BecomesVariadic {
    pub unsafe extern "C-unwind" fn call(_: i32, _: ...) {}
}

pub struct LosesVariadic;
pub use LosesVariadic as ReexportedLosesVariadic;
impl FixedCall for LosesVariadic {}
impl LosesVariadic {
    pub unsafe extern "C" fn call(_: i32, _: i64) {}
}

// Even after the inherent functions leave the supported API, they shadow
// the in-scope trait functions. Report only the loss of API visibility.
pub struct HiddenBecomesVariadic;
impl VariadicCall for HiddenBecomesVariadic {}
impl HiddenBecomesVariadic {
    #[doc(hidden)]
    pub unsafe extern "C-unwind" fn call(_: i32, _: ...) {}
}

pub struct HiddenLosesVariadic;
impl FixedCall for HiddenLosesVariadic {}
impl HiddenLosesVariadic {
    #[doc(hidden)]
    pub unsafe extern "C" fn call(_: i32, _: i64) {}
}

// Once the inherent functions become private, downstream lookup selects
// VariadicCall::call or FixedCall::call when the corresponding trait is in scope.
// The public inherent function `other` only shadows a function by that name.
// Report each change to `call` at the selected trait function.
pub struct PrivateBecomesVariadic;
impl VariadicCall for PrivateBecomesVariadic {}
impl PrivateBecomesVariadic {
    pub fn other() {}
    unsafe extern "C-unwind" fn call(_: i32) {}
}

pub struct PrivateLosesVariadic;
impl FixedCall for PrivateLosesVariadic {}
impl PrivateLosesVariadic {
    pub fn other() {}
    unsafe extern "C" fn call(_: i32, _: i64, _: ...) {}
}
