pub enum PlainVariantWillBeRemoved {
    Foo,
}

pub enum TupleVariantWillBeRemoved {
    Foo(usize),
}

pub enum StructVariantWillBeRemoved {
    Foo { bar: usize },
}
