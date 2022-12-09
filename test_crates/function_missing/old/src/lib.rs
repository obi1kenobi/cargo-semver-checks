pub fn will_be_removed_fn() {}

pub mod my_pub_mod {
    pub fn pub_use_removed_fn() {}
}

pub use my_pub_mod::pub_use_removed_fn;
