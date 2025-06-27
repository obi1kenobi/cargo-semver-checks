#![no_std]

pub struct SizedStruct {
    // Slices are not Sized.
    bar: [usize],
}
