#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct Foo {
    pub bar: usize,
}

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct Foo {
    pub bar: usize,
}

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct Bar(pub usize);

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct Bar(pub usize);

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct WithZeroSizedData<T> {
    pub bar: usize,
    _marker: std::marker::PhantomData<T>,
}

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct WithZeroSizedData<T> {
    pub bar: usize,
    _marker: std::marker::PhantomData<T>,
}

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct TupleWithZeroSizedData<T>(pub usize, core::marker::PhantomData<T>);

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct TupleWithZeroSizedData<T>(pub usize, core::marker::PhantomData<T>);

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct WithSpecificZeroSizedData {
    pub bar: usize,
    _marker: std::marker::PhantomData<&'static usize>,
}

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct WithSpecificZeroSizedData {
    pub bar: usize,
    _marker: std::marker::PhantomData<&'static usize>,
}

// Per https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent
// `repr(transparent)` is only part of the public ABI if the single non-zero-sized field
// within the struct is public. In the following structs, the field is not public,
// so removing `repr(transparent)` is not a breaking change since it was never public ABI.

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct FieldNotPublicSoNotPublicAbi {
    pub(crate) bar: usize,
}

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct FieldNotPublicSoNotPublicAbi {
    pub(crate) bar: usize,
}

#[cfg(not(feature = "struct_repr_transparent_removed"))]
#[repr(transparent)]
pub struct TupleFieldNotPublicSoNotPublicAbi(pub(crate) usize);

#[cfg(feature = "struct_repr_transparent_removed")]
pub struct TupleFieldNotPublicSoNotPublicAbi(pub(crate) usize);
