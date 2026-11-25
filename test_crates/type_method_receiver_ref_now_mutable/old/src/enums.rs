use std::pin::Pin;
use std::rc::Rc;
use std::boxed::Box;

pub enum PublicEnum {
    Ready,
    Busy(u32),
}

impl PublicEnum {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
    #[doc(hidden)]
    pub fn hidden_method(&self) {}
}

#[doc(hidden)]
pub enum HiddenEnum {
    A,
}

impl HiddenEnum {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
}

pub enum EnumWithHiddenImpl {
    A,
}

#[doc(hidden)]
impl EnumWithHiddenImpl {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
}

enum PrivateEnum {
    A,
}

impl PrivateEnum {
    pub fn public_method(&self) {}
    pub fn public_method_require_box(self: &Box<Self>) {}
    pub fn public_method_require_pin(self: &Pin<&mut Self>) {}
    pub fn public_method_require_rc_box(self: &Rc<Box<Self>>) {}
    fn private_method(&self) {}
}