mod error;
mod generate;
mod progress;
mod request;

use trustfall_rustdoc::{VersionedIndex, VersionedRustdocAdapter, VersionedStorage};

use crate::RustdocIndexingMode;

pub(crate) use error::{IntoTerminalResult, TerminalError};
pub(crate) use generate::GenerationSettings;
pub(crate) use generate::effective_witness_rustflags;
pub(crate) use progress::ProgressCallbacks;
pub(crate) use request::{CacheSettings, CrateDataRequest};

#[derive(Debug)]
pub(crate) struct DataStorage {
    current: VersionedStorage,
    baseline: VersionedStorage,
}

impl DataStorage {
    pub(crate) fn new(current: VersionedStorage, baseline: VersionedStorage) -> Self {
        Self { current, baseline }
    }

    pub(crate) fn current_crate(&self) -> &VersionedStorage {
        &self.current
    }

    pub(crate) fn baseline_crate(&self) -> &VersionedStorage {
        &self.baseline
    }
}

impl DataStorage {
    pub(crate) fn create_indexes(&self, mode: RustdocIndexingMode) -> IndexStorage<'_> {
        let (current_crate, baseline_crate) = match mode {
            RustdocIndexingMode::Ordinary => (
                VersionedIndex::from_storage(&self.current),
                VersionedIndex::from_storage(&self.baseline),
            ),
            RustdocIndexingMode::StabilityAware => (
                VersionedIndex::from_rust_std_component_storage(&self.current),
                VersionedIndex::from_rust_std_component_storage(&self.baseline),
            ),
        };

        IndexStorage {
            current_crate,
            baseline_crate,
        }
    }
}

#[derive(Debug)]
pub(crate) struct IndexStorage<'a> {
    current_crate: VersionedIndex<'a>,
    baseline_crate: VersionedIndex<'a>,
}

impl IndexStorage<'_> {
    pub(crate) fn create_adapter(&self) -> VersionedRustdocAdapter<'_> {
        VersionedRustdocAdapter::new(&self.current_crate, Some(&self.baseline_crate))
            .expect("failed to construct adapter, this is a bug and should never happen")
    }
}
