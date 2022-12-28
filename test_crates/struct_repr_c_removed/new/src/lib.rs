pub struct CRemoved {
    pub bar: usize,
}

#[repr(align(16))]
pub struct CRemovedAlign16 {
    pub bar: usize,
}

#[repr(C, align(16))]
pub struct SeparateCAlign16 {
    pub bar: usize,
}
