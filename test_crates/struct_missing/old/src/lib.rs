pub struct WillBeRemovedStruct;

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;
}

pub use my_pub_mod::PubUseRemovedStruct;

// This struct is not removed, it only changes kind from tuple to plain.
// It should not be reported as missing.
pub struct ChangeStructKind(u64);
