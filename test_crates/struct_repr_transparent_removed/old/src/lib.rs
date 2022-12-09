#[repr(transparent)]
pub struct Foo {
    pub bar: usize,
}

#[repr(transparent)]
pub struct Bar(pub usize);

#[repr(transparent)]
pub struct WithZeroSizedData<T> {
    pub bar: usize,
    _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct TupleWithZeroSizedData<T>(pub usize, core::marker::PhantomData<T>);

#[repr(transparent)]
pub struct WithPubZeroSizedData<T> {
    pub bar: usize,
    pub _marker: std::marker::PhantomData<T>,
}

#[repr(transparent)]
pub struct WithSpecificZeroSizedData {
    pub bar: usize,
    _marker: std::marker::PhantomData<&'static usize>,
}

#[repr(transparent)]
pub struct WithFoo {
    pub bar: Foo,
    _marker: std::marker::PhantomData<&'static usize>,
}

#[repr(transparent)]
pub struct WithRef {
    pub bar: &'static usize,
    _marker: std::marker::PhantomData<&'static usize>,
}

#[repr(transparent)]
pub struct WithTuple {
    pub bar: (usize, i64),
    _marker: std::marker::PhantomData<&'static usize>,
}

#[repr(transparent)]
pub struct WithGeneric {
    pub bar: WithZeroSizedData<usize>,
    _marker: std::marker::PhantomData<&'static usize>,
}

// Per https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
// `repr(transparent)` is only part of the public ABI if the single non-zero-sized field
// within the struct is public. In the following structs, the field is not public,
// so removing `repr(transparent)` is not a breaking change since it was never public ABI.

#[repr(transparent)]
pub struct FieldNotPublicSoNotPublicAbi {
    pub(crate) bar: usize,
}

#[repr(transparent)]
pub struct TupleFieldNotPublicSoNotPublicAbi(pub(crate) usize);
