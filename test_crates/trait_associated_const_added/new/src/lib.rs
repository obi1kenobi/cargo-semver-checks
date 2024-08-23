mod private {
    pub(crate) trait Sealed {}
}

pub trait Foo {
    const BAR: bool;
}

pub trait FooSealed: private::Sealed {
    const BAR: bool;
}
pub trait FooNotSealed: private::Sealed {
    const BAR: bool;
}

pub trait Bar {
    const BAR: bool = true;
}
pub trait BarSealed: private::Sealed {
    const BAR: bool = true;
}

pub trait Baz {
    const ONE: bool;
    const TWO: bool;
}
pub trait BazSealed: private::Sealed {
    const ONE: bool;
    const TWO: bool;
}
