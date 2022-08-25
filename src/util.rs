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
        .with_context(|| format!("failed to open rustdoc JSON file {}", path.display()))?
        .read_to_string(&mut s)
        .with_context(|| format!("failed to read rustdoc JSON file {}", path.display()))?;

    match serde_json::from_str(&s) {
        Ok(value) => Ok(value),
        Err(e) => {
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
                format!("unrecognized rustdoc format for file {}", path.display(),)
            })?;

            match version.format_version.cmp(&rustdoc_types::FORMAT_VERSION) {
                std::cmp::Ordering::Less => {
                    // The error here is case (2).
                    bail!(
                        "\
rustdoc output format is too old (v{1}, need v{0}) for file {2}

note: using a newer Rust nightly version should help",
                        rustdoc_types::FORMAT_VERSION,
                        version.format_version,
                        path.display(),
                    )
                }
                std::cmp::Ordering::Greater => {
                    // The error here is case (3).
                    bail!(
                        "\
rustdoc output format is too new (v{1}, need v{0}) when parsing {2}

note: a newer version of cargo-semver-checks is likely available",
                        rustdoc_types::FORMAT_VERSION,
                        version.format_version,
                        path.display(),
                    )
                }
                std::cmp::Ordering::Equal => Err(e).with_context(|| {
                    format!(
                        "\
unexpected parse error for v{} rustdoc for file {}

note: this is a bug, please report it on the tool's GitHub together with \
the output of `cargo-semver-checks --bugreport`",
                        rustdoc_types::FORMAT_VERSION,
                        path.display(),
                    )
                }),
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
