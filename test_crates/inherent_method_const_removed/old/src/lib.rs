pub struct Foo;

impl Foo {
    pub const fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub const fn method(&self, x: i64) -> i64 {
        x
    }
}
