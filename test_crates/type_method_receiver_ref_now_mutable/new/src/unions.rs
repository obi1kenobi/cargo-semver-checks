use std::pin::Pin;
use std::rc::Rc;
use std::boxed::Box;

pub union PublicUnion {
    pub a: i32,
    pub b: f32,
}

impl PublicUnion {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
    #[doc(hidden)]
    pub fn hidden_method(&mut self) {}
}

#[doc(hidden)]
pub union HiddenUnion {
    a: i32,
}

impl HiddenUnion {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}

pub union UnionWithHiddenImpl {
    a: i32,
}

#[doc(hidden)]
impl UnionWithHiddenImpl {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}

union PrivateUnion {
    a: i32,
}

impl PrivateUnion {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}