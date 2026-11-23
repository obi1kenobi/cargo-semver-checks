#![no_std]

pub struct Reader<T>(pub T);

impl Reader<u8> {
    pub fn changed(&self) {}
    pub fn removed(&self) {}
    pub fn hidden(&self) {}
    pub fn private(&self) {}
    pub fn moved(&self) {}
}

impl Reader<u16> {
    pub fn changed(_: &Self) {}
    pub fn removed(_: &Self) {}
    pub fn hidden(_: &Self) {}
    pub fn private(_: &Self) {}
}

pub struct GainsParameter;
impl GainsParameter {
    pub fn read(&self) {}
}

pub struct LosesParameter<const N: usize = 0>;
impl LosesParameter<1> {
    pub fn read(&self) {}
}

pub trait Read {
    fn changed(_: &Self) {}
    fn hidden(_: &Self) {}
    fn from_trait(_: &Self) {}
    fn preserved(&self) {}
    fn public_const(_: &Self) {}
    fn private_const(_: &Self) {}
}

pub trait OtherRead {
    fn from_trait(_: &Self) {}
}

pub struct Lookup;
impl Read for Lookup {}
impl OtherRead for Lookup {}

impl Lookup {
    pub fn changed(&self) {}
    pub fn hidden(&self) {}
    pub fn from_trait(&self) {}
    pub fn preserved(&self) {}
    pub fn public_const(&self) {}
    pub fn private_const(&self) {}
}
