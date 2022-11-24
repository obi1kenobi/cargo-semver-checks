#[cfg(not(feature = "enum_repr_int_removed"))]
#[repr(u8)]
pub enum U8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_removed")]
pub enum U8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_removed"))]
#[repr(i32)]
pub enum I32Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_removed")]
pub enum I32Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_removed"))]
#[repr(isize)]
pub enum IsizeEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_removed")]
pub enum IsizeEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_removed"))]
#[repr(usize)]
pub enum UsizeEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_removed")]
pub enum UsizeEnum {
    Bar,
    Baz,
}
