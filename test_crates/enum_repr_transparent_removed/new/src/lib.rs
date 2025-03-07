#![no_std]

pub enum Foo {
    Bar(usize),
}

pub enum Bar {
    Baz { quux: usize },
}

pub enum StructStyleWithZeroSizedData<T> {
    Bar {
        bar: usize,
        _marker: core::marker::PhantomData<T>,
    },
}

pub enum TupleStyleWithZeroSizedData<T> {
    Bar(usize, core::marker::PhantomData<T>),
}

pub enum StructStyleWithFoo {
    Bar {
        bar: Foo,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

pub enum TupleStyleWithFoo {
    Bar(Foo, core::marker::PhantomData<&'static usize>),
}

pub enum StructStyleWithRef {
    Bar {
        bar: &'static usize,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

pub enum TupleStyleWithRef {
    Bar(&'static usize, core::marker::PhantomData<&'static usize>),
}

pub enum StructStyleWithTupleStyle {
    Bar {
        bar: (usize, i64),
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

pub enum TupleStyleWithTuple {
    Bar((usize, i64), core::marker::PhantomData<&'static usize>),
}

pub enum StructStyleWithGeneric {
    Bar {
        bar: StructStyleWithZeroSizedData<usize>,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

pub enum TupleStyleWithGeneric {
    Bar(
        StructStyleWithZeroSizedData<usize>,
        core::marker::PhantomData<&'static usize>,
    ),
}

pub enum StructStyleWithSpecificZeroSizedData {
    Bar {
        bar: usize,
        _marker: core::marker::PhantomData<&'static usize>,
    },
}

pub enum TupleStyleWithSpecificZeroSizedData {
    Bar(usize, core::marker::PhantomData<&'static usize>),
}

// A trailing comma corner case - checks if attributes are parsed correctly.

#[repr(transparent)]
pub enum TupleStyleTrailingComma {
    Foo(usize),
}

#[repr(transparent)]
pub enum StructStyleTrailingComma {
    Foo { bar: usize },
}
