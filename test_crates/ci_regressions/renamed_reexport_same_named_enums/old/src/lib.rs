//! Regression fixture for false positives when cargo-semver-checks matches
//! renamed re-exports of same-named public items from different modules:
//! <https://github.com/obi1kenobi/cargo-semver-checks/issues/202>
//!
//! The bug was found in `libp2p-dcutr` 1.62.0 and `libp2p-relay` 0.14.0.
//!
//! This crate shallowly mimics that original public API shape so the bug stays
//! covered without depending on those historical libp2p versions.

mod protocol_v1 {
    pub mod dcutr {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Error {
            UnsupportedProtocol,
            Timeout,
        }
    }

    pub mod relay {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Error {
            PermissionDenied,
            ReservationRefused,
        }
    }
}

pub mod dcutr {
    pub use crate::protocol_v1::dcutr::Error;

    pub fn timeout() -> Error {
        Error::Timeout
    }
}

pub mod relay {
    pub use crate::protocol_v1::relay::Error;

    pub fn denied() -> Error {
        Error::PermissionDenied
    }
}

pub use dcutr::Error as DcutrError;
pub use relay::Error as RelayError;

pub fn classify_dcutr(error: DcutrError) -> &'static str {
    match error {
        DcutrError::UnsupportedProtocol => "unsupported",
        DcutrError::Timeout => "timeout",
    }
}

pub fn classify_relay(error: RelayError) -> &'static str {
    match error {
        RelayError::PermissionDenied => "denied",
        RelayError::ReservationRefused => "refused",
    }
}
