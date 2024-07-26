// Tests false positives. If the struct becomes #[doc(hidden)], it should trigger
// struct_now_doc_hidden, and changing hidden state of methods should have no further effect.

#[doc(hidden)]
pub struct Foo;

impl Foo {
    #[doc(hidden)]
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    #[doc(hidden)]
    pub fn method(&self, x: i64) -> i64 {
        x
    }
}
