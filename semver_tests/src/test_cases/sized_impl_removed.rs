#[cfg(not(feature = "sized_impl_removed"))]
pub struct SizedStruct {
    bar: [usize; 1],
}

#[cfg(feature = "sized_impl_removed")]
pub struct SizedStruct {
    // Slices are not Sized.
    bar: [usize],
}
