// Tests false positives. If the struct is #[doc(hidden)], changing hidden state of methods should
// have no effect.

#[doc(hidden)]
pub struct Foo;

impl Foo {
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn method(&self, x: i64) -> i64 {
        x
    }
}
