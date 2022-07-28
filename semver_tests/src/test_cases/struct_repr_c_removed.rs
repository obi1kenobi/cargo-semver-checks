#[cfg(not(feature = "struct_repr_c_removed"))]
#[repr(C)]
pub struct Foo {
    pub bar: usize,
}

#[cfg(feature = "struct_repr_c_removed")]
pub struct Foo {
    pub bar: usize,
}
