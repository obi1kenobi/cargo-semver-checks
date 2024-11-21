mod error;
mod generate;
mod progress;
mod request;

use trustfall_rustdoc::{VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter};

pub(crate) use error::{IntoTerminalResult, TerminalError};
pub(crate) use generate::GenerationSettings;
pub(crate) use progress::ProgressCallbacks;
pub(crate) use request::{CacheSettings, CrateDataRequest};

#[derive(Debug)]
pub(crate) struct DataStorage {
    current_crate: VersionedCrate,
    baseline_crate: VersionedCrate,
}

impl DataStorage {
    pub(crate) fn new(current_crate: VersionedCrate, baseline_crate: VersionedCrate) -> Self {
        Self {
            current_crate,
            baseline_crate,
        }
    }

    pub(crate) fn current_crate(&self) -> &VersionedCrate {
        &self.current_crate
    }

    pub(crate) fn baseline_crate(&self) -> &VersionedCrate {
        &self.baseline_crate
    }
}

impl DataStorage {
    pub(crate) fn create_indexes(&self) -> IndexStorage<'_> {
        IndexStorage {
            current_crate: VersionedIndexedCrate::new(&self.current_crate),
            baseline_crate: VersionedIndexedCrate::new(&self.baseline_crate),
        }
    }
}

#[derive(Debug)]
pub(crate) struct IndexStorage<'a> {
    current_crate: VersionedIndexedCrate<'a>,
    baseline_crate: VersionedIndexedCrate<'a>,
}

impl IndexStorage<'_> {
    pub(crate) fn create_adapter(&self) -> VersionedRustdocAdapter {
        VersionedRustdocAdapter::new(&self.current_crate, Some(&self.baseline_crate))
            .expect("failed to construct adapter, this is a bug and should never happen")
    }
}
