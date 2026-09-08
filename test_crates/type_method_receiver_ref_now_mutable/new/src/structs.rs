use std::pin::Pin;
use std::rc::Rc;
use std::boxed::Box;

pub struct PublicStruct {
    a: i32,
    b: i32,
}

impl PublicStruct {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
    #[doc(hidden)]
    pub fn hidden_method(&mut self) {}
}

#[doc(hidden)]
pub struct HiddenStruct {
    value: i32,
}

impl HiddenStruct {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}

pub struct StructWithHiddenImpl {
    value: i32,
}

#[doc(hidden)]
impl StructWithHiddenImpl {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}

struct PrivateStruct {
    value: i32
}

impl PrivateStruct {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}