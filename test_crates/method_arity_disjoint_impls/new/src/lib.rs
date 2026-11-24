#![no_std]

pub struct Disjoint<T>(pub T);

impl Disjoint<u8> {
    // Old calls such as `Disjoint::<u8>::converged()` and `Disjoint(0u8).receiver_sibling()`
    // now need an argument. The receiverless u16 function cannot preserve the dot call.
    // TODO: Report all four changes here and `both_changed` on u16 once we can match the impls.
    pub fn partial(_: u8) {}
    pub fn converged(_: u8) {}
    pub fn both_changed(_: u8) {}
    pub fn receiver_sibling(&self, _: u8) {}

    // These methods leave the public API with their parameter counts intact.
    #[doc(hidden)]
    pub fn hidden() {}
    fn private() {}
}

impl Disjoint<u16> {
    pub fn partial() {}
    pub fn converged(_: u8) {}
    pub fn both_changed(_: u8) {}
    pub fn receiver_sibling() {}

    // Unlike `converged`, `removed` disappeared on u8. The three public
    // siblings here all keep their parameters.
    pub fn removed(_: u8) {}
    pub fn hidden(_: u8) {}
    pub fn private(_: u8) {}

    // The sole declaration of `moved` was on u8. This function is new on u16.
    pub fn moved(_: u8) {}

    pub fn call(_: u8) {}
}

pub trait WithReceiver {
    fn call(&self, _: u8) {}
}

pub trait WithTwoArguments {
    fn call(&self, _: u8, _: u8) {}
}

pub trait WithoutReceiver {
    fn call() {}
}

// With this trait in scope, `Disjoint::<u8>::call()` still compiles.
impl WithoutReceiver for Disjoint<u8> {}

pub struct DotCall;
impl DotCall {
    // Receiver removal has its own lint, even when the argument count also changes.
    pub fn loses_receiver(_: u8, _: u8) {}
}

// With either `WithReceiver` or `WithTwoArguments` in scope on its own, `DotCall.call()`
// now takes one or two arguments, respectively. Cite both methods in one report.
// `WithoutReceiver::call` cannot be called with dot syntax.
impl WithReceiver for DotCall {}
impl WithTwoArguments for DotCall {}
impl WithoutReceiver for DotCall {}

pub struct QualifiedCall;

// With only `WithoutReceiver` in scope, `QualifiedCall::call(1)` now takes no arguments.
// `WithReceiver::call` takes a receiver and cannot establish a change to this associated function.
impl WithReceiver for QualifiedCall {}
impl WithoutReceiver for QualifiedCall {}

// Removing the const parameter makes the old type `LosesParameter<1>` unavailable.
pub struct LosesParameter;
impl LosesParameter {
    pub fn call(_: u8) {}
}

// Given `type Old = GainsParameter;`, `Old::call()` loses the function on the default
// instantiation. The new function takes an argument and exists only on `GainsParameter<1>`.
pub struct GainsParameter<const N: u8 = 0>;
impl GainsParameter<1> {
    pub fn call(_: u8) {}
}
