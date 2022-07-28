#[cfg(not(feature = "enum_repr_c_removed"))]
#[repr(C)]
pub enum Foo {
    Bar,
}

#[cfg(feature = "enum_repr_c_removed")]
pub enum Foo {
    Bar,
}
