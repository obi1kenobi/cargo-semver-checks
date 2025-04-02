use std::pin::Pin;
use std::rc::Rc;
use std::boxed::Box;

pub enum PublicEnum {
    Ready,
    Busy(u32),
}

impl PublicEnum {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_pc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}

enum PrivateEnum {
    A,
}

impl PrivateEnum {
    pub fn public_method(&mut self) {}
    pub fn public_method_require_box(self: &mut Box<Self>) {}
    pub fn public_method_require_pin(self: &mut Pin<&mut Self>) {}
    pub fn public_method_require_pc_box(self:&mut Rc<Box<Self>>) {}
    fn private_method(&mut self) {}
}