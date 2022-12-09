pub enum WillBeRemovedEnum {}

pub mod my_pub_mod {
    pub enum PubUseRemovedEnum {}
}

pub use my_pub_mod::PubUseRemovedEnum;
