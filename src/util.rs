use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::{bail, Context};
use rustdoc_types::Crate;
use serde::Deserialize;

pub(crate) const SCOPE: &str = "semver-checks";

#[derive(Deserialize)]
struct RustdocFormatVersion {
    format_version: u32,
}

pub(crate) fn load_rustdoc_from_file(path: &Path) -> anyhow::Result<Crate> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut s = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open rustdoc JSON output file {:?}", path))?
        .read_to_string(&mut s)
        .with_context(|| format!("Failed to read rustdoc JSON output file {:?}", path))?;

    match serde_json::from_str(&s) {
        Ok(value) => Ok(value),
        Err(_) => {
            // Attempt to figure out the more precise reason the deserialization failed.
            // Several possible options and their resolutions:
            // (1) The file isn't actually a rustdoc JSON file. The user should supply a valid file.
            // (2) The rustdoc JSON file has a version number that is too old, and isn't supported.
            //     The user should upgrade to a newer nightly Rust version and regenerate the file.
            // (3) The rustdoc JSON file has a version number that is too new, and isn't supported.
            //     The user should attempt to upgrade to a newer cargo-semver-checks version
            //     if one is already available, or open a GitHub issue otherwise.

            // The error on this line is case (1).
            let version = serde_json::from_str::<RustdocFormatVersion>(&s).with_context(|| {
                format!(
                    "Failed to parse the rustdoc JSON file, and could not determine its \
                    format version. Are you sure this is a valid rustdoc JSON file? \
                    File path: {:?}",
                    path
                )
            })?;

            match version.format_version.cmp(&rustdoc_types::FORMAT_VERSION) {
                std::cmp::Ordering::Less => {
                    // The error here is case (2).
                    bail!(
                        "Failed to parse rustdoc JSON file: {path:?}\n\n\
                        This version of cargo-semver-checks requires rustdoc JSON version {0}, \
                        but the JSON file uses the older version {1}.\n\n\
                        Please upgrade to a Rust nightly version where rustdoc outputs \
                        JSON format version {0}.\n\n\
                        It's usually easiest to upgrade to the latest versions of \
                        both cargo-semver-checks and Rust nightly, \
                        which should always be compatible with each other.",
                        rustdoc_types::FORMAT_VERSION,
                        version.format_version,
                    )
                }
                std::cmp::Ordering::Greater => {
                    // The error here is case (3).
                    bail!(
                        "Failed to parse rustdoc JSON file: {path:?}\n\n\
                        This version of cargo-semver-checks requires rustdoc JSON version {0}, \
                        but the JSON file uses the newer version {1}.\n\n\
                        Please upgrade to a newer version of cargo-semver-checks \
                        which supports rustdoc JSON format version {1}.\n\n\
                        It's usually easiest to upgrade to the latest versions of \
                        both cargo-semver-checks and Rust nightly, \
                        which should always be compatible with each other.",
                        rustdoc_types::FORMAT_VERSION,
                        version.format_version,
                    )
                }
                std::cmp::Ordering::Equal => {
                    unreachable!(
                        "The rustdoc JSON versions matched but rustdoc_types serde \
                        deserialization failed for file: {:?}",
                        path,
                    )
                }
            }
        }
    }
}

pub(crate) fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
}
