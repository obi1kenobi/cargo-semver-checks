pub struct Owner;

impl Owner {
    pub fn previously_not_generic<T>() -> T {
        todo!()
    }

    pub fn add_generic_type<T, U>(data: T) -> U {
        todo!()
    }

    pub fn remove_generic_type(data: i64) {}

    // Not a major breaking change!
    // https://predr.ag/blog/some-rust-breaking-changes-do-not-require-major-version/
    pub fn becomes_impl_trait(data: impl Into<String>) {}
}
