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
        _marker: std::marker::PhantomData<T>,
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
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithSpecificZeroSizedData {
    Bar(usize, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithFoo {
    Bar {
        bar: Foo,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithFoo {
    Bar(Foo, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithRef {
    Bar {
        bar: &'static usize,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithRef {
    Bar(&'static usize, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithTupleStyle {
    Bar {
        bar: (usize, i64),
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithTuple {
    Bar((usize, i64), std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructStyleWithGeneric {
    Bar {
        bar: StructStyleWithZeroSizedData<usize>,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleStyleWithGeneric {
    Bar(
        StructStyleWithZeroSizedData<usize>,
        std::marker::PhantomData<&'static usize>,
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
