mod private {
    pub(crate) trait Sealed {}
}

pub trait Foo {}

pub trait FooSealed: private::Sealed {}
pub trait FooNotSealed {}

pub trait Bar {}
pub trait BarSealed: private::Sealed {}

pub trait Baz {
    const ONE: bool;
}
pub trait BazSealed: private::Sealed {
    const ONE: bool;
}
