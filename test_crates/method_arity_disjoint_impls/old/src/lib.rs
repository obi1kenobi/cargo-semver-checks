#![no_std]

pub struct Disjoint<T>(pub T);

impl Disjoint<u8> {
    pub fn partial() {}
    pub fn converged() {}
    pub fn both_changed() {}
    pub fn receiver_sibling(&self) {}

    pub fn removed() {}
    pub fn hidden() {}
    pub fn private() {}
    pub fn moved() {}

    pub fn call() {}
}

impl Disjoint<u16> {
    pub fn partial() {}
    pub fn converged(_: u8) {}
    pub fn both_changed() {}
    pub fn receiver_sibling() {}

    pub fn removed(_: u8) {}
    pub fn hidden(_: u8) {}
    pub fn private(_: u8) {}

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

impl WithoutReceiver for Disjoint<u8> {}

pub struct DotCall;
impl DotCall {
    pub fn call(&self) {}
    pub fn loses_receiver(&self) {}
}

impl WithReceiver for DotCall {}
impl WithTwoArguments for DotCall {}
impl WithoutReceiver for DotCall {}

pub struct QualifiedCall;
impl QualifiedCall {
    pub fn call(_: u8) {}
}

impl WithReceiver for QualifiedCall {}
impl WithoutReceiver for QualifiedCall {}

pub struct LosesParameter<const N: u8 = 0>;
impl LosesParameter<1> {
    pub fn call() {}
}

pub struct GainsParameter;
impl GainsParameter {
    pub fn call() {}
}
