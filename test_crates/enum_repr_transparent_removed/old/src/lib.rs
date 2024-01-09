#[repr(transparent)]
pub enum Foo {
    Bar(usize),
}

#[repr(transparent)]
pub enum Bar {
    Baz { quux: usize },
}

#[repr(transparent)]
pub enum StructWithZeroSizedData<T> {
    Bar {
        bar: usize,
        _marker: std::marker::PhantomData<T>,
    },
}

#[repr(transparent)]
pub enum TupleWithZeroSizedData<T> {
    Bar(usize, core::marker::PhantomData<T>),
}

#[repr(transparent)]
pub enum StructWithSpecificZeroSizedData {
    Bar {
        bar: usize,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleWithSpecificZeroSizedData {
    Bar(usize, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructWithFoo {
    Bar {
        bar: Foo,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleWithFoo {
    Bar(Foo, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructWithRef {
    Bar {
        bar: &'static usize,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleWithRef {
    Bar(&'static usize, std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructWithTuple {
    Bar {
        bar: (usize, i64),
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleWithTuple {
    Bar((usize, i64), std::marker::PhantomData<&'static usize>),
}

#[repr(transparent)]
pub enum StructWithGeneric {
    Bar {
        bar: StructWithZeroSizedData<usize>,
        _marker: std::marker::PhantomData<&'static usize>,
    },
}

#[repr(transparent)]
pub enum TupleWithGeneric {
    Bar(
        StructWithZeroSizedData<usize>,
        std::marker::PhantomData<&'static usize>,
    ),
}

// A trailing comma corner case - checks if attributes are parsed correctly.

#[repr(transparent, )]
pub enum TrailingCommaTupleStyle {
    Foo(usize),
}

#[repr(transparent, )]
pub enum TrailingCommaStructStyle {
    Foo { bar: usize },
}
