mod error;
mod generate;
mod progress;
mod request;

use trustfall_rustdoc::{VersionedIndex, VersionedRustdocAdapter, VersionedStorage};

pub(crate) use error::{IntoTerminalResult, TerminalError};
pub(crate) use generate::GenerationSettings;
pub(crate) use progress::ProgressCallbacks;
pub(crate) use request::{CacheSettings, CrateDataRequest};

#[derive(Debug)]
pub(crate) struct DataStorage {
    current: VersionedStorage,
    baseline: VersionedStorage,

    // TODO: This is temporary, until we stop supporting formats older than rustdoc v45.
    // v45+ formats carry target triple information embedded inside the rustdoc JSON file.
    pub(crate) target_triple: &'static str,
}

impl DataStorage {
    pub(crate) fn new(
        current: VersionedStorage,
        baseline: VersionedStorage,
        target_triple: &'static str,
    ) -> Self {
        Self {
            current,
            baseline,
            target_triple,
        }
    }

    pub(crate) fn current_crate(&self) -> &VersionedStorage {
        &self.current
    }

    pub(crate) fn baseline_crate(&self) -> &VersionedStorage {
        &self.baseline
    }
}

impl DataStorage {
    pub(crate) fn create_indexes(&self) -> IndexStorage<'_> {
        IndexStorage {
            current_crate: VersionedIndex::from_storage(&self.current, self.target_triple),
            baseline_crate: VersionedIndex::from_storage(&self.baseline, self.target_triple),
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
