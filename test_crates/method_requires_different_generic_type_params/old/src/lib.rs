pub struct Owner;

impl Owner {
    pub fn previously_not_generic() {}

    pub fn add_generic_type<T>(data: T) {}

    pub fn remove_generic_type<T>(data: T) {}
}
