#![no_std]

pub struct Reader<T>(pub T);

impl Reader<u8> {
    pub fn read_partial(&self) {}
    pub unsafe fn write_partial(&self) {}
    pub fn read_converged(&self) {}
    pub unsafe fn write_converged(&self) {}
    pub fn read_both_changed(&self) {}
    pub unsafe fn write_both_changed(&self) {}

    pub fn read_removed(&self) {}
    pub unsafe fn write_removed(&self) {}
    pub fn read_hidden(&self) {}
    pub unsafe fn write_hidden(&self) {}
    pub fn read_private(&self) {}
    pub unsafe fn write_private(&self) {}
    pub fn read_moved(&self) {}
    pub unsafe fn write_moved(&self) {}

    pub fn read(&self) {}
    pub unsafe fn write(&self) {}
}

impl Reader<u16> {
    pub fn read_partial(&self) {}
    pub unsafe fn write_partial(&self) {}
    pub unsafe fn read_converged(&self) {}
    pub fn write_converged(&self) {}
    pub fn read_both_changed(&self) {}
    pub unsafe fn write_both_changed(&self) {}

    pub unsafe fn read_removed(&self) {}
    pub fn write_removed(&self) {}
    pub unsafe fn read_hidden(&self) {}
    pub fn write_hidden(&self) {}
    pub unsafe fn read_private(&self) {}
    pub fn write_private(&self) {}

    pub unsafe fn read(&self) {}
    pub fn write(&self) {}
}

pub trait Access {
    fn read(&self) {}
    unsafe fn write(&self) {}
}

pub struct LosesParameter<const N: u8 = 0>;
impl LosesParameter<1> {
    pub fn read() {}
    pub unsafe fn write() {}
}

pub struct GainsParameter;
impl GainsParameter {
    pub fn read() {}
    pub unsafe fn write() {}
}

pub struct Cursor;
impl Cursor {
    pub fn read<T>(&self, _: T) {}
    pub unsafe fn write<T>(&self, _: T) {}
}
