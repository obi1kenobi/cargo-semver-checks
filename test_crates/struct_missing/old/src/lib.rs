pub struct WillBeRemovedStruct;

pub mod my_pub_mod {
    pub struct PubUseRemovedStruct;
}

pub use my_pub_mod::PubUseRemovedStruct;
