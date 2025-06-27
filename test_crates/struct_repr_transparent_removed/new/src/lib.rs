#![no_std]

pub struct Foo {
    pub bar: usize,
}

pub struct Bar(pub usize);

pub struct WithZeroSizedData<T> {
    pub bar: usize,
    _marker: core::marker::PhantomData<T>,
}

pub struct TupleWithZeroSizedData<T>(pub usize, core::marker::PhantomData<T>);

pub struct WithPubZeroSizedData<T> {
    pub bar: usize,
    pub _marker: core::marker::PhantomData<T>,
}

pub struct WithSpecificZeroSizedData {
    pub bar: usize,
    _marker: core::marker::PhantomData<&'static usize>,
}

pub struct WithFoo {
    pub bar: Foo,
    _marker: core::marker::PhantomData<&'static usize>,
}

pub struct WithRef {
    pub bar: &'static usize,
    _marker: core::marker::PhantomData<&'static usize>,
}

pub struct WithTuple {
    pub bar: (usize, i64),
    _marker: core::marker::PhantomData<&'static usize>,
}

pub struct WithGeneric {
    pub bar: WithZeroSizedData<usize>,
    _marker: core::marker::PhantomData<&'static usize>,
}

pub struct FieldNotPublicSoNotPublicAbi {
    pub(crate) bar: usize,
}

pub struct TupleFieldNotPublicSoNotPublicAbi(pub(crate) usize);

#[repr(transparent)]
pub struct BothTransparent {
    pub bar: usize,
}
