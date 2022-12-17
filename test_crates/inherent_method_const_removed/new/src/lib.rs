pub struct Foo;

impl Foo {
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn method(&self, x: i64) -> i64 {
        x
    }

    pub fn new_const_associated_fn(x: i64) -> i64 {
        x
    }

    pub fn new_const_method_fn(&self, x: i64) -> i64 {
        x
    }
}
