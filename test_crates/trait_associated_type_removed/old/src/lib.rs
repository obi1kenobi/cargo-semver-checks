pub trait Foo {
    type Apple;
    type Bar;
    fn throw() -> Self::Apple;
}
