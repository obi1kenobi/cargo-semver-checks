#[cfg(not(feature = "enum_repr_c_removed"))]
#[repr(C)]
pub enum Foo {
    Bar,
}

#[cfg(feature = "enum_repr_c_removed")]
pub enum Foo {
    Bar,
}

#[cfg(not(feature = "enum_repr_c_removed"))]
#[repr(C)]
#[repr(u8)]
pub enum ComplexEquivalentReprEnum {
    Bar,
}

#[cfg(feature = "enum_repr_c_removed")]
#[repr(u8, C)]
pub enum ComplexEquivalentReprEnum {
    Bar,
}

#[cfg(not(feature = "enum_repr_c_removed"))]
#[repr(u8, C)]
pub enum ComplexRemovedReprEnum {
    Bar,
}

#[cfg(feature = "enum_repr_c_removed")]
#[repr(u8)]
pub enum ComplexRemovedReprEnum {
    Bar,
}
