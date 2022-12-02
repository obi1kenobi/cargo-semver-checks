pub struct SizedStruct {
    // Slices are not Sized.
    bar: [usize],
}
