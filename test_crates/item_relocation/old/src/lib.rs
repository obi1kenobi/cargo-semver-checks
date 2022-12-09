/// Testing that items exposed via `pub use` aren't falsely flagged as removed
/// when they are relocated.
///
/// Items here are moved from `mod a` to `mod b`, which is fine because
/// they are only exposed via `pub use` and those paths don't change.
pub mod safe_relocation {
    mod a {
        pub struct RelocatedPubUseStruct;

        pub enum RelocatedPubUseEnum {}

        pub fn relocated_pub_use_fn() {}
    }

    mod b {}

    pub use a::RelocatedPubUseStruct;

    pub use a::RelocatedPubUseEnum;

    pub use a::relocated_pub_use_fn;
}
