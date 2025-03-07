#![no_std]

pub struct Owner;

impl Owner {
    pub fn previously_not_generic() {}

    pub fn add_generic_type<T>(data: T) {}

    pub fn remove_generic_type<T>(data: T) {}

    // Not a major breaking change!
    // https://predr.ag/blog/some-rust-breaking-changes-do-not-require-major-version/
    pub fn becomes_impl_trait(data: i64) {}
}
