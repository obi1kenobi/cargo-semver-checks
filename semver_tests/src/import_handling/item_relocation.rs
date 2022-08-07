/// Testing that items exposed via `pub use` aren't falsely flagged as removed
/// when they are relocated.
///
/// Items here are moved from `mod a` to `mod b`, which is fine because
/// they are only exposed via `pub use` and those paths don't change.
pub mod safe_relocation {
    mod a {
        #[cfg(not(feature = "struct_missing"))]
        pub struct RelocatedPubUseStruct;

        #[cfg(not(feature = "enum_missing"))]
        pub enum RelocatedPubUseEnum {}

        #[cfg(not(feature = "function_missing"))]
        pub fn relocated_pub_use_fn() {}
    }

    mod b {
        #[cfg(feature = "struct_missing")]
        pub struct RelocatedPubUseStruct;

        #[cfg(feature = "enum_missing")]
        pub enum RelocatedPubUseEnum {}

        #[cfg(feature = "function_missing")]
        pub fn relocated_pub_use_fn() {}
    }

    #[cfg(not(feature = "struct_missing"))]
    pub use a::RelocatedPubUseStruct;

    #[cfg(feature = "struct_missing")]
    pub use b::RelocatedPubUseStruct;

    #[cfg(not(feature = "enum_missing"))]
    pub use a::RelocatedPubUseEnum;

    #[cfg(feature = "enum_missing")]
    pub use b::RelocatedPubUseEnum;

    #[cfg(not(feature = "function_missing"))]
    pub use a::relocated_pub_use_fn;

    #[cfg(feature = "function_missing")]
    pub use b::relocated_pub_use_fn;
}
