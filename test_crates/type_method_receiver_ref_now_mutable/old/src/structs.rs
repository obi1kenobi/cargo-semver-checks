use std::pin::Pin;
use std::rc::Rc;
use std::boxed::Box;

pub struct PublicStruct {
    a: i32,
    b: i32,
}

impl PublicStruct {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_pc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
}

struct PrivateStruct {
    value: i32
}

impl PrivateStruct {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_pc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
}