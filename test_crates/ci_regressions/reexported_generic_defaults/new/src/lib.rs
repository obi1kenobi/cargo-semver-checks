//! Regression fixture for a cargo-semver-checks crash caused by re-exports
//! that omit generic arguments because the underlying item supplies generic
//! defaults.
//!
//! The bug was found in `libp2p-gossipsub` 0.44.0 and
//! `libp2p-request-response` 0.24.0:
//! <https://github.com/libp2p/rust-libp2p/pull/3401#issuecomment-1409381365>
//!
//! This crate shallowly mimics that original public API shape so the bug stays
//! covered without depending on those historical libp2p versions.

mod swarm {
    pub trait Store {
        fn id(&self) -> usize;
    }

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct DefaultStore;

    impl Store for DefaultStore {
        fn id(&self) -> usize {
            0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Behaviour<TStore: Store = DefaultStore> {
        store: TStore,
    }

    impl Behaviour {
        pub fn new() -> Self {
            Self {
                store: DefaultStore,
            }
        }
    }

    impl Default for Behaviour {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<TStore: Store> Behaviour<TStore> {
        pub fn with_store(store: TStore) -> Self {
            Self { store }
        }

        pub fn store(&self) -> &TStore {
            &self.store
        }
    }

    pub trait Handler<TStore: Store = DefaultStore> {
        fn handle(&mut self, store: &TStore) -> usize;
    }
}

pub mod gossipsub {
    pub use crate::swarm::{Behaviour, DefaultStore, Handler, Store};
}

pub mod request_response {
    pub use crate::swarm::{DefaultStore, Handler, Store};
    pub use crate::swarm::Behaviour as BehaviourWithStore;

    pub type Behaviour = BehaviourWithStore;
}

pub use gossipsub::Behaviour as GossipsubBehaviour;
pub use request_response::Behaviour as RequestResponseBehaviour;
