#[cfg(not(feature = "struct_repr_c_removed"))]
#[repr(C)]
pub struct CRemoved {
    pub bar: usize,
}

#[cfg(feature = "struct_repr_c_removed")]
pub struct CRemoved {
    pub bar: usize,
}

#[cfg(not(feature = "struct_repr_c_removed"))]
#[repr(C, align(16))]
pub struct CRemovedAlign16 {
    pub bar: usize,
}

#[cfg(feature = "struct_repr_c_removed")]
#[repr(align(16))]
pub struct CRemovedAlign16 {
    pub bar: usize,
}

#[cfg(not(feature = "struct_repr_c_removed"))]
#[repr(C)]
#[repr(align(16))]
pub struct SeparateCAlign16 {
    pub bar: usize,
}

#[cfg(feature = "struct_repr_c_removed")]
#[repr(C, align(16))]
pub struct SeparateCAlign16 {
    pub bar: usize,
}
