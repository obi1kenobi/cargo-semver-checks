#![no_std]

#[repr(transparent)]
pub enum Foo {
    Bar(usize),
}

#[repr(transparent)]
pub enum Bar {
    Baz { quux: usize },
}

#[repr(transparent)]
pub enum StructStyleWithZeroSizedData<T> {
    Bar {
        bar: usize,
        _marker: core::marker::PhantomData<T>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithZeroSizedData<T> {
    Bar(usize, core::marker::PhantomData<T>),
}

#[repr(transparent)]
pub enum StructStyleWithSpecificZeroSizedData {
    Bar {
        bar: usize,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithSpecificZeroSizedData {
    Bar(usize, core::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithFoo {
    Bar {
        bar: Foo,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithFoo {
    Bar(Foo, core::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithRef {
    Bar {
        bar: &'static usize,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithRef {
    Bar(&'static usize, core::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithTupleStyle {
    Bar {
        bar: (usize, i64),
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithTuple {
    Bar((usize, i64), core::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithGeneric {
    Bar {
        bar: StructStyleWithZeroSizedData<usize>,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithGeneric {
    Bar(
        StructStyleWithZeroSizedData<usize>,
        core::marker::PhantomData<&'static usize>,
    ),
}

// A trailing comma corner case - checks if attributes are parsed correctly.

#[repr(transparent, )]
pub enum TupleStyleTrailingComma {
    Foo(usize),
}

#[repr(transparent, )]
pub enum StructStyleTrailingComma {
    Foo { bar: usize },
}
