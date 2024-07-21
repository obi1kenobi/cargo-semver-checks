pub union Foo {
    f1: u32,
    f2: f32,
}

impl Foo {
    pub fn associated_fn(x: i64, y: i64) -> i64 {
        x + y
    }

    pub fn method(&self, x: i64) -> i64 {
        x
    }
}
