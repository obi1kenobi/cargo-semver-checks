use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::{bail, Context};
use serde::Deserialize;

use crate::{rustdoc_v18, rustdoc_v21};

pub(crate) const SCOPE: &str = "semver-checks";

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

pub(crate) enum CrateFormat {
    V18(rustdoc_types_14::Crate),
    V21(rustdoc_types_17::Crate),
}

impl CrateFormat {
    pub(crate) fn make_index(&self) -> Index<'_> {
        match self {
            CrateFormat::V18(c) => Index::V18(rustdoc_v18::indexed_crate::IndexedCrate::new(c)),
            CrateFormat::V21(c) => Index::V21(rustdoc_v21::indexed_crate::IndexedCrate::new(c)),
        }
    }
}

pub(crate) enum Index<'a> {
    V18(rustdoc_v18::indexed_crate::IndexedCrate<'a>),
    V21(rustdoc_v21::indexed_crate::IndexedCrate<'a>),
}

impl CrateFormat {
    pub(crate) fn crate_version(&self) -> Option<&str> {
        match self {
            CrateFormat::V18(c) => c.crate_version.as_deref(),
            CrateFormat::V21(c) => c.crate_version.as_deref(),
        }
    }
}

pub(crate) fn load_rustdoc_from_file(path: &Path) -> anyhow::Result<CrateFormat> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut s = String::new();
    File::open(path)
        .with_context(|| format!("failed to open rustdoc JSON file {}", path.display()))?
        .read_to_string(&mut s)
        .with_context(|| format!("failed to read rustdoc JSON file {}", path.display()))?;

    let version = serde_json::from_str::<RustdocFormatVersion>(&s).with_context(|| {
        format!("unrecognized rustdoc format for file {}", path.display(),)
    })?;
    match version.format_version {
        18 => Ok(CrateFormat::V18(serde_json::from_str(&s).expect("corrupted file"))),
        21 => Ok(CrateFormat::V21(serde_json::from_str(&s).expect("corrupted file"))),
        unsupported_version => {
            bail!(
                "unsupported rustdoc format version {unsupported_version}"
            )
        }
    }
}

pub(crate) fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
}
